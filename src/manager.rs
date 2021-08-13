const SCREEN_OFFSET: i32 = if cfg!(windows){ 1 }
else { 0 };

use crate::{
    shared::*,
    object::*,
    frame_types::Fill
};

use std::{
    io::{stdout, Write},
};

use crossterm::{QueueableCommand,
    style::{Print, SetForegroundColor, SetBackgroundColor},
    cursor, ErrorKind
};

/// The frame Manager holds on to all of the data necessary for drawing frames into the terminal
/// ## Functions
/// - new
/// - new_fill
/// 
/// ## Methods
/// - set_fill
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
    pub objects: Vec<Rc<RefCell<Object>>>,
    tasks: Vec<Task>,
    size: Coord,
    fill: Fill,
}

impl Manager {
    /// Returns a new frame manager, enters a new terminal screen, and is set to update the whole screen on first draw.
    /// - fill = the default character printed when no other data is present.
    pub fn new_fill(fill: Pixel) -> Result<Manager, ErrorKind> {
        Ok(Manager {
            objects: Vec::new(),
            tasks: vec![Task::UpdateAll],
            size: screen_size()?,
            fill: Fill::new_struct(fill),
        })
    }

    /// Returns a new frame manager, enters a new terminal screen, and is set to update the whole screen on first draw.
    pub fn new() -> Result<Manager, ErrorKind> {
        Manager::new_fill(Pixel::new(' ', Color::Rgb{r: 255, g: 255, b: 255}, Color::Rgb{r: 0, g: 0, b: 0}))
    }

    /// Sets the managers fill value.
    pub fn set_fill(&mut self, fill: Pixel) {
        self.fill = Fill::new_struct(fill);
    }

    /// calls the update functions of all contained objects.
    pub fn update_objects(&mut self){
        for obj in &self.objects {
            obj.borrow_mut().size_update(&self.size);
        }
    }

    /// Changes the size of the screen to the new size, and refreshes the screen on next draw.
    pub fn set_size(&mut self, size: Coord) {
        self.size = size;

        self.update_objects();

        self.add_task(Task::UpdateAll);
    }

    /// Checks if the screen size has changed and if it has sets it to the new size and returns true, else false.
    pub fn match_size(&mut self) -> Result<(), ErrorKind>{
        self.set_size(screen_size()?);
        Ok(())
    }

    /// Sets the size based on the output of crossterm Resize event.
    pub fn resize(&mut self, x: u16, y: u16){
        self.set_size(Coord{x: (x as i32) + SCREEN_OFFSET, y: (y as i32) + SCREEN_OFFSET});
    }

    /// Gets the current manager size
    pub fn get_size(&self) -> Coord {
        self.size
    }

    //adds the areas specified by the task to be updated next draw.
    pub fn add_task(&mut self, task: Task) {
        if let Some(first) = self.tasks.first() {
            match first {
            Task::UpdateAll => (),
            _ => {
                match task {
                    Task::UpdateAll => {
                        self.tasks.clear();
                        self.tasks.push(task);
                    },
                    _ => { self.tasks.push(task); }
                }
            }
            }
        }
        else {
            self.tasks.push(task);
        }
    }

    /// Makes a list of all rectangular areas that have requested to be updated.
    fn make_rec_list(&mut self) -> Vec<Rec> {
        let mut reclist: Vec<Rec> = Vec::with_capacity(self.tasks.len());
        let range = Rec{ start: Coord {x: 0, y: 0}, end: self.size};

        for task in self.tasks.drain(..) {
            match task {
                //add one area to the list
                Task::Update(mut rec) => {
                    if rec.in_range(&range) {
                        reclist.push(rec)
                    }
                },
                //adds multiple areas
                Task::UpdateMult(mut recs) => {
                    for mut rec in recs.drain(..) {
                        if rec.in_range(&range){
                            reclist.push(rec)
                        }
                    }
                },
                Task::UpdateAll => {
                    reclist.push(Rec {start: Coord {y: 0, x:0}, end: self.size});
                },
            }
        }

        reclist.sort_unstable_by_key(|rec| rec.start.y);
        //println!("first reclist: {:?}", reclist);
        reclist
    }

    /// Cuts up the list of rectangles into line segments with no overlap.
    fn drawlist(&mut self) -> Vec<DrawData> {
        let mut recs = self.make_rec_list();
        let mut drawlist: Vec<DrawData> = Vec::new();

        while !recs.is_empty() {
            let mut newsegs: Vec<Drawsegment> = Vec::new();
            let cur_y = recs[0].start.y;
            let mut i = 0;

            //add all segments on current line to list
            while (i < recs.len()) && (recs[i].start.y == cur_y) {

                //try to pull a drawsegment of the top of the rec.
                //if the rec has no volume it is dropped from the list
                match recs[i].pull_drawseg() {
                    None => {
                        recs.remove(i);
                    }
                    Some(seg) => {
                        Drawsegment::merge_into_list(&mut newsegs, &seg);
                        i += 1;
                    }
                }
            }
            if !newsegs.is_empty() {
                drawlist.append(&mut self.fill.get_draw_data(&newsegs, Coord{x:0,y:0}, Coord{x:0,y:0}));
            }
        }
        
        drawlist
    }

    /// Draws all of the areas given by the tasks onto the screen.
    pub fn draw(&mut self) -> Result<(), ErrorKind> {
        let mut drawsegs = self.drawlist();
        let objects = &self.objects;

        //let mut i = 0;
        for object in objects {
            let borrowed = object.borrow();
            if borrowed.enabled { borrowed.get_draw_data(&mut drawsegs); }
            //i += 1;
        }
 
        stdout().write_datasegments(drawsegs)?;
        
        stdout().flush()?;
        Ok(())
    }
}

trait TerminalOut {
    fn write_pixel(&mut self, pixle: &PixelData, last_colors: &mut [Option<Color>; 2]) -> Result<(), ErrorKind>;

    fn write_datasegments(&mut self, segments: Vec<DrawData>) -> Result<(), ErrorKind>;
}

impl TerminalOut for std::io::Stdout {
    fn write_pixel(&mut self, pixle: &PixelData, last_colors: &mut [Option<Color>; 2]) -> Result<(), ErrorKind> {

        if !same_color(&pixle.fg, &last_colors[0]) {
            self.queue(SetForegroundColor(pixle.fg))?;
        }
        if !same_color(&pixle.bg, &last_colors[1]) {
            self.queue(SetBackgroundColor(pixle.bg))?;
        }
        
        self.queue(Print(pixle.character))?;
        Ok(())
    }

    fn write_datasegments(&mut self, segments: Vec<DrawData>) -> Result<(), ErrorKind> {
        let mut last_colors: [Option<Color>; 2] = [None; 2];
        for segment in segments {
            self.queue(cursor::MoveTo(segment.start.x as u16, segment.start.y as u16))?;

            for pixle in segment.data {
                match pixle {
                    Pixel::Clear => { self.queue(cursor::MoveRight(1))?; }

                    Pixel::Opaque(data) => {
                        self.write_pixel(&data, &mut last_colors)?;
                        last_colors[0] = Some(data.fg);
                        last_colors[1] = Some(data.bg);
                    }
                }
            }
        }
        
        Ok(())
    }
}

fn same_color(new: &Color, old: &Option<Color>) -> bool {

    match old{
        None => false,
        Some(color) => {
            if color == new { true }
            else { false }
        }
    }
}

fn screen_size() -> Result<Coord, ErrorKind> {
    let (x, y) = crossterm::terminal::size()?;
    Ok(Coord{
        x: (x as i32) + SCREEN_OFFSET,
        y: (y as i32) + SCREEN_OFFSET,
    })
}