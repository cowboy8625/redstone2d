use ggez::graphics::Image;
use crate::Context;
use crate::GameResult;
use crate::CELL;
use crate::Block;
use crate::Repeater;


fn get_sprite(name: &str) -> String {
    format!("/sprites/{}.png", name)
}

pub struct Assets {
    pub iron: Image,
    pub air: Image,
    redstone_dust:  Image,
    pub redstone_block: Image,
    repeater: Image,
    repeater_on: Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let iron = Image::new(ctx, &get_sprite("iron_block"))?;
        let air = Image::solid(ctx, CELL as u16, (0.0, 0.0, 0.0, 0.0).into())?;
        let redstone_block = Image::new(ctx, &get_sprite("redstone_block"))?;
        let redstone_dust = Image::solid(ctx, CELL as u16, (0, 0, 0, 0).into())?;
        let repeater = Image::new(ctx, &get_sprite("repeater"))?;
        let repeater_on = Image::new(ctx, &get_sprite("repeater_on"))?;
        Ok( Self {
            iron, air, redstone_block, redstone_dust, repeater, repeater_on,
        })
    }

    fn repeater(&self, block: &Repeater) -> &Image {
        match block.powered {
            true => &self.repeater_on,
            false => &self.repeater,
        }
    }

    fn redstone_dust(&mut self, ctx: &mut Context, power_level: u8) {
        let r = power_level * 12 + 75;
        let color = (r, 0, 0, 255).into();
        self.redstone_dust = Image::solid(ctx, CELL as u16, color).expect("RedstoneDust Image error");
    }

    pub fn get(&mut self, ctx: &mut Context, block: &Block) -> &Image {
        match block {
            Block::RedstoneDust(b) => {
                self.redstone_dust(ctx, b.power_level);
                &self.redstone_dust
            }
            Block::RedstoneBlock(_) => &self.redstone_block,
            Block::Iron(_) => &self.iron,
            Block::Air(_) => &self.air,
            Block::Repeater(r) => self.repeater(r),
        }
    }
}