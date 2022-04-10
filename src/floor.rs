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
    pub states: Box<[usize]>,
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
                let tile_id = states[(x * self.wid)+y];
                let tile = if tile_id < tile_count() {
                    get_tile(tile_id)
                } else {
                    get_maq_tile(tile_id-tile_count())
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
    fn click(&mut self, mouse: MouseButton, pos: (f32, f32), dat: isize){
        let (x, y) = (
            pos.0/(screen_width()*0.95)*self.wid as f32,
            pos.1/screen_height()*self.len as f32
            );
        if mouse == MouseButton::Left {
            if dat >= 0{
                self.place(dat as usize, (x as usize,y as usize));
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
    pub fn place(&mut self, id: usize, pos: (usize, usize)){
        let orig = self.states[pos.0*self.wid+pos.1];
        if orig != id{
            self.states[pos.0*self.wid+pos.1] = id;
            if id >= tile_count()+10 {
                return
            } else if id as usize >= tile_count() {
                self.maqs.insert(pos, Maq{
                    counter: 0,
                    enact: match id as usize-tile_count() {
                        0|1 => 15,
                        2|3|4|5 => 10,
                        6|7|8|9 => 10,
                        _ => 30,
                    },
                    id: id,
                });
            }
        }
    }

    //Shift function
    // pos_in: starting position
    // right: true go right, false go down
    // dist: distance to go. negatives for backwards
    pub fn shift(&mut self, pos_in: (usize, usize), right: bool, dist: i16) -> bool {
        let mut i = 0;
        let mut pos_out = pos_in.clone();
        while i != dist {
            let refer = if right {&mut pos_out.0} else {&mut pos_out.1};
            *refer = if dist > 0 {*refer+1 as usize} else {
                if *refer<=0 {
                    return false
                } else {
                    *refer-1 as usize
                }
            };
            if right{
                if *refer >= self.wid {
                    return false
                }
            } else {
                if *refer >= self.len {
                    return false
                }
            }
            i += if dist > 0 {1} else {-1};
            let temp = self.states[pos_out.0*self.wid+pos_out.1];
            let this = self.states[pos_in.0*self.wid+pos_in.1];
            let tile = if temp<tile_count() {get_tile(temp)} else {get_maq_tile(temp-tile_count())};
            if !tile.passable || temp==this {
                return false
            } else {
                if tile.movable && temp!=0 {
                    self.shift(pos_out,right,dist);
                }
                self.swap(pos_in,pos_out);    
            }
        }
        return true;
    }
    
    //Swap function
    // Swaps the tiles at two positions, swapping their Maqs as well.
    pub fn swap(&mut self, pos_1: (usize, usize), pos_2: (usize, usize)) {
        if pos_1.0>=self.wid||pos_2.0>=self.wid||pos_1.1>=self.len||pos_2.1>=self.len {
            return
        }
        let temp1 = self.states[pos_1.0*self.wid+pos_1.1];
        let temp2 = self.states[pos_2.0*self.wid+pos_2.1];

        if !(get_tile(temp1).movable&&get_tile(temp1).movable) {
            return
        }

        self.states[pos_1.0*self.wid+pos_1.1] = temp2;
        self.states[pos_2.0*self.wid+pos_2.1] = temp1;
        
        let maq_temp1 = self.maqs.get(&pos_1).cloned();
        let maq_temp2 = self.maqs.get(&pos_2).cloned();

        match maq_temp1 {
            Some(maq1) => {
                self.maqs.insert(pos_2,maq1);
            },
            None => {
                self.maqs.remove(&pos_2);
            }
        }
        match maq_temp2 {
            Some(maq2) => {
                self.maqs.insert(pos_1,maq2);
            },
            None => {
                self.maqs.remove(&pos_1);
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
