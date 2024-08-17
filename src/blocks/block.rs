use macroquad::texture::Texture2D;

use super::{chunk::BlockPos, drill::Drill, ore::CoalOre};

pub struct Side;
impl Side {
    pub const UP: u32 = 1;
    pub const DOWN: u32 = 2;
    pub const LEFT: u32 = 4;
    pub const RIGHT: u32 = 8;
    pub const FOR: u32 = 16;
    pub const BACK: u32 = 32;
}

pub trait MultiBlock {
    fn place_offset(&self, pos: BlockPos) -> Vec<BlockPos>;
    fn render(&self, assets: &Vec<Texture2D>, pos: BlockPos);
}

pub fn place(id: u64) -> Box<dyn MultiBlock> {
    match id {
        0 => Box::new(CoalOre {}),
        1 => Box::new(Drill {}),
        _ => Box::new(Error),
    }
}


pub struct Error;

impl MultiBlock for Error {
    fn place_offset(&self, pos: BlockPos) -> Vec<BlockPos> {
        vec![
            BlockPos::new(0, 0, 0),
        ]
    }
    fn render(&self, assets: &Vec<Texture2D>, pos: BlockPos) {

    }
}