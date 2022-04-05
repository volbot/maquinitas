use macroquad::prelude::*;
use crate::floor::MaqFloor;
use crate::tiles::Tile;

pub trait Maq {
    fn work(&mut self, floor: &mut MaqFloor, pos: (usize, usize));
}
pub struct Mover {
    counter: u8,
    right: bool,
}
impl Maq for Mover {
    fn work(&mut self, floor: &mut MaqFloor, pos: (usize, usize)) {
        if self.counter<10 {
            println!("yea");
            self.counter = self.counter+1;
        } else {
            println!("pls");
            self.counter=0;
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
        }
    }
}

pub fn get_maq_tile(id: u8) -> Tile {
    let tile = Tile {
        name: "Maq",
        color: RED,
    };
    tile
}

pub fn get_maq(id: u8) -> impl Maq {
    let maq = Mover {
                counter: 9,
                right: true,   
            };
    maq
}
