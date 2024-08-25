use macroquad::prelude::*;

use crate::{blocks::block::{Side, Z2, Y2, Z1, X1, X2, Y1}, items::{filter::Filter, inventory::{Inventory, Storage}, items::{Item, ItemId}}, mesh::MeshBuilder};

use super::{block::MultiBlock, chunk::BlockPos, event::Event};

#[derive(Default)]
pub struct Conveyor {
    inv: Inventory,
    progress: u64,
    orientation: Orientation
}

#[derive(Default, Debug)]
enum Orientation {
    None,
    #[default]
    X1ToX2,
    X2ToX1,
    Z1ToZ2,
    Z2ToZ1,
    X1Y1ToX2,
    X2Y1ToX1,
    Z1Y1ToZ2,
    Z2Y1ToZ1,
    X1Y2ToX2,
    X2Y2ToX1,
    Z1Y2ToZ2,
    Z2Y2ToZ1,
    X1ToZ1,
    X1ToZ2,
    X2ToZ1,
    X2ToZ2,
    Z1ToX1,
    Z1ToX2,
    Z2ToX1,
    Z2ToX2,
    X1End,
    X2End,
    Z1End,
    Z2End,
}

impl Orientation {
    pub fn pull(&self) -> Side {
        match self {
            Orientation::None => 0,
            Orientation::X1ToX2 => X1,
            Orientation::X2ToX1 => X2,
            Orientation::Z1ToZ2 => Z1,
            Orientation::Z2ToZ1 => Z2,
            Orientation::X1Y1ToX2 => X1 | Y1,
            Orientation::X2Y1ToX1 => X2 | Y1,
            Orientation::Z1Y1ToZ2 => Z1 | Y1,
            Orientation::Z2Y1ToZ1 => Z2 | Y1,
            Orientation::X1Y2ToX2 => X1 | Y2,
            Orientation::X2Y2ToX1 => X2 | Y2,
            Orientation::Z1Y2ToZ2 => Z1 | Y2,
            Orientation::Z2Y2ToZ1 => Z2 | Y2,
            Orientation::X1ToZ1 => X1,
            Orientation::X1ToZ2 => X1,
            Orientation::X2ToZ1 => X2,
            Orientation::X2ToZ2 => X2,
            Orientation::Z1ToX1 => Z1,
            Orientation::Z1ToX2 => Z1,
            Orientation::Z2ToX1 => Z2,
            Orientation::Z2ToX2 => Z2,
            Orientation::X1End => X1,
            Orientation::X2End => X2,
            Orientation::Z1End => Z1,
            Orientation::Z2End => Z2,
        }
    }
}

impl MultiBlock for Conveyor {
    fn place_offset(&self, pos: BlockPos) -> Vec<BlockPos> {
        vec![
            BlockPos::new(0, 0, 0),
        ]
    }

    fn render(&self, assets: &Vec<Texture2D>, pos: BlockPos) {
        let mesh = MeshBuilder::new(pos.into(), 0.0)
            .set_texture(assets[0].weak_clone(), 0.25)
            .add_rect(vec3(0.0, 0.0, 0.0), Vec3::X, Vec3::Z, 0.0, 0.0)
            .add_rect(vec3(0.0, 0.5, 0.0), Vec3::X, -Vec3::Y / 2.0, 0.0, 0.0)
            .add_rect(vec3(1.0, 0.5, 0.0), Vec3::Z, -Vec3::Y / 2.0, 0.0, 0.0)
            .add_rect(vec3(1.0, 0.5, 1.0), -Vec3::X, -Vec3::Y / 2.0, 0.0, 0.0)
            .add_rect(vec3(0.0, 0.5, 1.0), -Vec3::Z, -Vec3::Y / 2.0, 0.0, 0.0)
            .add_rect(vec3(0.0, 0.5, 0.0), Vec3::X, Vec3::Z, 0.0, 0.0)
            .build();
        draw_mesh(&mesh);
        draw_line_3d(pos + vec3(0.5, 0.5, 0.5), pos + self.orientation.pull() + vec3(0.5, 0.5, 0.5), BLUE);
    }

    fn update(&mut self, ticks: u64) -> Vec<Event> {
        if !self.inv.input.is_empty() {
            self.progress += 1;
        }
        if self.progress > 256 && self.inv.output.is_empty() {
            self.progress = 0;
        }
        if self.progress > 128 && !self.inv.output.is_empty() {
            self.progress = 128;
        }
        if self.inv.input.is_empty() {
            self.progress = 0;
            vec![
                Event::Pull { count: 1, filter: Filter::new("*"), side: self.orientation.pull() },
            ]
        }
        else if self.progress == 256 && self.inv.output.is_empty() {
            self.progress = 0;
            vec![
                Event::Craft { input: self.inv.input.clone(), output: self.inv.input.clone() }
            ]
        }
        else {
            vec![]
        }
    }

    fn event(&mut self, event: Event) -> Vec<Event> {
        self.progress = 0;
        if self.inv.input.is_empty() {
            vec![
                Event::Pull { count: 1, filter: Filter::new("*"), side: self.orientation.pull() },
            ]
        }
        else {
            vec![]
        }
    }

    fn log(&self, pos: BlockPos) -> String {
        format!("progress: {} / 256\norientation: {:?}\n{}", self.progress, self.orientation, self.inv.view())
    }

    fn as_storage(&mut self) -> Option<&mut dyn Storage> {
        Some(self)
    }
}

impl Storage for Conveyor {
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