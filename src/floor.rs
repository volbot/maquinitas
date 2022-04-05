use macroquad::prelude::*;

use crate::display::Drawable;
use crate::maqs::get_maq;

//Maquinita Floor (the map)
// len + wid are length and width
// states is an array of what is in each area
#[derive(Debug, Clone)]
pub struct MaqFloor {
    pub len: usize,
    pub wid: usize,
    pub states: Box<[u8]>,
}

impl Drawable for MaqFloor {
    fn draw(&self){
        //loop variables
        let tile_wid = (screen_width()-30.0) / self.wid as f32;
        let tile_len = screen_height() / self.len as f32;
        //loop params
        let mut x = 0;
        let mut y = 0;
        let states = &self.states;
        //iterate over floor
        while x < self.wid {
            while y < self.len {
                //get machine filling [x][y]
                let maq = get_maq(states[(x * self.wid)+y]);
                //draw its color at [x][y]
                draw_rectangle(tile_wid*(x as f32), tile_len*(y as f32), tile_wid, tile_len, maq.color);
                
                y+=1;
            }
            y=0;
            x+=1;
        }
    }
}

pub fn init_floor() -> MaqFloor {
    //create the floor size
    const L: usize = 40;
    const W: usize = 40;

    //create state array
    let states = [0;L*W];

    //create floor, encasing the array in a Box
    let floor = MaqFloor {
        len: L,
        wid: W,
        states: Box::new(states),
    };

    //return the floor
    floor
}
