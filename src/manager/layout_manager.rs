use crate::prelude::*;
use super::manager::*;
use crate::frame_types::layout::{self, Layout};

pub struct LayoutManager {
    pub layout: Layout,
    manager: Manager,
}

impl ManagerTrait for LayoutManager {
    ///Calls the update function on the root frame.
    fn update(&mut self) {
        self.manager.update(self.layout.clone())
    }
    
    ///Checks if the screen size has changed and if it has sets it to the new size and returns true, else false.
    fn match_size(&mut self) -> Result<(), ErrorKind> {
        self.manager.match_size()
    }

    ///Gets the current manager size
    fn size(&self) -> Coord {
        self.manager.size()
    }

    ///Draws all of the areas given by the tasks onto the screen.
    fn draw(&mut self) -> Result<(), ErrorKind> {
        self.manager.draw(self.layout.clone())
    }

    ///Returns the next input value automatically handling screen resizes.
    fn get_input(&mut self) -> Input {
        self.manager.get_input()
    }

    ///Returns the next input but returns if the duration is met. automatically handling screen resizes.
    fn poll_input(&mut self, duration: Duration) -> Option<Input> {
        self.manager.poll_input(duration)
    }

    ///Returns a list of all inputs that occurred during the given duration. Automatically handling screen resizes.
    fn inputs_over_duration(&mut self, inputs: &mut Vec<Input>, duration: Duration) {
        self.manager.inputs_over_duration(inputs, duration)
    }

    ///true goes to alt screen false returns from alt.
    fn set_alt_screen(&mut self, alt: bool) {
        self.manager.set_alt_screen(alt)
    }

    fn toggle_alt_screen(&mut self) {
        self.manager.toggle_alt_screen()
    }
}

impl LayoutManager {
    pub fn new() -> Result<Self, ErrorKind> {
        Ok(Self {
            layout: layout::new(),
            manager: Manager::new()?,
        })
    }
}