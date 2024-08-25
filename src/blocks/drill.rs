use macroquad::prelude::*;

use crate::{items::{filter::Filter, inventory::{Inventory, Storage}, items::{Item, ItemId}}, mesh::MeshBuilder};

use super::{block::{MultiBlock, Side}, chunk::BlockPos, event::Event};

#[derive(Default)]
pub struct Drill {
    inv: Inventory,
    progress: u64,
}

impl MultiBlock for Drill {
    fn place_offset(&self, pos: BlockPos) -> Vec<BlockPos> {
        vec![
            BlockPos::new(0, 0, 0),
            BlockPos::new(0, 1, 0),
        ]
    }

    fn render(&self, assets: &Vec<Texture2D>, pos: BlockPos) {
        let mesh = MeshBuilder::new(pos.into(), 0.0)
            .set_texture(assets[2].weak_clone(), 0.25)
            .add_rect(vec3(0.0, 1.0, 0.0), Vec3::X + Vec3::Z, -Vec3::Y, 0.0, 0.0)
            .add_rect(vec3(1.0, 1.0, 0.0), Vec3::Z - Vec3::X, -Vec3::Y, 0.0, 0.0)
            .add_rect(vec3(0.0, 1.0, 0.0), Vec3::X, Vec3::Z, 0.5, 0.0)
            .add_rect(vec3(0.0, 2.0, 0.25), Vec3::X, -Vec3::Y * 2.0, 0.75, 0.0)
            .add_rect(vec3(0.75, 2.0, 0.0), Vec3::Z, -Vec3::Y * 2.0, 0.75, 0.0)
            .add_rect(vec3(1.0, 2.0, 0.75), -Vec3::X, -Vec3::Y * 2.0, 0.75, 0.0)
            .add_rect(vec3(0.25, 2.0, 1.0), -Vec3::Z, -Vec3::Y * 2.0, 0.75, 0.0)
            .build();

        draw_mesh(&mesh);
    }

    fn update(&mut self, ticks: u64) -> Vec<Event> {
        self.progress += 1;

        if self.progress == 256 {
            self.progress = 0;
            vec![
                Event::Craft { input: Vec::new(), output: vec![Item::new(ItemId::Coal, 1)] }
            ]
        }
        else {
            vec![]
        }
    }

    fn log(&self, pos: BlockPos) -> String {
        format!("progress: {} / 256\n{}", self.progress, self.inv.view())
    }

    fn as_storage(&mut self) -> Option<&mut dyn Storage> {
        Some(self)
    }
}

impl Storage for Drill {
    fn insert(&mut self, items: Vec<Item>) {
        self.inv.insert(items)
    }

    fn extract(&mut self, count: u64, filter: Filter, side: Side) -> Vec<Item> {
        self.inv.extract(count, filter)
    }

    fn craft(&mut self, items: Vec<Item>, output: Vec<Item>) {
        self.inv.craft(items, output)
    }
}