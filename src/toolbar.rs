use macroquad::prelude::*;

use crate::display::Drawable;
use crate::inputs::Clickable;

use crate::maqs::get_maq;

//Toolbar for selecting Maquinitas
// currently only contains an offset from the start
#[derive(Debug, Clone, Copy)]
pub struct Toolbar {
    pub offset: u16,
    pub selected: i16,
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
impl Clickable for Toolbar {
    fn click(&mut self, mouse: MouseButton, pos: (f32, f32), dat: i16) {
        if mouse==MouseButton::Left {
            let n = (screen_height() - pos.1) / 30.0;
            self.selected = n as i16;    
        }
    }
}

pub fn init_toolbar() -> Toolbar {
    let toolbar = Toolbar {
        offset: 0,
        selected: -1,
    };
    toolbar
}
