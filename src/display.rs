use macroquad::prelude::*;

use crate::floor::MaqFloor;

pub fn draw(floor: MaqFloor) {
    //loop variables
    let tile_wid = screen_width() / floor.wid as f32;
    let tile_len = screen_height() / floor.len as f32;
    //loop params
    let mut x = 0;
    let mut y = 0;
    //iterate over floor
    while x < floor.wid {
        while y < floor.len {
            draw_rectangle(tile_wid*(x as f32), tile_len*(y as f32), tile_wid, tile_len, BLUE);
            y+=1;
        }
        y=0;
        x+=1;
    }
}
