use macroquad::prelude::*;

pub mod floor;
pub mod toolbar;
pub mod display;
pub mod tiles;
pub mod inputs;

use crate::display::Drawable;

#[macroquad::main("BasicShapes")]
async fn main() {

    //game vars
    let mut game_floor = floor::init_floor();
    let mut toolbar = toolbar::init_toolbar();

    //MAIN GAME LOOP
    loop {
        //clear screen to red
        clear_background(GRAY);

        //draw things
        game_floor.draw();
        toolbar.draw();

        //query actions
        inputs::parse_main(&mut toolbar, &mut game_floor);

        //wait for the next frame to arrive before looping again
        next_frame().await;
    }
}
