use zellij_tile::prelude::*;

#[derive(Default)]
struct State;

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {}

    fn update(&mut self, event: Event) {}

    fn render(&mut self, rows: usize, cols: usize) {}
}
