use crate::floor::MaqFloor;
use crate::tiles::tile_count;
use crate::maqs::*;

pub fn advance(floor: &mut MaqFloor) {
    let maqs = floor.maqs.clone();
    for k in maqs.keys() {
        match maqs.get(k) {
            //If key contains something, enact its will
            Some(key) => enact(floor, *key),
            //Otherwise, continue on
            None => continue,
        }
    }
}

pub fn enact(floor: &mut MaqFloor, pos: (usize, usize)) {
    let id = floor.states[(pos.0 * floor.wid)+pos.1];
    const tc: u8 = tile_count() as u8;
    //If id doesn't apply to any known MaqID, remove this pos from the list
    if(id < tc){
        floor.maqs.remove(&pos);
    } else {
        let mut maq = get_maq(id - tc);
        maq.work(floor, pos);
    }
}
