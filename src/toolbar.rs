use macroquad::prelude::*;

use crate::display::Drawable;
use crate::inputs::Clickable;

use crate::tiles::get_tile;

//Toolbar for selecting Maquinitas
// currently only contains an offset from the start
#[derive(Debug, Clone, Copy)]
pub struct Toolbar {
    pub offset: u16,
    pub selected: i16,
}
impl Drawable for Toolbar {
    fn draw(&self){
        let n = screen_height() / (screen_width()*0.05 as f32);
        let mut i = 0;
        let x = screen_width()*0.95;
        let mut y = screen_height() - screen_width()*0.05;
        while i < (n as u8) {
            draw_rectangle(x+screen_width()*0.005, y+screen_width()*0.005, screen_width()*0.04, screen_width()*0.04, get_tile(i).color);
            i+=1;
            y-=screen_width()*0.05;
        }
    }
}
impl Clickable for Toolbar {
    fn click(&mut self, mouse: MouseButton, pos: (f32, f32), dat: i16) {
        if mouse==MouseButton::Left {
            let n = (screen_height() - pos.1) / (screen_width()*0.05);
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
