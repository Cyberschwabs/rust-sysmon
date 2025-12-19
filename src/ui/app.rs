#[derive(Default)]
pub struct App {
    pub should_quit: bool,
}

impl App {
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
