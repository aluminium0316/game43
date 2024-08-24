use std::{arch::x86_64::_CMP_GE_OS, collections::HashMap};

use macroquad::prelude::*;

use super::{block::MultiBlock, event::Event};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct BlockPos {
    x: i64,
    y: i64,
    z: i64,
}

impl BlockPos {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

impl std::ops::Add for BlockPos {
    type Output = BlockPos;

    fn add(self, rhs: Self) -> Self::Output {
        BlockPos::new(self.x+rhs.x, self.y+rhs.y, self.z+rhs.z)
    }
}

impl std::ops::Mul<i64> for BlockPos {
    type Output = BlockPos;

    fn mul(self, rhs: i64) -> Self::Output {
        BlockPos::new(self.x*rhs, self.y*rhs, self.z*rhs)
    }
}

impl Into<Vec3> for BlockPos {
    fn into(self) -> Vec3 {
        vec3(self.x as f32, self.y as f32, self.z as f32)
    }
}

pub struct Chunk {
    blocks: HashMap<u64, (Box<dyn MultiBlock>, BlockPos)>,
    access: HashMap<BlockPos, u64>,
    max_id: u64,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            access: HashMap::new(),
            max_id: 0,
        }
    }

    pub fn add_block(&mut self, center: BlockPos, block: Box<dyn MultiBlock>) -> bool {
        let mut fail = false;
        for pos in block.place_offset(center) {
            if let Some(_) = self.access.get(&(pos + center.clone())) {
                fail = true;
            }
        }
        if fail {
            return false;
        }
        for pos in block.place_offset(center) {
            self.access.insert(pos + center.clone(), self.max_id);
        }
        
        self.blocks.insert(self.max_id, (block, center));
        self.max_id += 1;

        return true;
    }

    pub fn render(&self, assets: &Vec<Texture2D>) {
        for (_id, (multiblock, pos)) in &self.blocks {
            multiblock.render(assets, *pos);
        }
    }

    pub fn update(&mut self, ticks: u64) {
        let mut events = Vec::new();
        for (_id, (multiblock, _pos)) in &mut self.blocks {
            let event = multiblock.update(ticks);
            match event {
                Event::None => {}
                _ => {
                    events.push(event);
                }
            }
        }
        while !events.is_empty() {
            let mut new_events = Vec::new();
            for event in &events {
                
            }
            events.clear();
            events.append(&mut new_events);
        }
    }
}