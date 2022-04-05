use macroquad::prelude::*;

use crate::display::Drawable;
use crate::inputs::Clickable;

use crate::tiles::get_tile;

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
        let tile_wid = (screen_width()*0.95) / self.wid as f32;
        let tile_len = screen_height() / self.len as f32;
        //loop params
        let mut x = 0;
        let mut y = 0;
        let states = &self.states;
        //iterate over floor
        while x < self.wid {
            while y < self.len {
                //get machine filling [x][y]
                let tile = get_tile(states[(x * self.wid)+y]);
                //draw its color at [x][y]
                draw_rectangle(tile_wid*(x as f32), tile_len*(y as f32), tile_wid, tile_len, tile.color);
                
                y+=1;
            }
            y=0;
            x+=1;
        }
    }
}

impl Clickable for MaqFloor {
    fn click(&mut self, mouse: MouseButton, pos: (f32, f32), dat: i16){
        let (x, y) = (
            pos.0/(screen_width()*0.95)*self.wid as f32,
            pos.1/screen_height()*self.len as f32
            );
        if mouse == MouseButton::Left {
            if dat >= 0{
                self.place(dat as u8, (x as usize,y as usize));
            }
        }
    }
}

impl MaqFloor{
    pub fn place(&mut self, id: u8, pos: (usize, usize)){
        self.states[pos.0*self.wid+pos.1] = id;
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
