use macroquad::texture::Texture2D;

use crate::items::inventory::Storage;

use super::{chunk::BlockPos, drill::Drill, event::Event, ore::CoalOre};

pub type Side = u32;
pub const Y1: u32 = 1;
pub const Y2: u32 = 2;
pub const X1: u32 = 4;
pub const X2: u32 = 8;
pub const Z1: u32 = 16;
pub const Z2: u32 = 32;

pub trait MultiBlock {
    fn place_offset(&self, pos: BlockPos) -> Vec<BlockPos>;
    fn render(&self, assets: &Vec<Texture2D>, pos: BlockPos);
    fn update(&mut self, ticks: u64) -> Vec<Event>;
    fn event(&mut self, event: Event) -> Vec<Event> {
        vec![]
    }
    fn event_callback(&mut self, event: Event) {

    }
    fn log(&self, pos: BlockPos) -> String {
        "".to_owned()
    }
    fn as_storage(&mut self) -> Option<&mut dyn Storage> {
        None
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
    fn update(&mut self, ticks: u64) -> Vec<Event> {
        Vec::new()
    }
}