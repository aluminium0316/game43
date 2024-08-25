use macroquad::prelude::*;

use crate::{blocks::chunk::Chunk, player::Player};

pub struct Jade {
    info: String,
}

impl Jade {
    pub fn new() -> Self {
        Self { info: "".to_owned() }
    }

    pub fn update(&mut self, players: &Vec<Player>, chunk: &Chunk) {
        let pos = players[0].block_pos();

        self.info = chunk.log_block(pos);
    }

    pub fn ui(&self, assets: &Vec<Texture2D>) {
        draw_rectangle(screen_width() - 256.0, screen_height() - 144.0, 256.0, 144.0, Color::from_rgba(128, 136, 144, 72));
        let mut i = 0.0;
        for info in self.info.split('\n') {
            draw_text(info, screen_width() - 256.0 + 8.0, screen_height() - 144.0 + 16.0 + i, 16.0, BLACK);
            i += 16.0;
        }
        println!("{}", self.info);
    }
}