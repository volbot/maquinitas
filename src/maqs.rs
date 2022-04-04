use macroquad::prelude::*;

pub struct Maq {
    pub color: Color,
}

pub fn get_maq(id: u8) -> Maq {
    if id == 0 {
        let maq = Maq {
            color: GREEN
        };
        maq
    } else {
        let maq = Maq {
            color: BLUE
        };
        maq
    }
}
