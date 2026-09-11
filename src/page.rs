use super::http;
use az_ui_components::{
    admin::{
        AsyncResult, CollectionTable, DeleteRecordsDialog, EditorDialog, PageHeader, PageSurface,
        RequestState, SortValue, StatusMessage,
    },
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonSize, ButtonVariant},
    data_table::{DataTableCellContext, DataTableColumn},
    input::Input,
};
use dioxus::prelude::*;
use dioxus_icons::lucide::{Plus, RefreshCw, Trash2};

#[allow(non_snake_case)]
pub(super) fn SettingsPage() -> Element {
    let mut revision = use_signal(|| 0_u64);
    let resource = use_resource(move || {
        let _ = revision();
        http::load()
    });
    let mut adding = use_signal(|| false);
    let mut removing = use_signal(|| None::<String>);
    let mut feedback = use_signal(|| None::<String>);
    let (session, registries) = match resource.read().as_ref().cloned() {
        Some(Ok(value)) => value,
        Some(Err(error)) => {
            return rsx! { PageSurface { RequestState { error, on_retry: move |_| revision += 1 } } };
        }
        None => return rsx! { PageSurface { RequestState {} } },
    };
    rsx! {
        PageSurface {
            PageHeader { title: "设置中心", detail: session.tenant_label.clone(),
                Button { size: ButtonSize::Icon, variant: ButtonVariant::Outline, title: "刷新设置", aria_label: "刷新设置", onclick: move |_| revision += 1, RefreshCw {} }
            }
            if let Some(message) = feedback() { StatusMessage { message } }
            section { class: "admin-section", h2 { "当前会话" }
                dl { class: "admin-details", dt { "账户" } dd { "{session.display_name} (@{session.account})" } dt { "租户" } dd { "{session.tenant_label}" } dt { "租户 ID" } dd { code { class: "admin-code", "{session.tenant_id}" } } }
            }
            if session.permissions.iter().any(|p| p == "plugin:manage") {
                section { class: "admin-section", h2 { "自定义市场源" }
                    CollectionTable { label: "市场源", rows: registries, columns: vec![DataTableColumn::leaf("source", "HTTPS Git / 索引地址").width(640), DataTableColumn::leaf("actions", "操作").width(80)],
                        row_key: |source: String| source, search_text: |source: String| source, sort_value: |(source, _): (String, String)| SortValue::Text(source), sortable: vec!["source".into()],
                        tools: rsx! { Button { onclick: move |_| adding.set(true), Plus {} "添加市场源" } }, empty_text: "暂无自定义市场源",
                        render_cell: move |context: DataTableCellContext<String>| { let source = context.row; if context.column.key == "source" { rsx! { code { class: "admin-code", "{source}" } } } else { rsx! {
                            Button { size: ButtonSize::IconSm, variant: ButtonVariant::Ghost, title: "移除市场源", aria_label: "移除 {source}", onclick: move |_| removing.set(Some(source.clone())), Trash2 {} }
                        } } },
                    }
                }
            }
            section { class: "admin-section", h2 { "当前权限" }
                div { class: "admin-badges", for permission in session.permissions { Badge { variant: BadgeVariant::Outline, "{permission}" } } }
            }
        }
        if adding() { RegistryEditor { on_close: move |_| adding.set(false), on_saved: move |_| { adding.set(false); feedback.set(Some("市场源已添加".into())); revision += 1; } } }
        if let Some(source) = removing() { DeleteRecordsDialog { title: "移除市场源", warning: "停止从该来源发现插件，不会卸载已安装的插件。", items: vec![source], item_label: |source: String| source,
            delete: |source: String| -> AsyncResult<()> { Box::pin(async move { http::save(source, true).await }) },
            on_close: move |_| removing.set(None), on_deleted: move |_| { feedback.set(Some("市场源已移除".into())); revision += 1; },
        } }
    }
}

#[component]
fn RegistryEditor(on_close: Callback<()>, on_saved: Callback<()>) -> Element {
    let mut source = use_signal(String::new);
    rsx! { EditorDialog { title: "添加市场源", description: "HTTPS Git 仓库或 index.json 地址。", on_close, on_saved,
        save: move |_| -> AsyncResult<()> { let source = source(); Box::pin(async move { http::save(source, false).await }) },
        label { class: "admin-field", span { "市场源地址" } Input { aria_label: "市场源地址", r#type: "url", required: true, value: source(), oninput: move |event: FormEvent| source.set(event.value()) } }
    } }
}
