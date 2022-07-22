const SCREEN_OFFSET: i32 = if cfg!(windows){ 1 }
                                      else { 0 };

use crate::prelude::*;

use std::{
    io::{stdout, Write},
    time::Instant,
};

use crossterm::{
    QueueableCommand, ExecutableCommand,
    style::{Print, SetForegroundColor, SetBackgroundColor},
    event::{read, poll, Event},
    cursor,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen}
};

pub use crossterm::ErrorKind;
pub use std::time::Duration;

pub trait ManagerTrait {
    ///Calls the update function on the root frame.
    fn update(&mut self);

    ///Checks if the screen size has changed and if it has sets it to the new size and returns true, else false.
    fn match_size(&mut self) -> Result<(), ErrorKind>;

    ///Gets the current manager size
    fn size(&self) -> Coord;

    ///Draws all of the areas given by the tasks onto the screen.
    fn draw(&mut self) -> Result<(), ErrorKind>;

    ///Returns the next input value automatically handling screen resizes.
    fn get_input(&mut self) -> Input;

    ///Returns the next input but returns if the duration is met. automatically handling screen resizes.
    fn poll_input(&mut self, duration: Duration) -> Option<Input>;

    ///Returns a list of all inputs that occurred during the given duration. Automatically handling screen resizes.
    fn inputs_over_duration(&mut self, inputs: &mut Vec<Input>, duration: Duration);

    ///true goes to alt screen false returns from alt.
    fn set_alt_screen(&mut self, alt: bool);

    fn toggle_alt_screen(&mut self);
}

/// The frame Manager holds on to all of the data necessary for drawing frames into the terminal
/// ## Functions
/// - new
/// 
/// ## Methods
/// - update_objects
/// - set_size
/// - match_size
/// - resize
/// - get_size
/// - kill
/// - objects
/// - add_task
/// - draw
pub struct Manager {
    screenbuf: ScreenBuf,
    printer: PixelPrinter,
    size_updated: bool,
    alt_screen:   bool,
}

impl Manager {
    /// Returns a new frame manager, enters a new terminal screen, and is set to update the whole screen on first draw.
    pub fn new() -> Result<Manager, ErrorKind> {
        let size = screen_size()?;
        std::io::stdout().execute(EnterAlternateScreen)?;
        //println!("{:?}", size);

        Ok(Manager {
            screenbuf: ScreenBuf::new(size),
            printer: PixelPrinter::new(),
            size_updated: true,
            alt_screen:   true,
        })
    }

    ///Calls the update function on the root frame.
    pub fn update(&mut self, root: Frame){
        root.borrow_mut().update(self.screenbuf.size())
    }

    ///Checks if the screen size has changed and if it has sets it to the new size and returns true, else false.
    pub fn match_size(&mut self) -> Result<(), ErrorKind>{
        self.set_size(screen_size()?);
        Ok(())
    }

    ///Gets the current manager size
    pub fn size(&self) -> Coord {
        self.screenbuf.size()
    }

    ///Draws all of the areas given by the tasks onto the screen.
    pub fn draw(&mut self, root: Frame) -> Result<(), ErrorKind> {
        if self.size_updated {
            self.update(root.clone());
            self.size_updated = false;
        }

        let size = self.screenbuf.size();
        root.borrow().get_draw_data(&mut self.screenbuf, Coord{x: 0, y: 0}, size);

        self.printer.print_buffer(&self.screenbuf)?;
        
        Ok(())
    }

    ///Returns the next input value automatically handling screen resizes.
    pub fn get_input(&mut self) -> Input {
        loop {
            if let Some(input) = self.event_to_input() {
                return input
            }
        }
    }

    ///Returns the next input but returns if the duration is met. automatically handling screen resizes.
    pub fn poll_input(&mut self, duration: Duration) -> Option<Input> {
        if let Ok(true) = poll(duration) {
            self.event_to_input()
        }
        else {
            None
        }
    }

    ///Returns a list of all inputs that occurred during the given duration. Automatically handling screen resizes.
    pub fn inputs_over_duration(&mut self, inputs: &mut Vec<Input>, duration: Duration) {
        let time = Instant::now();
        inputs.clear();

        while time.elapsed() < duration {
            if let Some(input) = self.poll_input(duration - time.elapsed()) {
                inputs.push(input)
            }
        }
    }

    ///true goes to alt screen false returns from alt.
    pub fn set_alt_screen(&mut self, alt: bool) {
        if alt != self.alt_screen {
            if alt {
                let _ = std::io::stdout().execute(EnterAlternateScreen);
            }
            else {
                let _ = std::io::stdout().execute(LeaveAlternateScreen);
            }

            self.alt_screen = alt
        }
    }

    pub fn toggle_alt_screen(&mut self) {
        self.alt_screen = !self.alt_screen;

        if self.alt_screen {
            let _ = std::io::stdout().execute(EnterAlternateScreen);
        }
        else {
            let _ = std::io::stdout().execute(LeaveAlternateScreen);
        }
    }

    /// Changes the size of the screen to the new size, and refreshes the screen on next draw.
    fn set_size(&mut self, size: Coord) {
        self.screenbuf.set_size(size);
        self.size_updated = true;
    }

    /// Sets the size based on the output of crossterm Resize event.
    fn resize(&mut self, x: u16, y: u16){
        self.set_size(Coord{x: (x as i32) + SCREEN_OFFSET, y: (y as i32) + SCREEN_OFFSET});
    }

    fn event_to_input(&mut self) -> Option<Input> {
        match read().unwrap() {
            Event::Resize(x, y) => {
                self.resize(x, y);
                None
            }
            Event::Key(e) => {
                Some(Input::KeyBoard(e))
            }
            Event::Mouse(e) => {
                Some(Input::Mouse(e))
            }
        }
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        //if we are still in an alternate when manager dies try and return to the normal one.
        if self.alt_screen {
            let _ = std::io::stdout().execute(LeaveAlternateScreen);
        }
    }
}

struct PixelPrinter {
    current_fg: Option<Color>,
    current_bg: Option<Color>
}

impl PixelPrinter {
    pub fn new() -> Self {
        PixelPrinter {
            current_fg: None,
            current_bg: None,
        }
    }

    pub fn print_pixel(&mut self, pixel: &Pixel) -> Result<(), ErrorKind> {
        match pixel {
            Pixel::Clear => {stdout().queue(cursor::MoveRight(1))?;}
            Pixel::Opaque(data) => {
                if !(Some(data.fg) == self.current_fg) {
                    stdout().queue(SetForegroundColor(data.fg))?;
                    self.current_fg = Some(data.fg);
                }

                if !(Some(data.bg) == self.current_bg) {
                    stdout().queue(SetBackgroundColor(data.bg))?;
                    self.current_bg = Some(data.bg);
                }

                stdout().queue(Print(data.character))?;
            }
        }

        Ok(())
    }

    pub fn print_buffer(&mut self, buf: &ScreenBuf)  -> Result<(), ErrorKind> {
        stdout().queue(cursor::MoveTo(0, 0))?;

        let pix = &buf.buffer;
        let size = pix.size();

        for i in 0..(size.y * size.x) as usize {
            self.print_pixel(&pix.get_flat(i))?
        }

        stdout().flush()
    }
}

fn screen_size() -> Result<Coord, ErrorKind> {
    let (x, y) = crossterm::terminal::size()?;
    Ok(Coord{
        x: (x as i32),
        y: (y as i32),
    })
}