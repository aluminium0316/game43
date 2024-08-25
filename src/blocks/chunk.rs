use std::collections::HashMap;

use macroquad::prelude::*;

use super::{block::{MultiBlock, Side, X1, X2, Y2, Y1, Z1, Z2}, event::Event};

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

impl std::ops::Add<Side> for BlockPos {
    type Output = BlockPos;

    fn add(self, rhs: Side) -> Self::Output {
        BlockPos::new(self.x + if1(rhs, X1) - if1(rhs, X2), self.y + if1(rhs, Y1) - if1(rhs, Y2), self.z + if1(rhs, Z1) - if1(rhs, Z2))
    }
}

fn if1(a: Side, b: Side) -> i64 {
    if a & b != 0 { 1 } else { 0 }
}

impl std::ops::Add<Vec3> for BlockPos {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        let lhs: Vec3 = self.into();
        lhs + rhs
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

    pub fn add_block(&mut self, center: BlockPos, blocks: Option<Box<dyn MultiBlock>>) -> bool {
        if blocks.is_none() {
            return false;
        }
        let block = blocks.unwrap();
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
        for (id, (multiblock, _pos)) in &mut self.blocks {
            let event = multiblock.update(ticks);
            events.append(&mut event
                .iter()
                .map(|x| (x.clone(), id.clone()))
                .collect());
        }
        while !events.is_empty() {
            let mut new_events = Vec::new();
            for event in &events {
                match &event.0 {
                    Event::Craft { input, output } => {
                        if let Some(block) = self.blocks.get_mut(&event.1) {
                            if let Some(storage) = block.0.as_storage() {
                                storage.craft(input.clone(), output.clone());
                            }
                        }
                    },
                    Event::Pull { count, filter, side } => {
                        let blocks: *mut HashMap<u64, (Box<dyn MultiBlock>, BlockPos)> = &mut self.blocks as *mut HashMap<u64, (Box<dyn MultiBlock>, BlockPos)>;
                        if let Some(block) = unsafe { blocks.as_mut().unwrap() }.get_mut(&event.1) {
                            if let Some(access) = self.access.get(&(block.1 + side.clone())) {
                                if let Some(input_block) = unsafe { blocks.as_mut().unwrap() }.get_mut(&access) {
                                    if let Some(input_storage) = input_block.0.as_storage() {
                                        let extract = input_storage.extract(count.clone(), filter.clone(), 0);
                                        if let Some(storage) = block.0.as_storage() {
                                            storage.insert(extract);
                                        }
                                        // let event1 = input_block.0.event(event.0.clone());
                                        // new_events.append(&mut event1
                                        //     .iter()
                                        //     .map(|x| (x.clone(), access.clone()))
                                        //     .collect());
                                    }
                                }
                            }
                        }
                    },
                }
                if let Some(block) = self.blocks.get_mut(&event.1) {
                    block.0.event_callback(event.0.clone());
                }
            }
            events.clear();
            events.append(&mut new_events);
        }
    }

    pub fn log_block(&self, pos: BlockPos) -> String {
        if let Some(access) = self.access.get(&pos) {
            if let Some(block) = self.blocks.get(access) {
                block.0.log(block.1)
            }
            else {
                "Corrupted block".to_owned()
            }
        }
        else {
            "No block".to_owned()
        }
    }
}