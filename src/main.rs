use macroquad::prelude::*;

pub mod floor;
pub mod display;
pub mod maqs;

#[macroquad::main("BasicShapes")]
async fn main() {

    let mut game_floor = floor::init_floor();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut flop: u8 = 0;
    while x < 10{

        if flop == 0 {
            flop = 1;
        } else {
            flop = 0;
        }

        while y < 10{

            if flop == 0 {
                flop = 1;
            } else {
                flop = 0;
            }

            game_floor.states.push(flop);
            y+=1;
        }
        y=0;
        x+=1;
    }
    loop {
        clear_background(RED);

        display::draw(game_floor.clone());

        next_frame().await;
    }
}
