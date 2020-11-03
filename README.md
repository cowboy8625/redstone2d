# Redstone 2D

##### Installation

Install *Rust* at https://rustup.rs then down load this project and run `cargo run --release` in the terminal project folder.
you also will need to place `resource` folder in `target/release/` folder and it would look like `target/release/resources`.

##### KeyBindings

Select Block:
    1 = Air
    2 = Redstone Dust
    3 = Redstone Block
    4 = Iron
    5 = Repeater

Change Direction of Block:
    W = North/Up
    S = South/Down
    A = West/Right
    D = East/Left

Save & Load:
    Contol + S = Save World
    Contol + L = Load World

Miscellanies:
    C = Clear Screen
    F = Fill Screen with currect block selected


##### Plains

I want the redstone in Redstone 2D to behave the same as in Java minecraft so that at some point I can export the world
to a schematic file for litematica so you can place your contraption in minecraft it's self.

- [x] Save World to a ron file.
- [x] Load World for a ron file.
- [ ] Make Redstone update in one game tick.
- [ ] Add Repeater tick delay.
- [ ] Solid Block may be powered.
