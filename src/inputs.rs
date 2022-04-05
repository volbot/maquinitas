use macroquad::prelude::*;

use crate::floor::MaqFloor;
use crate::toolbar::Toolbar;

pub trait Clickable {
    fn click(&mut self, mouse: MouseButton, pos: (f32, f32), dat: i16);
}

pub fn parse_main(toolbar: &mut Toolbar, floor: &mut MaqFloor) {
    if is_mouse_button_down(MouseButton::Left) {
       let (x, y) = mouse_position();
       if x > (screen_width()*0.95) {
           toolbar.click(MouseButton::Left, (x, y), 0);
       } else {
           floor.click(MouseButton::Left, (x, y), toolbar.selected);
       }
    }
}
