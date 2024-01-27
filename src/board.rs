use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum TileState {
    White,
    Black,
}

#[derive(Resource, Clone)]
pub struct Board {
    pub tiles: Vec<Option<TileState>>,
    /// Number of rows.
    pub row_count: usize,
    /// Size of each tile.
    pub tile_size: f32,
}

impl Board {
    pub fn new(row_count: usize, tile_size: f32) -> Self {
        let mut tiles = Vec::new();
        tiles.resize_with(row_count * row_count, || None);
        Self {
            tiles,
            row_count,
            tile_size,
        }
    }

    pub fn reset(&mut self) {
        let tile_count: usize = self.tiles.len();
        // Rest all tiles to None
        for t in 0..tile_count {
            self.tiles[t] = None;
        }

        // Set first row to White
        for t in 0..self.row_count {
            self.tiles[t] = Some(TileState::White);
        }

        // Set last row to Black
        for t in 1..=self.row_count {
            self.tiles[tile_count - t] = Some(TileState::Black);
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<TileState> {
        self.tiles[self.get_tile_index(x, y)]
    }

    pub fn get_tile_index(&self, x: usize, y: usize) -> usize {
        x + y * self.row_count
    }
}
