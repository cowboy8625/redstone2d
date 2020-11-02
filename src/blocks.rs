use crate::graphics;
use crate::{RenderBlock, CELL, CW, from_idx, Assets, Context, GameResult};

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    North = CW as isize * -1,
    South = CW as isize,
    West = -1,
    East = 1,
}

#[derive(Debug, Clone)]
pub enum Block {
    RedstoneDust(RedstoneDust),
    RedstoneBlock(RedstoneBlock),
    Iron(Iron),
    Air(Air),
    Repeater(Repeater),
}

impl Block {
    pub fn rotation(&self) -> f32 {
        let tau = std::f32::consts::TAU;
        match self {
            Self::RedstoneDust(_) => 0.0,
            Self::RedstoneBlock(_) => 0.0,
            Self::Iron(_) => 0.0,
            Self::Air(_) => 0.0,
            Self::Repeater(r) => {
                match r.facing {
                    Direction::North => 0.0,
                    Direction::East => 1.570796,//tau / 4.0,
                    Direction::South => 3.141593,//tau / 2.0,
                    Direction::West => 4.712389,//3.0 * tau / 4.0,
                }
            }
        }
    }

    pub fn update(&mut self, dir: Direction) {
        match self {
            Self::RedstoneDust(_) | Self::RedstoneBlock(_) | Self::Iron(_) | Self::Air(_) => {},
            Self::Repeater(r) => r.facing = dir,
        }
    }
}

impl RenderBlock for Block {
    fn render(&self, ctx: &mut Context, idx: usize, assets: &mut Assets) -> GameResult {
        let (x, y) = from_idx(idx);
        let image = assets.get(ctx, &self);
        let drawparams = graphics::DrawParam::new()
            .rotation(self.rotation())
            .offset([0.5, 0.5])
            .dest([((x * CELL) + CELL / 2) as f32, ((y * CELL) + CELL / 2) as f32]);

        graphics::draw(ctx, image, drawparams)?;
        Ok(())
    }
}

impl From<Air> for Block {
    fn from(air: Air) -> Self {
        Self::Air(air)
    }
}

impl From<Iron> for Block {
    fn from(iron: Iron) -> Self {
        Self::Iron(iron)
    }
}

impl From<RedstoneDust> for Block {
    fn from(rd: RedstoneDust) -> Self {
        Self::RedstoneDust(rd)
    }
}

impl From<RedstoneBlock> for Block {
    fn from(rb: RedstoneBlock) -> Self {
        Self::RedstoneBlock(rb)
    }
}

impl From<Repeater> for Block {
    fn from(r: Repeater) -> Self {
        Self::Repeater(r)
    }
}


/* Block Types */

#[derive(Debug, Clone)]
pub struct Air {
}

impl Air {
    pub fn new() -> Self {
        Self {
        }
    }
}

#[derive(Debug, Clone)]
pub struct Iron {
}

impl Iron {
    pub fn new() -> Self {
        Self {
        }
    }
}

#[derive(Debug, Clone)]
pub struct RedstoneDust {
    pub power_level: u8,
}

impl RedstoneDust {
    pub fn new() -> Self {
        Self {
            power_level: 0
        }
    }
}


#[derive(Debug, Clone)]
pub struct RedstoneBlock {
}

impl RedstoneBlock {
    pub fn new() -> Self {
        Self {
        }
    }
}

#[derive(Debug, Clone)]
pub struct Repeater {
    pub facing: Direction,
    pub powered: bool,
}

impl Repeater {
    pub fn new(facing: Direction) -> Self {
        Self {
            facing,
            powered: false,
        }
    }
}
