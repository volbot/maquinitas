use macroquad::prelude::*;

//Tile
// contains a name and color
pub struct Tile {
    pub name: &'static str,
    pub color: Color,
    pub passable: bool,
    pub gravity: bool,
}

//returns a Tile for a given TileID
pub fn get_tile(id: usize) -> Tile {
    let tile = Tile{
        name: match id {
            0 => "Nothing",
            1 => "Grass",
            2 => "Water",
            _ => "Block"
        }, 
        color: match id {
            0 => BLACK,
            1 => GREEN,
            2 => BLUE,
            _ => GRAY
        },
        passable: match id {
            0|1|2 => true,
            _ => false
        },
        gravity: match id {
            0|1 => false,
            2 => true,
            _ => false
        }
    };
    tile
}

const TC: usize = 4;
pub const fn tile_count() -> usize {TC}
