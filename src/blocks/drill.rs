use macroquad::prelude::*;

use crate::mesh::MeshBuilder;

use super::{block::MultiBlock, chunk::BlockPos};

pub struct Drill {
    
}

impl MultiBlock for Drill {
    fn place_offset(&self, pos: BlockPos) -> Vec<BlockPos> {
        vec![
            BlockPos::new(0, 0, 0),
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
}