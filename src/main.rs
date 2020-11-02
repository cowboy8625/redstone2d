mod blocks;
mod assets;
use blocks::{Direction, Block, RedstoneDust, RedstoneBlock, Repeater, Iron, Air};
use assets::Assets;
use ggez::{
    Context, GameResult, event, graphics, timer,
    graphics::DrawParam,
    event::KeyCode,
    input::mouse::MouseButton,
};


const CELL: u32 = 16;
const CW: u32 = 60;
const CH: u32 = 40;
const SCREEN_SIZE: (f32, f32) = (
    (CELL * CW) as f32,
    (CELL * CH) as f32
    );

// Redstone signal
const MAX_SIGNAL: u8 = 15;
const MIN_SIGNAL: u8 = 0;

type World = Vec<Block>;

fn _display_fps(ctx: &mut Context) -> GameResult {
    let text = graphics::Text::new(graphics::TextFragment::new(format!(
        "FPS: {}",
        timer::fps(ctx) as u16
    )));
    graphics::draw(ctx, &text, graphics::DrawParam::default())?;
    Ok(())
}

trait RenderBlock {
    fn render(&self, ctx: &mut Context, idx: usize, assets: &mut Assets) -> GameResult;
}

fn from_idx(idx: usize) -> (u32, u32) {
    (idx as u32 % CW, idx as u32 / CW)
}

fn from_cords(x: u32, y: u32) -> usize {
    (y * CW + x) as usize
}


struct Player {
    current: Block,
    facing: Direction,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            current: Block::from(Air::new()),
            facing: Direction::North,
        }
    }
}

struct Mouse {
    x: f32,
    y: f32,
    left: bool,
    right: bool,
    middle: bool,
}

impl Default for Mouse {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, left: false, right: false, middle: false }
    }
}


fn clamp<T>(x: T, min: T, max: T) -> T where T: Ord{
    std::cmp::max(min, std::cmp::min(x, max))
}

fn is_powered(idx: u32, dir: &Direction, world: &World) -> bool {
    let idx_by = match dir {
        Direction::North => CW as i32,
        Direction::South => CW as i32 * -1,
        Direction::West => 1,
        Direction::East => -1,
    };
    if let Some(block) = world.get(idx as usize + idx_by as usize) {
        match block {
            Block::RedstoneBlock(_) => true,
            Block::RedstoneDust(r) => if r.power_level > 0 { true } else { false },
            _ => false,

        }
    } else {
        false
    }
}

fn power_check(idx: u32, world: &World) -> u8 {
    use Direction::*;
    let mut power = 0;
    let idx = idx as i32;
    for idx_by in vec![North, South, East, West] {
        if let Some(block) = world.get(idx as usize + idx_by.clone() as usize) {
            match block {
                Block::RedstoneBlock(_) => power = 15,
                Block::RedstoneDust(b) => {
                    if b.power_level > 0 {
                        power = std::cmp::max(power, clamp(b.power_level - 1, MIN_SIGNAL, MAX_SIGNAL));
                    }
                },
                Block::Repeater(r) => if r.powered {
                    match (&r.facing, idx_by) {
                        (Direction::North, Direction::South) |
                        (Direction::South, Direction::North) |
                        (Direction::West, Direction::East) |
                        (Direction::East, Direction::West)  => power = 15,
                        _ => {},
                    }
                },
                _ => {},
            }
        }
    }
    power
}

fn advance_world(world: &World) -> World {
    let mut new_world = Vec::new();
    for idx in 0..CW * CH {
        if let Some(block) = world.get(idx as usize) {
            // Do some Logic
            match block {
                Block::Air(_) | Block::Iron(_) | Block::RedstoneBlock(_) => {
                    new_world.push(block.clone());
                },
                Block::RedstoneDust(_) => {
                    let mut new_block = RedstoneDust::new();
                    new_block.power_level = power_check(idx, &world);
                    new_world.push(Block::from(new_block));
                },
                Block::Repeater(r) => {
                    let mut new_block = Repeater::new(r.facing.clone());
                    new_block.powered = is_powered(idx, &r.facing, &world);
                    new_world.push(Block::from(new_block));
                },
            }
        }
    }
    new_world
}

fn create_world(block: &Block) -> World {
    let mut world = Vec::new();
    (0..CW * CH).for_each(|_| world.push(block.clone()));
    world
}


struct GameState {
    player: Player,
    world: Vec<Block>,
    mouse: Mouse,
    assets: Assets,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let world = create_world(&Block::from(Air::new()));
        let assets = Assets::new(ctx)?;

        Ok(Self {
            player: Player::default(),
            world,
            mouse: Mouse::default(),
            assets,
        })
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            if self.mouse.left {
                if let Some(block) = self.world.get_mut(
                    // y * width + x
                    (self.mouse.y as u32 / CELL * CW + self.mouse.x as u32 / CELL) as usize) {
                    *block = self.player.current.clone();
                }
            }

            self.world = advance_world(&self.world);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());



        //let mut meshbuilder = graphics::MeshBuilder::new();

        // Give mesh builder to other objects
        for (idx, block) in self.world.iter().enumerate() {
            block.render(ctx, idx, &mut self.assets)?;
        }

        // let mesh = meshbuilder.build(ctx)?;
        // graphics::draw(ctx, &mesh, DrawParam::default())?;

        //_display_fps(ctx)?;

        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: ggez::input::keyboard::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Key1 => self.player.current = Block::from(Air::new()),
            KeyCode::Key2 => self.player.current = Block::from(RedstoneDust::new()),
            KeyCode::Key3 => self.player.current = Block::from(RedstoneBlock::new()),
            KeyCode::Key4 => self.player.current = Block::from(Iron::new()),
            KeyCode::Key5 => self.player.current = Block::from(Repeater::new(self.player.facing.clone())),
            KeyCode::C    => self.world = create_world(&Block::from(Air::new())),
            KeyCode::F    => self.world = create_world(&self.player.current),
            KeyCode::W    => self.player.current.update(Direction::North),
            KeyCode::S    => self.player.current.update(Direction::South),
            KeyCode::A    => self.player.current.update(Direction::West),
            KeyCode::D    => self.player.current.update(Direction::East),
            _ => {},
        }
        println!("{:?}", self.player.current);
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymod: ggez::input::keyboard::KeyMods,
    ) {
    }
    fn mouse_motion_event(
    &mut self,
    _ctx: &mut Context,
    x: f32,
    y: f32,
    _dx: f32,
    _dy: f32
    ) {
        self.mouse.x = x;
        self.mouse.y = y;
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32
    ) {
        match button {
            MouseButton::Left   => self.mouse.left = true,
            MouseButton::Right  => self.mouse.right = true,
            MouseButton::Middle => self.mouse.middle = true,
            MouseButton::Other(_) => {},
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32
    ) {
        match button {
            MouseButton::Left   => self.mouse.left = false,
            MouseButton::Right  => self.mouse.right = false,
            MouseButton::Middle => self.mouse.middle = false,
            MouseButton::Other(_) => {},
        }
    }
}

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("Redstone", "Cowboy8625")
        .window_setup(ggez::conf::WindowSetup::default().title("Redstone"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("Failed to build ggez context");

    let state = &mut GameState::new(&mut ctx)?;
    match event::run(&mut ctx, &mut event_loop, state) {
        Err(e) => println!("Error encountered running game: {}", e),
        Ok(_) => println!("Game exited cleanly!"),
    }
    Ok(())
}
