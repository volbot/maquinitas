use macroquad::prelude::*;

pub mod floor;
pub mod display;
pub mod maqs;

#[macroquad::main("BasicShapes")]
async fn main() {

    //game vars
    let mut game_floor = floor::init_floor();

    //vars for filling floor
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut flop: u8 = 0;

    //for each x
    while x < 10{

        //flipflop Flop when it gets here
        if flop == 0 {
            flop = 1;
        } else {
            flop = 0;
        }

        //for each y
        while y < 10{

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
    loop {
        //clear screen to red
        clear_background(RED);

        //draw the floor
        display::draw(game_floor.clone());

        //wait for the next frame to arrive before looping again
        next_frame().await;
    }
}
