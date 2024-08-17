use std::f32::consts::PI;

use macroquad::models::Vertex;
use macroquad::prelude::*;

pub const fn vertex(x: f32, y: f32, z: f32, u: f32, v: f32, color: Color) -> Vertex {
    Vertex {
        position: Vec3 { x, y, z },
        uv: Vec2 { x: u, y: v },
        color,
    }
}

pub struct MeshBuilder {
    pixel_density: f32,
    rects: Vec<Vertex>,
    texture: Option<Texture2D>,
    pos: Vec3,
    rot: Quat,
}

impl MeshBuilder {
    pub fn new(pos: Vec3, r: f32) -> Self {
        let rot = Quat::from_axis_angle(Vec3::Y, PI / 2.0 * r);
        Self {
            pixel_density: 16.0,
            rects: Vec::new(), 
            texture: None,
            pos,
            rot,
        }
    }

    pub fn add_rect(&mut self, pos: Vec3, x: Vec3, y: Vec3, u0: f32, v0: f32) -> &mut Self {
        let pos0 = pos + self.pos;
        let x0 = self.rot * x;
        let y0 = self.rot * y;
        let pos1 = pos0 + x0;
        let pos2 = pos0 + y0;
        let pos3 = pos0 + x0 + y0;
        let u1 = u0 + self.pixel_density * x.length();
        let v1 = v0 + self.pixel_density * y.length();
        self.rects.append(&mut vec![
            vertex(pos0.x, pos0.y, pos0.z, u0, v0, WHITE),
            vertex(pos1.x, pos1.y, pos1.z, u1, v0, WHITE),
            vertex(pos2.x, pos2.y, pos2.z, u0, v1, WHITE),
            vertex(pos3.x, pos3.y, pos3.z, u1, v1, WHITE),
        ]);
        self
    }

    pub fn set_texture(&mut self, tex: Texture2D, density: f32) -> &mut Self {
        self.texture = Some(tex);
        self.pixel_density = density;
        self
    }

    pub fn build(&self) -> Mesh {
        let mut ind = vec![0, 1, 3, 0, 2, 3];
        let mut indices = Vec::new();
        for _ in 0..self.rects.len() / 4 {
            indices.append(&mut ind.clone());
            for i in &mut ind {
                *i += 4;
            }
        }
        Mesh {
            vertices: self.rects.clone(),
            texture: self.texture.clone(),
            indices,
        }
    }
}