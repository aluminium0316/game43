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
    let mut ticks = 0;
    let assets = vec![
        load_texture("assets/player.png").await.unwrap(),
        load_texture("assets/ore.png").await.unwrap(),
        load_texture("assets/drill.png").await.unwrap(),
    ];

    for asset in &assets {
        asset.set_filter(FilterMode::Nearest);
    }
    
    let mut fullscreen = false;
    
    let material = load_material(ShaderSource::Glsl { 
        vertex: &VERTEX_SHADER, 
        fragment: &FRAGMENT_SHADER 
    }, MaterialParams {
        pipeline_params: PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        },
        ..Default::default()
    }).unwrap();

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
                player.update(&mut input, &mut chunk);
            }
            chunk.update(ticks);
            input.update();

            i += 1;
            if i > 8 {
                break;
            }

            ticks += 1;
        }

        clear_background(LIGHTGRAY);

        players[0].camera();

        gl_use_material(&material);

        for player in &mut players {
            player.render(&assets);
        }

        chunk.render(&assets);
        
        next_frame().await
    }
}

const FRAGMENT_SHADER: &'static str = "#version 330
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    vec4 color = texture2D(Texture, uv);
    if (color.w < 0.5) {
        discard;
    }
    gl_FragColor = color;
}
";

const VERTEX_SHADER: &'static str = "#version 330
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
";