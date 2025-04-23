use serde::Serialize;
use tilepad_manifest::plugin::Manifest;

use super::manifest::{ActionId, PluginId};

#[derive(Debug, Serialize)]
pub struct ActionCategory {
    pub plugin_id: PluginId,

    pub label: String,
    pub icon: Option<String>,
    pub actions: Vec<Action>,
}

#[derive(Debug, Serialize)]
pub struct Action {
    pub plugin_id: PluginId,
    pub action_id: ActionId,

    pub label: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub inspector: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ActionWithCategory {
    #[serde(flatten)]
    pub action: Action,
    pub category_label: String,
}

pub fn actions_from_manifests<'a, I>(manifests: I) -> Vec<ActionCategory>
where
    I: IntoIterator<Item = &'a Manifest>,
{
    let mut categories = Vec::new();

    for manifest in manifests {
        let manifest_category = &manifest.category;
        let mut category = ActionCategory {
            plugin_id: manifest.plugin.id.clone(),
            label: manifest_category.label.clone(),
            icon: manifest_category.icon.clone(),
            actions: Vec::new(),
        };

        for (action_id, manifest_action) in manifest.actions.0.iter() {
            let action = Action {
                action_id: action_id.clone(),
                plugin_id: manifest.plugin.id.clone(),

                label: manifest_action.label.clone(),
                icon: manifest_action.icon.clone(),
                description: manifest_action.description.clone(),
                inspector: manifest_action.inspector.clone(),
            };

            category.actions.push(action);
        }

        categories.push(category);
    }

    categories.sort_by(|a, b| a.plugin_id.cmp(&b.plugin_id));
    categories
}
