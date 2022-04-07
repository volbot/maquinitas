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
    pub id: u8,
}
pub struct Mover {
    pub maq: Maq,
    pub right: bool,
}
pub struct Walker {
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
            floor.shift(pos, self.dir==0 || self.dir == 2, 
                if self.dir==0 || self.dir==1 {
                    1
                } else {
                    -1
                });
        }
    }
}

pub fn get_maq_tile(id: u8) -> Tile {
    let tile = Tile {
        name: match id {
            0 => "Mover (H)",
            1 => "Mover (V)",
            2 => "Walker (R)",
            3 => "Walker (D)",
            4 => "Walker (L)",
            5 => "Walker (U)",
            _ => "Block",
        },
        color: match id {
            0 => ORANGE,
            1 => PURPLE,
            2|4 => RED,
            3|5 => YELLOW,
            _ => GRAY,
        },
        passable: match id {
            0|1 => false,
            2|3|4|5 => true,
            _ => false,
        }
    };
    tile
}
