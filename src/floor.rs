use macroquad::prelude::*;
use std::collections::HashMap;

use crate::display::Drawable;
use crate::inputs::Clickable;
use crate::maqs::*;

use crate::tiles::*;

//Maquinita Floor (the map)
// len + wid are length and width
// states is an array of what is in each area
#[derive(Debug, Clone)]
pub struct MaqFloor {
    pub len: usize,
    pub wid: usize,
    pub states: Box<[u8]>,
    pub maqs: HashMap<(usize, usize), Maq>,
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
                let tileID = states[(x * self.wid)+y];
                let tile = if tileID < tile_count() as u8 {
                    get_tile(tileID)
                } else {
                    get_maq_tile(tileID-tile_count() as u8)
                };
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
    //Place function
    // id: Tile/Maq ID to place
    // pos: where
    //This function has the functions for placing an object, and additionally,
    //it adds Maqs to the entity list.
    pub fn place(&mut self, id: u8, pos: (usize, usize)){
        let orig = self.states[pos.0*self.wid+pos.1];
        if orig != id{
            self.states[pos.0*self.wid+pos.1] = id;
            if id as usize > tile_count()+3 {
                return
            } else if id as usize >= tile_count() {
                self.maqs.insert(pos, Maq{
                    counter: 0,
                    enact: 30,
                    id: id,
                });
            }
        }
    }

    pub fn shift(&mut self, pos_in: (usize, usize), right: bool, dist: i16) {
        let mut i = 0;
        let mut pos_out = pos_in.clone();
        while i < dist {
            let mut refer = if right {&mut pos_out.0} else {&mut pos_out.1};
            *refer += 1;
            if right{
                if *refer >= self.wid {
                    return
                }
            } else {
                if *refer >= self.len {
                    return
                }
            }
            if *refer < 0 {
                return
            }
            i += 1;
            let temp = self.states[pos_in.0*self.wid+pos_in.1];
            let temp1 = self.states[pos_out.0*self.wid+pos_out.1];
            self.states[pos_in.0*self.wid+pos_in.1] = temp1;
            self.states[pos_out.0*self.wid+pos_out.1] = temp;
            match self.maqs.get(&pos_in) {
                Some(x) => {
                    self.maqs.insert(pos_out,*x);
                    self.maqs.remove(&pos_in);
                },
                None => return         
            }
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
        maqs: HashMap::new(),
    };

    //return the floor
    floor
}
