use macroquad::prelude::*;

use crate::display::Drawable;
use crate::maqs::get_maq;

//Toolbar for selecting Maquinitas
// currently only contains an offset from the start
#[derive(Debug, Clone, Copy)]
pub struct Toolbar {
    pub offset: u16,
}
impl Drawable for Toolbar {
    fn draw(&self){
        let n = screen_height() / (30.0 as f32);
        let mut i = 0;
        let x = screen_width() - 30.0;
        let mut y = screen_height() - 30.0;
        while i < (n as u8) {
            draw_rectangle(x+2.0, y+2.0, 26.0, 26.0, get_maq(i).color);
            i+=1;
            y-=30.0;
        }
    }
}

pub fn init_toolbar() -> Toolbar {
    let toolbar = Toolbar {
        offset: 0,
    };
    toolbar
}
