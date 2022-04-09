use macroquad::prelude::*;

use crate::display::Drawable;
use crate::inputs::Clickable;

use crate::tiles::*;
use crate::maqs::*;

//Toolbar for selecting Maquinitas
// currently only contains an offset from the start
#[derive(Debug, Clone, Copy)]
pub struct Toolbar {
    pub offset: usize,
    pub selected: isize,
}
impl Drawable for Toolbar {
    fn draw(&self){
        let mut i = self.offset * (screen_height() / (screen_width()*0.05)) as usize;
        let n = ((screen_height() / (screen_width()*0.05))) as usize+i; 
        let x = screen_width()*0.95;
        let mut y = screen_height() - screen_width()*0.05;
        draw_rectangle(x+screen_width()*0.00225, y + screen_width()*0.02, screen_width()*0.02, screen_width()*0.02, BLACK); 
        draw_rectangle(x+screen_width()*0.0275, y + screen_width()*0.02, screen_width()*0.02, screen_width()*0.02, BLACK);
        y -= screen_width()*0.03;
        while i < (n) {
            let mut tile = get_tile(i as usize);
            if i >= tile_count() {
                tile = get_maq_tile(i-tile_count());
            }
            draw_rectangle(x+screen_width()*0.005, y+screen_width()*0.005, screen_width()*0.04, screen_width()*0.04, tile.color);
            i+=1;
            y-=screen_width()*0.05;
        }
    }
}
impl Clickable for Toolbar {
    fn click(&mut self, mouse: MouseButton, pos: (f32, f32), _dat: isize) {
        if mouse==MouseButton::Left {
            if pos.1>screen_height()*0.975 {
                if pos.0>screen_width()*0.975 {
                    if self.offset<1 {self.offset+=1;}
                } else {
                    if self.offset>0 {self.offset-=1;} 
                }
            } else {
                let n = (screen_height() / (screen_width()*0.05)) as usize;
                let i = (self.offset*n)+((screen_height() - screen_width()*0.03 - pos.1)/(screen_width()*0.05))as usize;
                self.selected = i as isize;
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
