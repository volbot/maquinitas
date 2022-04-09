use macroquad::prelude::*;

use crate::floor::MaqFloor;
use crate::tiles::*;
use crate::maqs::*;

pub fn advance(floor: &mut MaqFloor) {
    let maqs = floor.maqs.clone();
    for k in maqs.keys() {
        enact(floor,*k);    
    }
}

pub fn pass_time(floor: &mut MaqFloor) {
    let states = floor.states.clone();
    let mut x = 0;
    while x < (floor.wid*floor.len) {
        let id = states[x];
        let mut tile = get_tile(0);
        if id < tile_count()  {
            tile = get_tile(id);
        } else if id < maq_count() {
            tile = get_maq_tile(id);
        }
        let pos:(usize,usize) = (x / floor.wid, x % floor.len);
        let mut cont = false;
        if tile.gravity >= 1 && !cont {
            cont = floor.shift(pos, false, 1);
        }
        if tile.gravity >= 2 && !cont {
            let mut dir = 1;
            if rand::gen_range(0,2)==0 {
                dir = -1;
            }
            cont = floor.shift(pos, true, dir);
        }
        x+=1;
    }
}

pub fn enact(floor: &mut MaqFloor, pos: (usize, usize)) {
    let id = floor.states[(pos.0 * floor.wid)+pos.1];
    const TC: usize = tile_count();
    //If id doesn't apply to any known MaqID, remove this pos from the list
    if id < TC {
        floor.maqs.remove(&pos);
    } else {
        match floor.maqs.get(&pos) {
            Some(x) => {
                let mut worker: Box<dyn Worker> = if id == TC || id == TC+1 {
                    Box::new(Mover {
                        maq: *x,
                        right: if id == TC+1 {
                            false
                        } else {
                            true
                        }    
                    })
                } else if id >= TC+2 && id < TC+6 {
                    Box::new(Walker {
                        maq: *x,
                        dir: (id-6) as u8
                    })
                } else if id >= TC+6 && id < TC+10 {
                    Box::new(Rotator {
                        maq: *x,
                        dir: (id-10) as u8
                    })
                } else {
                    Box::new(Mover {
                        maq: *x,
                        right: true,
                    })
                };
                worker.work(floor,pos);
            },
            None => return
        }   
    }
}
