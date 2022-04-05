use crate::floor::MaqFloor;
use crate::tiles::tile_count;

pub fn advance(floor: &mut MaqFloor) {
    let maqs = floor.maqs.clone();
    for k in maqs.keys() {
        match maqs.get(k) {
            Some(key) => enact(floor, *key),
            None => continue,
        }
    }
}

pub fn enact(floor: &mut MaqFloor, pos: (usize, usize)) {
    let id = floor.states[(pos.0 * floor.wid)+pos.1];
    const tc: u8 = tile_count() as u8;
    match id {
        tc => {

        },
        _ => {
            floor.maqs.remove(&pos);
        },
    }
}
