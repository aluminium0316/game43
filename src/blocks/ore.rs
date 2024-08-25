use macroquad::prelude::*;
use crate::mesh::MeshBuilder;

use super::{block::MultiBlock, chunk::BlockPos, event::Event};

pub struct CoalOre {
    
}

impl MultiBlock for CoalOre {
    fn place_offset(&self, pos: BlockPos) -> Vec<BlockPos> {
        vec![
            BlockPos::new(0, 0, 0),
        ]
    }
    fn render(&self, assets: &Vec<Texture2D>, pos: BlockPos) {
        let mesh = MeshBuilder::new(pos.into(), 0.0)
            .set_texture(assets[1].weak_clone(), 0.25)
            .add_rect(vec3(0.0, 1.0, 0.0), Vec3::X, Vec3::Z, 0.0, 0.0)
            .add_rect(vec3(0.0, 1.0, 0.0), Vec3::X, -Vec3::Y / 2.0, 0.0, 0.0)
            .add_rect(vec3(1.0, 1.0, 0.0), Vec3::Z, -Vec3::Y / 2.0, 0.0, 0.0)
            .add_rect(vec3(1.0, 1.0, 1.0), -Vec3::X, -Vec3::Y / 2.0, 0.0, 0.0)
            .add_rect(vec3(0.0, 1.0, 1.0), -Vec3::Z, -Vec3::Y / 2.0, 0.0, 0.0)
            .build();

        draw_mesh(&mesh);
    }

    fn update(&mut self, ticks: u64) -> Vec<Event> {
        vec![]
    }
}