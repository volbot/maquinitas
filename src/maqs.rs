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
    pub right: bool,
}
impl Worker for Mover {
    fn work(&mut self, floor: &mut MaqFloor, pos: (usize, usize)) {
        if self.maq.counter<self.maq.enact {
            self.maq.counter+=1;
            floor.maqs.insert(pos,self.maq);
        } else {
            self.maq.counter=0;
            let target1 = if self.right {
                (pos.0+1)*floor.wid+pos.1
            } else {
                pos.0*floor.wid+(pos.1+1)
            };
            let target2 = if self.right {
                (pos.0-1)*floor.wid+pos.1
            } else {
                pos.0*floor.wid+(pos.1-1)
            };
            let temp = floor.states[target2].clone();
            floor.states[target2] = floor.states[target1].clone();
            floor.states[target1] = temp;
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
            floor.shift(pos, self.right, 1);
        }
    }
}

pub fn get_maq_tile(id: u8) -> Tile {
    let tile = Tile {
        name: match id {
            0 => "Mover (H)",
            1 => "Mover (V)",
            2 => "Walker (H)",
            3 => "Walker (V)",
            _ => "Block",
        },
        color: match id {
            0 => ORANGE,
            1 => PURPLE,
            2 => RED,
            3 => YELLOW,
            _ => GRAY,
        },
    };
    tile
}
