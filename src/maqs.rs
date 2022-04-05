use macroquad::prelude::*;

//Maquinita
// currently only contains a color to be drawn wherever it is
pub struct Maq {
    pub color: Color,
}

//returns a Maq for a given MaqID
pub fn get_maq(id: u8) -> Maq {
    let maq = Maq{
        color: match id {
            0 => BLUE,
            1 => GREEN,
            _ => GRAY,
        }
    };
    maq
}
