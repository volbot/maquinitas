use crate::floor::MaqFloor;
use crate::tiles::tile_count;
use crate::maqs::*;

pub fn advance(floor: &mut MaqFloor) {
    let maqs = floor.maqs.clone();
    for k in maqs.keys() {
        enact(floor,*k);    
    }
}

pub fn enact(floor: &mut MaqFloor, pos: (usize, usize)) {
    let id = floor.states[(pos.0 * floor.wid)+pos.1];
    const TC: u8 = tile_count() as u8;
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
                } else {
                    Box::new(Walker {
                        maq: *x,
                        dir: id-6,
                        })
                };
                worker.work(floor,pos);
            },
            None => return
        }   
    }
}
