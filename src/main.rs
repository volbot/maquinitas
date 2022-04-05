use macroquad::prelude::*;

pub mod floor;
pub mod toolbar;
pub mod display;
pub mod maqs;

use crate::display::Drawable;

#[macroquad::main("BasicShapes")]
async fn main() {

    //game vars
    let mut game_floor = floor::init_floor();
    let toolbar = toolbar::init_toolbar();

    //vars for filling floor
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut flop: u8 = 0;

    //for each x
    while x < game_floor.wid{

        //flipflop Flop when it gets here
        if flop == 0 {
            flop = 1;
        } else {
            flop = 0;
        }

        //for each y
        while y < game_floor.len{

            //flipflop Flop when it gets here
            if flop == 0 {
                flop = 1;
            } else {
                flop = 0;
            }

            //add Flop to the states Vec
            game_floor.states[(x*game_floor.wid)+y] = flop;

            y+=1;
        }
        y=0;
        x+=1;
    }

    //MAIN GAME LOOP
    loop {
        //clear screen to red
        clear_background(GRAY);

        //draw things
        game_floor.draw();
        toolbar.draw();

        //query actions
        

        //wait for the next frame to arrive before looping again
        next_frame().await;
    }
}
