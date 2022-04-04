//Maquinita Floor (the map)
// len + wid are length and width
// states is an array of what is in each area
#[derive(Debug, Clone)]
pub struct MaqFloor {
    len: usize,
    wid: usize,
    states: Box<[u8]>,
}

pub fn init_floor() -> MaqFloor {
    //create the floor size
    const L: usize = 10;
    const W: usize = 10;
    //allocate map data
    let states = [0;L*W];

    let floor = MaqFloor {
        len: L,
        wid: W,
        states: Box::new(states)
    };
    floor
}
