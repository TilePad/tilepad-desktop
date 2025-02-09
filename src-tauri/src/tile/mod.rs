use std::collections::HashMap;

use uuid::Uuid;

use crate::plugin::manifest::ActionPath;

/// Tile is a populated instance of an action
/// with additional configuration
pub struct Tile {
    /// Unique ID for this tile
    pub id: Uuid,
    /// Path to the action this tile executes
    pub action: ActionPath,
    /// Row the tile is apart of
    pub row: u8,
    //
    pub column: u8,
}

/// Collection of tiles basically a page
pub struct Profile {
    pub id: Uuid,
    pub name: String,
    pub tiles: Vec<Tile>,
    pub slots: HashMap<(u8 /* Tile X */, u8 /* Tile Y */), usize /* Tile Index */>,
    pub rows: u8,
    pub columns: u8,
}
