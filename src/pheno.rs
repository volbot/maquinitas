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
        let mut tile = get_tile(0);
        if x < tile_count() {
            let mut tile = get_tile(states[x]);
        } else if x < maq_count() {
            let mut tile = get_maq_tile(states[x]);
        } else {
            return
        }
        if tile.gravity {
            //floor.shift();
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
