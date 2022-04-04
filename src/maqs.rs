use macroquad::prelude::*;

//Maquinita
// currently only contains a color to be drawn wherever it is
pub struct Maq {
    pub color: Color,
}

//returns a Maq for a given MaqID
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
