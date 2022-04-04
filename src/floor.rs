//Maquinita Floor (the map)
// len + wid are length and width
// states is an array of what is in each area
#[derive(Debug, Clone)]
pub struct MaqFloor {
    pub len: usize,
    pub wid: usize,
    pub states: Box<[u8]>,
}

pub fn init_floor() -> MaqFloor {
    //create the floor size
    const L: usize = 10;
    const W: usize = 10;

    let states = [0;L*W];

    //create floor
    let floor = MaqFloor {
        len: L,
        wid: W,
        states: Box::new(states),
    };

    //return the floor
    floor
}
