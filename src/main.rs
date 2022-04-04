use macroquad::prelude::*;

pub mod floor;
pub mod display;

#[macroquad::main("BasicShapes")]
async fn main() {

    let game_floor = floor::init_floor();

    loop {
        clear_background(RED);

        display::draw(game_floor.clone());

        next_frame().await;
    }
}
