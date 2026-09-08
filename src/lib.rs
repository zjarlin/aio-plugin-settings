use az_dioxus_admin_shell::{ApplicationPage, ApplicationPlugin, ApplicationScene};
use az_ui_components::button::{Button, ButtonVariant};
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
    let mut saved = use_signal(|| false);
    rsx! {
        section {
            h2 { "设置中心" }
            p { "管理当前租户、会话和插件运行权限。" }
            div { class: "grid gap-3 md:grid-cols-2",
                article { class: "border p-4",
                    h3 { "当前租户" }
                    p { "默认租户" }
                    Button {
                        r#type: "button",
                        variant: ButtonVariant::Outline,
                        onclick: move |_| saved.set(true),
                        "保存设置"
                    }
                }
                article { class: "border p-4",
                    h3 { "运行时" }
                    p { "Wasm Component 与独立进程由宿主隔离管理。" }
                }
            }
            if saved() { p { role: "status", "设置已保存" } }
        }
    }
}
