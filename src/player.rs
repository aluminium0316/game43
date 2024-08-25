
use core::f32;

use macroquad::prelude::*;
use crate::{blocks::{block::MultiBlock, chunk::{BlockPos, Chunk}}, input::Input, items::items::{Item, ItemId}, key, mesh::{self, vertex, MeshBuilder}};

pub struct Player {
    pos: Vec3,
    vel: Vec3,
    dir: Vec2,
    inventory: Vec<Item>,
    selected: usize,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vec3::default(),
            vel: Vec3::default(),
            dir: Vec2::default(),
            inventory: vec![
                Item::new(ItemId::Drill, 1),
                Item::new(ItemId::Conveyor, 1),
            ],
            selected: 0,
        }
    }

    pub fn update(&mut self, input: &mut Input, chunk: &mut Chunk) {
        if is_mouse_button_down(MouseButton::Left) {
            self.dir.x -= input.dy / 1.5;
            self.dir.y += input.dx / 1.5;
        }

        let mut v = Vec3::new(0.0, 0.0, 0.0);
        if input.key[key!(W)] {
            v.z += 1.0;
        }
        if input.key[key!(A)] {
            v.x += 1.0;
        }
        if input.key[key!(S)] {
            v.z -= 1.0;
        }
        if input.key[key!(D)] {
            v.x -= 1.0;
        }
        if input.down[key!(Q)] < 4 {
            input.down[key!(Q)] = 4;
            self.pos.y += 1.0;
        }
        if input.down[key!(E)] < 4 && self.pos.y > 0.0 {
            input.down[key!(E)] = 4;
            self.pos.y -= 1.0;
        }
        if input.down[key!(Key1)] < 4 {
            input.down[key!(Key1)] = 4;
            self.selected += 1;
            if self.selected > self.inventory.len() - 1 {
                self.selected = 0;
            }
        }

        if input.down[2] == 0 {
            if !chunk.add_block(self.block_pos(), self.inventory[self.selected].id.to_block()) {
                println!("failed to place");
            }
        }

        let rot = Quat::from_axis_angle(Vec3::Y, self.dir.y);
        self.vel += rot * v / 256.0;

        if input.key[key!(LeftShift)] {
            self.vel.x *= 0.8;
            self.vel.z *= 0.8;
        }
        else {
            self.vel.x *= 0.95;
            self.vel.z *= 0.95;
        }

        self.pos += self.vel;

        if self.dir.x > f32::consts::PI - 0.01 {
            self.dir.x = f32::consts::PI - 0.01;
        }
        if self.dir.x < 0.01 {
            self.dir.x = 0.01;
        }
    }

    pub fn render(&self, assets: &Vec<Texture2D>) {
        let mesh = MeshBuilder::new(self.pos, 0.0)
            .set_texture(assets[0].weak_clone(), 0.25)
            .add_rect(vec3(-0.5, 0.0, -0.5), Vec3::X, Vec3::Z, 0.0, 0.0)
            .add_rect(vec3(-0.5, 1.0, -0.5), Vec3::X, -Vec3::Y, 0.0, 0.0)
            .add_rect(vec3( 0.5, 1.0, -0.5), Vec3::Z, -Vec3::Y, 0.0, 0.0)
            .add_rect(vec3( 0.5, 1.0,  0.5), -Vec3::X, -Vec3::Y, 0.0, 0.0)
            .add_rect(vec3(-0.5, 1.0,  0.5), -Vec3::Z, -Vec3::Y, 0.0, 0.0)
            .add_rect(vec3(-0.5, 1.0, -0.5), Vec3::X, Vec3::Z, 0.0, 0.0)
            .build();
        draw_mesh(&mesh);
    }

    pub fn ui(&self) {
        let mut i = 0;
        for item in &self.inventory {
            draw_text(&format!("{:?}", item), screen_width() / 2.0 - 256.0 + 8.0, screen_height() / 2.0 - 144.0 + 8.0 + i as f32 * 16.0, 16.0, if i == self.selected { BLUE } else { BLACK });
            i += 1;
        }
    }

    pub fn camera(&self) {
        set_camera(&Camera3D {
            position: self.pos - 6.0 * vec3(self.dir.x.sin() * self.dir.y.sin(), self.dir.x.cos(), self.dir.x.sin() * self.dir.y.cos()),
            target: self.pos,
            fovy: 90.0,
            up: Vec3::Y,
            ..Default::default()
        });
    }

    pub fn block_pos(&self) -> BlockPos {
        BlockPos::new(self.pos.x.floor() as i64, self.pos.y.floor() as i64, self.pos.z.floor() as i64)
    }
}