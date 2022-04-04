use macroquad::prelude::*;

use crate::floor::MaqFloor;
use crate::maqs::get_maq;

pub fn draw(floor: MaqFloor) {
    //loop variables
    let tile_wid = screen_width() / floor.wid as f32;
    let tile_len = screen_height() / floor.len as f32;
    //loop params
    let mut x = 0;
    let mut y = 0;
    let states = floor.states;
    //iterate over floor
    while x < floor.wid {
        while y < floor.len {
            //get machine filling [x][y]
            let maq = get_maq(states[(x * floor.wid)+y]);
            //draw its color at [x][y]
            draw_rectangle(tile_wid*(x as f32), tile_len*(y as f32), tile_wid, tile_len, maq.color);
            
            y+=1;
        }
        y=0;
        x+=1;
    }
}
