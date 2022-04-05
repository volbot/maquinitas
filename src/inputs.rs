use macroquad::prelude::*;

use crate::floor::MaqFloor;
use crate::toolbar::Toolbar;

//Defines something clickable
pub trait Clickable {
    //Function to run when clicking something.
    // mouse: MouseButton enum
    // pos: Tuple with the x and y
    // dat: any data that needs to get passed to the clicked entity
    fn click(&mut self, mouse: MouseButton, pos: (f32, f32), dat: i16);
}

//Main parse loop, another loop will run if a non-main menu is active
pub fn parse_main(toolbar: &mut Toolbar, floor: &mut MaqFloor) {
    //Left click block
    if is_mouse_button_down(MouseButton::Left) {
       let (x, y) = mouse_position();
       //If x is to the right of the toolbar
       if x > (screen_width()*0.95) {
           toolbar.click(MouseButton::Left, (x, y), 0);
       } else {
           //Left click floor, passing selected ID from toolbar
           floor.click(MouseButton::Left, (x, y), toolbar.selected);
       }
    }
}
