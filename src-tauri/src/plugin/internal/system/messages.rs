use crate::{
    database::{entity::tile::TileModel, DbPool},
    events::InspectorContext,
    plugin::Plugins,
};

pub async fn handle(
    plugins: &Plugins,
    db: &DbPool,
    _tile: &TileModel,

    context: InspectorContext,
    message: serde_json::Value,
) -> anyhow::Result<()> {
    todo!();
}
