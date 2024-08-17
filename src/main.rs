mod input;
mod player;
mod blocks;
mod mesh;
mod items;

use blocks::chunk::Chunk;
use input::Input;
use macroquad::prelude::*;
use player::Player;

fn window_conf() -> Conf {
    Conf {
        window_title: "41".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut input = Input::new();
    let mut players = vec![Player::new()];
    let mut chunk = Chunk::new();

    let t = std::time::Instant::now();
    let mut prev_ns = 0;
    let mut dt = 0;
    let fps = 240;
    let assets = vec![
        load_texture("assets/player.png").await.unwrap(),
        load_texture("assets/ore.png").await.unwrap(),
        load_texture("assets/drill.png").await.unwrap(),
    ];

    for asset in &assets {
        asset.set_filter(FilterMode::Nearest);
    }
    
    let mut fullscreen = false;
    

    loop {
        input.input();

        if input.down[key!(F11)] == 0 {
            fullscreen ^= true;
            set_fullscreen(fullscreen);
        }

        let ns = t.elapsed().as_nanos();
        dt += ns - prev_ns;
        prev_ns = ns;
        let mut i = 0;

        while dt > 1000000000/fps {
            dt -= 1000000000/fps;

            for player in players.iter_mut() {
                player.update(&input, &mut chunk);
            }
            input.update();

            i += 1;
            if i > 8 {
                break;
            }
        }

        clear_background(LIGHTGRAY);

        players[0].camera();

        draw_grid(16, 1.0, BLUE, GRAY);

        for player in &mut players {
            player.render(&assets);
        }

        chunk.render(&assets);
        
        next_frame().await
    }
}