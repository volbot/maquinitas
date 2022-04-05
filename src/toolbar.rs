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
        let mut i: u8 = (self.offset as u8 * (screen_height() / (screen_width()*0.05)) as u8);
        let n = ((screen_height() / (screen_width()*0.05 as f32)) as i16)+i as i16; 
        let x = screen_width()*0.95;
        let mut y = screen_height() - screen_width()*0.05;
        draw_rectangle(x+screen_width()*0.00225, y + screen_width()*0.02, screen_width()*0.02, screen_width()*0.02, BLACK); 
        draw_rectangle(x+screen_width()*0.0275, y + screen_width()*0.02, screen_width()*0.02, screen_width()*0.02, BLACK);
        y -= screen_width()*0.03;
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
            if pos.1>screen_height()*0.95 {
                if pos.0>screen_width()*0.975 {
                    if self.offset<1 {self.offset+=1;}
                } else {
                    if self.offset>0 {self.offset-=1;} 
                }
            } else {
                let n = screen_height() / (screen_width()*0.05);
                let i = (self.offset*n as u16) as f32+(screen_height() - screen_width()*0.03 - pos.1) / (screen_width()*0.05);
                self.selected = i as i16;
            }
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
