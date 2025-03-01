use serde::Serialize;

use super::{
    manifest::{ActionId, PluginId},
    Plugin,
};

#[derive(Debug, Serialize)]
pub struct ActionCategory {
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
}

pub fn actions_from_plugins<'a, I>(plugins: I) -> Vec<ActionCategory>
where
    I: IntoIterator<Item = &'a Plugin>,
{
    let mut categories = Vec::new();

    for plugin in plugins {
        let manifest = &plugin.manifest;
        let manifest_category = &manifest.category;

        let category = match categories
            .iter_mut()
            .find(|category: &&mut ActionCategory| {
                category.label.eq(&manifest_category.label)
                    && category.icon.eq(&manifest_category.icon)
            }) {
            Some(value) => value,
            None => {
                let category = ActionCategory {
                    label: manifest_category.label.clone(),
                    icon: manifest_category.icon.clone(),
                    actions: Vec::new(),
                };

                let index = categories.len();
                categories.push(category);

                &mut categories[index]
            }
        };

        for (action_id, manifest_action) in &manifest.actions {
            let action = Action {
                action_id: action_id.clone(),
                plugin_id: manifest.plugin.id.clone(),

                label: manifest_action.label.clone(),
                icon: manifest_action.icon.clone(),
                description: manifest_action.description.clone(),
            };

            category.actions.push(action);
        }
    }

    categories
}
