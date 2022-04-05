use macroquad::prelude::*;

//Tile
// contains a name and color
pub struct Tile {
    pub name: &'static str,
    pub color: Color,
}

//returns a Tile for a given TileID
pub fn get_tile(id: u8) -> Tile {
    let tile = Tile{
        name: match id {
            0 => "Nothing",
            1 => "Grass",
            2 => "Water",
            _ => "Block",
        }, 
        color: match id {
            0 => BLACK,
            1 => GREEN,
            2 => BLUE,
            _ => GRAY,
        }
    };
    tile
}

const tc: usize = 3;
pub const fn tile_count() -> usize {
    tc
}
