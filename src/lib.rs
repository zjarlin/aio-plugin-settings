use az_dioxus_admin_shell::{ApplicationPage, ApplicationPlugin, ApplicationScene};
use az_ui_components::{
    button::{Button, ButtonVariant},
    input::Input,
};
use dill::CatalogBuilder;
use dioxus::prelude::*;

#[derive(Debug)]
pub struct SettingsPlugin;

impl ApplicationPlugin for SettingsPlugin {
    fn pages(&self) -> Vec<ApplicationPage> {
        vec![ApplicationPage {
            id: "settings",
            label: "设置中心",
            icon: Some("settings"),
            scene: ApplicationScene {
                id: "system",
                label: "系统",
            },
            render: SettingsPage,
        }]
    }
}

pub fn register(builder: &mut CatalogBuilder) {
    builder
        .add_value(SettingsPlugin)
        .bind::<dyn ApplicationPlugin, SettingsPlugin>();
}

#[allow(non_snake_case)]
fn SettingsPage() -> Element {
    let mut registry = use_signal(String::new);
    let status = use_signal(|| None::<String>);
    let session = use_resource(aio_plugin_identity_client::load_session);
    let session = session.read().as_ref().cloned();
    rsx! {
        section {
            h2 { "设置中心" }
            p { "管理当前租户、会话和插件运行权限。" }
            div { class: "grid gap-3 md:grid-cols-2",
                article { class: "border p-4",
                    h3 { "当前租户" }
                    match session.as_ref() {
                        Some(Ok(Some(session))) => rsx! {
                            p { "{session.tenant_label}" }
                            p { class: "text-sm text-muted-foreground", "{session.tenant_id}" }
                        },
                        Some(Ok(None)) => rsx! { p { role: "alert", "会话已失效" } },
                        Some(Err(error)) => rsx! { p { role: "alert", "读取失败：{error}" } },
                        None => rsx! { p { "正在读取" } },
                    }
                }
                article { class: "border p-4",
                    h3 { "Git 市场源" }
                    Input {
                        value: registry(),
                        placeholder: "https://github.com/example/aio-marketplace.git",
                        aria_label: "Git 市场源",
                        oninput: move |event: FormEvent| registry.set(event.value()),
                    }
                    Button {
                        r#type: "button",
                        variant: ButtonVariant::Outline,
                        onclick: move |_| {
                            let source = registry();
                            spawn(async move { add_registry(source, status).await });
                        },
                        "添加市场源"
                    }
                }
                article { class: "border p-4",
                    h3 { "运行时" }
                    p { "Wasm Component 与独立进程由宿主隔离管理。" }
                }
                article { class: "border p-4",
                    h3 { "会话安全" }
                    p { "会话使用 HttpOnly、SameSite Cookie；修改密码后其他会话失效。" }
                }
            }
            if let Some(message) = status() {
                p { role: "status", "{message}" }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
async fn add_registry(source: String, mut status: Signal<Option<String>>) {
    status.set(Some("正在校验市场源".to_owned()));
    let body = serde_json::json!({ "source": source }).to_string();
    let result = gloo_net::http::Request::post("/api/runtime/registries")
        .header("content-type", "application/json")
        .body(body)
        .map_err(|error| error.to_string());
    match result {
        Ok(request) => match request.send().await {
            Ok(response) if response.ok() => status.set(Some("市场源已添加".to_owned())),
            Ok(response) => status.set(Some(response.text().await.unwrap_or_default())),
            Err(error) => status.set(Some(error.to_string())),
        },
        Err(error) => status.set(Some(error)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn add_registry(_source: String, mut status: Signal<Option<String>>) {
    status.set(Some("桌面端市场源保存尚未连接服务端".to_owned()));
}
