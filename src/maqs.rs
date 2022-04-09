use macroquad::prelude::*;
use crate::floor::MaqFloor;
use crate::tiles::Tile;

pub trait Worker {
    fn work(&mut self, floor: &mut MaqFloor, pos:(usize, usize));
}

#[derive(Debug,Clone,Copy)]
pub struct Maq {
    pub counter: u8,
    pub enact: u8,
    pub id: usize,
}
pub struct Mover {
    pub maq: Maq,
    pub right: bool,
}
pub struct Walker {
    pub maq: Maq,
    pub dir: u8,
}
pub struct Rotator {
    pub maq: Maq,
    pub dir: u8,
}

impl Worker for Mover {
    fn work(&mut self, floor: &mut MaqFloor, pos: (usize, usize)) {
        if self.maq.counter<self.maq.enact {
            self.maq.counter+=1;
            floor.maqs.insert(pos,self.maq);
        } else {
            self.maq.counter=0;
            //get one position
            let target1 = if self.right {
                if pos.0==floor.wid {
                    return
                }
                (pos.0+1,pos.1)
            } else {
                if pos.1==floor.len {
                    return
                }
                (pos.0,pos.1+1)
            };
            //get the other position
            let target2 = if self.right {
                if pos.0==0 {
                    return
                }
                (pos.0-1,pos.1)
            } else {
                if pos.1==0 {
                    return
                }
                (pos.0,pos.1-1)
            };
            //swap them
            floor.swap(target1,target2);
            floor.maqs.insert(pos,self.maq);
        }
    }
}

impl Worker for Walker {
    fn work(&mut self, floor: &mut MaqFloor, pos: (usize, usize)) {
        if self.maq.counter<self.maq.enact {
            self.maq.counter+=1;
            floor.maqs.insert(pos,self.maq);
        } else {
            self.maq.counter = 0;
            floor.maqs.insert(pos,self.maq);
            //move in direction
            floor.shift(pos, self.dir==0 || self.dir == 2, 
                if self.dir==0 || self.dir==1 {
                    1
                } else {
                    -1
                });
        }
    }
}

impl Worker for Rotator {
    fn work(&mut self, floor: &mut MaqFloor, pos: (usize, usize)) {
        if self.maq.counter<self.maq.enact {
            self.maq.counter+=1;
            floor.maqs.insert(pos,self.maq);
        } else {
            self.maq.counter = 0;
            floor.maqs.insert(pos,self.maq);
            //find position to rotate and set it depending on direction
            let mut pos_new = (pos.0 as i8, pos.1 as i8);
            if self.dir==0||self.dir==2 {pos_new.0 += if self.dir==0 {1} else {-1};}
            if self.dir==1||self.dir==3 {pos_new.1 += if self.dir==1 {1} else {-1};}
            //if position doesn't exist, stop doing this code or it'll crash
            if pos_new.0 < 0 || pos_new.1 < 0 || pos_new.0 >= floor.wid as i8 || pos_new.1 >= floor.len as i8 {
                return
            }
            //get the Maq at the position to rotate, if it exists
            let maq_get = floor.maqs.get(&(pos_new.0 as usize,pos_new.1 as usize));
            match maq_get {
                //if there is one, increment the id of the maq to be rotated
                Some(x) => {
                    let new_id = match x.id{
                        2|3|4 => x.id+1,
                        5 => 2,
                        6|7|8 => x.id+1,
                        9 => 6,
                        //if non-rotatable, return
                        _ => return,
                    };
                    //get the counter and enact of the existing maq, so that the data is preserved
                    let counter_new = x.counter;
                    let enact_new = x.enact;
                    //place everthing as it should be
                    floor.place(new_id, (pos_new.0 as usize,pos_new.1 as usize));
                    floor.maqs.insert((pos_new.0 as usize, pos_new.1 as usize),Maq{
                        counter: counter_new,
                        enact: enact_new,
                        id: new_id
                    });
                }
                //if no maq, then nothing to rotate
                None => return,
            }
        }
    }
}

//Returns a Tile for a given Maq.
// id is the MaqID (subtract tilecount!)
pub fn get_maq_tile(id: usize) -> Tile {
    let tile = Tile {
        name: match id {
            0 => "Mover (H)",
            1 => "Mover (V)",
            2 => "Walker (R)",
            3 => "Walker (D)",
            4 => "Walker (L)",
            5 => "Walker (U)",
            6 => "Rotator (R)",
            7 => "Rotator (D)",
            8 => "Rotator (L)",
            9 => "Rotator (U)",
            _ => "Block",
        },
        color: match id {
            0 => ORANGE,
            1 => PURPLE,
            2|4 => RED,
            3|5 => YELLOW,
            6|8 => WHITE,
            7|9 => BROWN,
            _ => GRAY,
        },
        passable: match id {
            0|1|6|7|8|9 => false,
            2|3|4|5 => false,
            _ => false,
        },
        gravity: false,
    };
    tile
}

const MC: usize = 10;
pub const fn maq_count() -> usize {MC}
