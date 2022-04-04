use macroquad::prelude::*;

use crate::floor::MaqFloor;

pub fn draw(floor: MaqFloor) {
    //loop variables
    let tileWid = screen_width() / floor.wid;
    let tileLen = screen_height() / floor.len;
    //loop params
    let mut x = 0;
    let mut y = 0;
    //iterate over floor
    while x < floor.wid {
        while y < floor.len {
            draw_rectangle(tileWid, tileLen, 0.0, 0.0, GREEN);
            y+=1;
        }
        x+=1;
    }
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
}
