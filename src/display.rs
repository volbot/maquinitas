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
            draw_rectangle(tile_wid as f32, tile_len as f32, x as f32, y as f32, GREEN);
            y+=1;
        }
        x+=1;
    }
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
}