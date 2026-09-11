mod http;
mod page;

use az_dioxus_admin_shell::{ApplicationPage, ApplicationPlugin, ApplicationScene};
use dill::CatalogBuilder;

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
            menu_path: Vec::new(),
            required_permission: None,
            render: page::SettingsPage,
        }]
    }
}
pub fn register(builder: &mut CatalogBuilder) {
    builder
        .add_value(SettingsPlugin)
        .bind::<dyn ApplicationPlugin, SettingsPlugin>();
}
