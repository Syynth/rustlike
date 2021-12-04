use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Up | VirtualKeyCode::W => Point::new(0, -1),
                VirtualKeyCode::Down | VirtualKeyCode::S => Point::new(0, 1),
                VirtualKeyCode::Left | VirtualKeyCode::A => Point::new(-1, 0),
                VirtualKeyCode::Right | VirtualKeyCode::D => Point::new(1, 0),
                _ => Point::zero(),
            };

            let new_pos = self.position + delta;
            if map.can_enter_tile(new_pos) {
                self.position = new_pos;
            }
        }
    }
}
