use macroquad::prelude::*;

pub mod floor;

#[macroquad::main("BasicShapes")]
async fn main() {

    init_game().await;

    loop {
        clear_background(RED);

        

        next_frame().await;
    }
}

async fn init_game() {
    
}
