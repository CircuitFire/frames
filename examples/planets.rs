use frames::*;
use frame_types::*;
use frames::modifiers::position::Position;
use layout::Object;
use modifiers::*;
use position::update_types::*;
use prelude::*;

use std::io;
use std::time::{SystemTime};

use crossterm::ExecutableCommand;
use crossterm::terminal;

use Color::Rgb;

fn main() {
    io::stdout().execute(terminal::SetTitle("Frames Demo!")).unwrap();

    let mut manager = LayoutManager::new().unwrap();

    let mut main_page = MainPage::new(&mut manager);

    main_page.main(&mut manager);
}

struct MenuUpdate {}

impl position::SizeUpdate for MenuUpdate {
    fn size_update(&mut self, pos: &mut position::PosData, new_size: Coord) {
        pos.pos = Coord {
            y: new_size.y - 2,
            x: (new_size.x / 2) - (pos.size.x / 2)
        }
    }
}

struct Menu {
    text: text::Text,
    pos:  Position,
    exit: bool,
}

impl Menu {
    pub fn new(manager: &mut LayoutManager) -> Self {
        let text = text::new();
        {
            let mut temp = text.borrow_mut();
            use text::Entry;

            temp.entries.push_back(Entry::new_color("Slot Machine.", ColorSet { fg: Rgb { r: 255, g: 255, b: 0 }, bg: Rgb { r: 0, g: 0, b: 0 } }));
            temp.entries.push_back(Entry::new("Exit."));
        }

        let pos = position::craft().size(Coord{x: 30, y: 2}).update(MenuUpdate {}).done();

        manager.layout.borrow_mut().objects.push(Object {
            frame: text.clone(),
            pos:   pos.clone(),
        });

        Menu {
            text,
            pos,
            exit: false
        }
    }

    pub fn toggle(&mut self) {
        self.exit = !self.exit;

        if self.exit {
            let mut temp = self.text.borrow_mut();

            temp.entries[0].colors = None;
            temp.entries[1].colors = Some( ColorSet { fg: Rgb { r: 255, g: 255, b: 0 }, bg: Rgb { r: 0, g: 0, b: 0 } } );
        }
        else {
            let mut temp = self.text.borrow_mut();

            temp.entries[1].colors = None;
            temp.entries[0].colors = Some( ColorSet { fg: Rgb { r: 255, g: 255, b: 0 }, bg: Rgb { r: 0, g: 0, b: 0 } } );
        }
    }

    pub fn exit(&self) -> bool {
        self.exit
    }

    pub fn disable(&mut self) {
        self.pos.borrow_mut().data.enabled = false;
    }

    pub fn enabled(&mut self) {
        self.pos.borrow_mut().data.enabled = true;
    }
}

struct PlanetUpdate {}

impl position::SizeUpdate for PlanetUpdate {
    fn size_update(&mut self, data: &mut position::PosData, new_size: Coord) {
        let temp = Coord{x: (new_size.x / 6), y: ((new_size.y / 5) * 3)};
        data.pos = temp - (data.size / Coord{ x: 2, y: 2 });
    }
}

struct MoonUpdate {
    planet: position::Position,
}

impl position::SizeUpdate for MoonUpdate {
    fn size_update(&mut self, data: &mut position::PosData, _new_size: Coord) {
        let temp = self.planet.borrow().data.pos + Coord{x: 25, y: -5};
        data.pos = temp - (data.size / Coord{ x: 2, y: 2 });
    }
}

struct MainPage {
    planet:     Position,
    moon:       Position,
    menu:       Menu,
    slots:      SlotMachine,
}

impl MainPage {
    pub fn new(manager: &mut LayoutManager) -> Self {
        let background_frame = {
            let s = Pixel::new('*', Rgb{r: 255, g: 255, b: 255}, Rgb{r: 0, g: 0, b: 0});
            let b = Pixel::new('.', Rgb{r: 255, g: 255, b: 255}, Rgb{r: 0, g: 0, b: 0});
            let x = Pixel::new('x', Rgb{r: 255, g: 255, b: 255}, Rgb{r: 0, g: 0, b: 0});
            let n = Pixel::new(' ', Rgb{r: 255, g: 255, b: 255}, Rgb{r: 0, g: 0, b: 0});
    
            let sprite = vec![
                s,n,n,n,n,n,n,n,n,n,n,n,n,n,
                n,n,n,n,n,n,n,n,n,n,n,n,n,n,
                n,n,n,n,n,n,n,n,n,n,n,n,n,n,
                n,n,n,n,n,n,n,n,n,b,n,n,n,n,
                n,n,n,n,n,n,n,n,n,n,n,n,n,n,
                n,n,n,n,n,n,n,n,n,n,n,n,n,n,
                n,n,b,n,n,n,n,n,n,n,n,n,n,n,
                n,n,n,n,n,n,n,n,n,n,n,n,n,n,
                n,n,n,n,n,n,n,n,n,x,n,n,n,n,
                n,n,n,n,n,n,n,n,n,n,n,n,n,n,
                n,n,n,n,n,n,n,n,n,n,n,n,n,n,
                n,n,n,n,s,n,n,n,n,n,n,n,n,n,
                n,n,n,n,n,n,n,n,n,n,n,n,n,n,
                n,n,n,n,n,n,n,n,n,n,n,n,n,n,
            ];
    
            basic::new(Coord{x: 14, y: 14}, sprite)
        }.unwrap();
    
        let planet_frame = {
            let y = Pixel::new('█', Rgb{r: 224, g: 167, b:  43}, Rgb{r: 0, g: 0, b: 0});
            let w = Pixel::new('█', Rgb{r: 230, g: 230, b: 230}, Rgb{r: 0, g: 0, b: 0});
    
            let sprite = vec![
                y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,
                y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,
                y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,w,w,y,y,w,
                w,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,w,w,y,w,y,y,w,w,y,
                y,w,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,w,w,y,w,w,y,w,w,y,y,y,y,y,y,y,w,y,y,w,y,y,y,y,y,y,
                y,y,y,y,y,y,y,y,y,y,y,y,y,w,w,y,w,y,y,w,y,y,w,y,y,w,y,w,w,y,w,w,y,y,y,y,w,y,y,y,y,y,
                y,y,y,y,y,y,y,y,y,y,y,y,w,y,y,w,y,y,y,y,y,y,y,y,y,y,w,y,y,w,y,y,y,y,y,y,y,y,y,y,y,y,
                y,y,y,y,y,y,y,y,y,y,y,w,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,w,y,y,y,y,y,y,y,y,y,y,y,
                y,y,y,y,y,y,y,y,y,y,w,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,w,y,y,y,y,y,y,y,y,y,y,
                y,y,y,y,y,y,y,w,y,w,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,w,y,w,y,y,y,y,y,y,y,
                w,y,y,y,y,w,w,y,w,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,w,y,w,w,y,y,y,y,w,
                y,w,y,w,w,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,w,w,y,w,y,
                y,y,w,y,y,w,w,y,w,w,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,w,y,y,
                y,y,y,y,y,y,y,w,y,y,y,y,y,y,y,y,y,y,w,w,w,w,w,w,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,
                y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,w,w,w,y,y,y,y,y,y,w,w,w,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,
                y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,w,y,y,y,y,y,y,y,y,y,y,w,y,y,y,y,y,y,y,y,y,w,y,y,y,y,y,
                y,y,y,y,y,y,y,y,y,y,y,y,y,w,w,y,y,y,y,y,y,y,y,y,y,y,y,w,w,y,w,w,w,y,w,w,y,w,y,y,y,y,
                y,y,y,y,y,y,y,y,y,y,y,y,w,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,w,y,y,y,w,y,y,y,y,w,y,y,y,
                y,y,y,y,y,y,y,y,y,y,w,w,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,w,y,y,
                y,y,y,y,y,y,y,y,w,w,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,w,y,
                y,y,y,y,y,y,y,w,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,y,w,
            ];
    
            with_modifier::new(
                basic::new(Coord{x: 42, y: 21}, sprite).unwrap(),
                circle_mask::new(false)
            )
        };
    
        let moon_frame = {
            let b = Pixel::new('█', Rgb{r: 140, g: 140, b: 140}, Rgb{r: 140, g: 140, b: 140});
    
            with_modifier::new(
                fill::new(b),
                circle_mask::new(false)
            )
        };

        let planet = position::craft().size(Coord{x: 21, y: 21}).frame_size(Coord{x: 42, y: 21}).update(PlanetUpdate{}).done();
        let moon = position::craft().size(Coord{x: 10, y: 10}).update(MoonUpdate{planet: planet.clone()}).done();
    
        {
            let mut layout = manager.layout.borrow_mut();
    
            layout.objects.push(Object{
                frame: background_frame,
                pos: position::craft().update(MatchSize{}).done(),
            });
            layout.objects.push(Object{
                frame: planet_frame,
                pos: planet.clone()
            });
            layout.objects.push(Object{
                frame: moon_frame,
                pos: moon.clone()
            });
        }

        MainPage {
            planet,
            moon,
            menu: Menu::new(manager),
            slots: SlotMachine::new(manager)
        }
    }

    pub fn main(&mut self, manager: &mut LayoutManager) {
        let mut inputs = Vec::new();

        loop {
            manager.draw().unwrap();
    
            self.planet.borrow_mut().inc_offset(Coord{ x: 1, y: 0 });
    
            manager.fps_input(&mut inputs);

            for input in &inputs {
                if let Input::KeyBoard(e) = input {
                    use crossterm::event::KeyCode::*;

                    match e.code {
                        Esc => {
                            return
                        }
                        Up | Down => {
                            self.menu.toggle()
                        }
                        Enter => {
                            if self.menu.exit() {
                                return
                            }
                            else {
                                self.leave(manager);
                                break
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn leave(&mut self, manager: &mut LayoutManager) {
        self.moon.borrow_mut().data.enabled = false;
        self.planet.borrow_mut().data.enabled = false;
        self.menu.disable();

        self.slots.main(manager);

        self.moon.borrow_mut().data.enabled = true;
        self.planet.borrow_mut().data.enabled = true;
        self.menu.enabled();
    }
}

struct MachineUpdate {}

impl position::SizeUpdate for MachineUpdate {
    fn size_update(&mut self, pos: &mut position::PosData, new_size: Coord) {
        pos.pos = (new_size / Coord{x: 2, y: 2}) - (pos.size / Coord{x: 2, y: 2})
    }
}

struct SlotUpdate {
    machine: Position,
    num:     i32,
}

impl position::SizeUpdate for SlotUpdate {
    fn size_update(&mut self, pos: &mut position::PosData, _new_size: Coord) {
        let mut temp = self.machine.borrow().data.pos + Coord{x: 2, y: 2};
        temp.x += 17 * self.num;

        pos.pos = temp
    }
}

struct SlotMachine {
    machine: Position,
    slot1:   Position,
    slot2:   Position,
    slot3:   Position,
}

impl SlotMachine {
    pub fn new(manager: &mut LayoutManager) -> Self {
        let machine_frame = {
            let c = Pixel::Clear;
            let r = Pixel::new('█', Rgb{r: 255, g: 0, b: 0}, Rgb{r: 0, g: 0, b: 0});

            let sprite = vec![
                c,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,c,
                r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,c,r,r,
                r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,
                c,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,c,
            ];

            basic::new(Coord {x: 54, y: 20}, sprite).unwrap()
        };

        let slot_frame = {
            let w = Pixel::new('█', Rgb{r: 255, g: 255, b: 255}, Rgb{r: 0, g: 0, b: 0});
            let b = Pixel::new('█', Rgb{r: 0, g: 0, b: 0}, Rgb{r: 0, g: 0, b: 0});
            let r = Pixel::new('█', Rgb{r: 255, g: 0, b: 0}, Rgb{r: 0, g: 0, b: 0});
            let g = Pixel::new('█', Rgb{r: 40, g: 40, b: 40}, Rgb{r: 0, g: 0, b: 0});

            let sprite = vec![
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                w,g,g,g,g,g,g,g,g,g,g,g,g,g,g,w,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                w,b,b,b,w,w,w,b,b,w,w,b,b,b,w,w,
                w,b,w,w,b,w,b,w,w,b,w,b,w,w,b,w,
                w,b,w,w,b,w,b,w,w,b,w,b,w,w,b,w,
                w,b,b,b,w,w,b,b,b,b,w,b,b,b,w,w,
                w,b,w,w,b,w,b,w,w,b,w,b,w,w,b,w,
                w,b,w,w,b,w,b,w,w,b,w,b,w,w,b,w,
                w,b,b,b,w,w,b,w,w,b,w,b,w,w,b,w,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                w,g,g,g,g,g,g,g,g,g,g,g,g,g,g,w,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                b,b,b,b,b,b,b,b,b,b,b,b,b,b,b,b,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                w,w,w,b,b,b,w,w,w,w,b,b,b,w,w,w,
                w,w,b,r,r,r,b,w,w,b,r,r,r,b,w,w,
                w,b,r,r,r,r,r,b,b,r,r,r,r,r,b,w,
                w,b,r,r,r,r,r,r,r,r,r,r,r,r,b,w,
                w,b,r,r,r,r,r,r,r,r,r,r,r,r,b,w,
                w,w,b,r,r,r,r,r,r,r,r,r,r,b,w,w,
                w,w,w,b,r,r,r,r,r,r,r,r,b,w,w,w,
                w,w,w,w,b,r,r,r,r,r,r,b,w,w,w,w,
                w,w,w,w,w,b,r,r,r,r,b,w,w,w,w,w,
                w,w,w,w,w,w,b,r,r,b,w,w,w,w,w,w,
                w,w,w,w,w,w,w,b,b,w,w,w,w,w,w,w,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                b,b,b,b,b,b,b,b,b,b,b,b,b,b,b,b,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                w,w,w,w,w,w,b,b,b,b,w,w,w,w,w,w,
                w,w,w,w,w,b,g,g,g,g,b,w,w,w,w,w,
                w,w,w,w,b,g,g,g,g,g,g,b,w,w,w,w,
                w,w,w,w,b,g,g,g,g,g,g,b,w,w,w,w,
                w,w,w,b,b,b,g,g,g,g,b,b,b,w,w,w,
                w,w,b,g,g,g,g,g,g,g,g,g,g,b,w,w,
                w,b,g,g,g,g,g,g,g,g,g,g,g,g,b,w,
                w,b,g,g,g,g,g,g,g,g,g,g,g,g,b,w,
                w,b,g,g,g,g,g,b,b,g,g,g,g,g,b,w,
                w,w,b,g,g,g,b,b,b,b,g,g,g,b,w,w,
                w,w,w,b,b,b,w,b,b,w,b,b,b,w,w,w,
                w,w,w,w,w,w,w,b,b,w,w,w,w,w,w,w,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                b,b,b,b,b,b,b,b,b,b,b,b,b,b,b,b,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                w,w,w,w,w,w,w,b,b,w,w,w,w,w,w,w,
                w,w,w,w,w,w,b,r,r,b,w,w,w,w,w,w,
                w,w,w,w,w,b,r,r,r,r,b,w,w,w,w,w,
                w,w,w,w,b,r,r,r,r,r,r,b,w,w,w,w,
                w,w,w,b,r,r,r,r,r,r,r,r,b,w,w,w,
                w,w,b,r,r,r,r,r,r,r,r,r,r,b,w,w,
                w,w,w,b,r,r,r,r,r,r,r,r,b,w,w,w,
                w,w,w,w,b,r,r,r,r,r,r,b,w,w,w,w,
                w,w,w,w,w,b,r,r,r,r,b,w,w,w,w,w,
                w,w,w,w,w,w,b,r,r,b,w,w,w,w,w,w,
                w,w,w,w,w,w,w,b,b,w,w,w,w,w,w,w,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                b,b,b,b,b,b,b,b,b,b,b,b,b,b,b,b,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                w,w,w,w,w,w,w,b,b,w,w,w,w,w,w,w,
                w,w,w,w,w,w,b,g,g,b,w,w,w,w,w,w,
                w,w,w,w,w,b,g,g,g,g,b,w,w,w,w,w,
                w,w,w,w,b,g,g,g,g,g,g,b,w,w,w,w,
                w,w,w,b,g,g,g,g,g,g,g,g,b,w,w,w,
                w,w,b,g,g,g,g,g,g,g,g,g,g,b,w,w,
                w,b,g,g,g,g,g,g,g,g,g,g,g,g,b,w,
                w,b,g,g,g,g,g,g,g,g,g,g,g,g,b,w,
                w,b,g,g,g,g,g,b,b,g,g,g,g,g,b,w,
                w,w,b,g,g,g,b,b,b,b,g,g,g,b,w,w,
                w,w,w,b,b,b,w,b,b,w,b,b,b,w,w,w,
                w,w,w,w,w,w,w,b,b,w,w,w,w,w,w,w,
                w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,
                b,b,b,b,b,b,b,b,b,b,b,b,b,b,b,b,
            ];

            basic::new(Coord {x: 16, y: 80}, sprite).unwrap()
        };

        let machine = position::craft().size(Coord {x: 54, y: 20}).enabled(false).update(MachineUpdate{}).done();
        let slot1 = position::craft().size(Coord {x: 16, y: 16}).frame_size(Coord {x: 16, y: 80}).enabled(false).update(
            SlotUpdate{
                machine: machine.clone(),
                num: 0,
            }).done();
        let slot2 = position::craft().size(Coord {x: 16, y: 16}).frame_size(Coord {x: 16, y: 80}).enabled(false).update(
            SlotUpdate{
                machine: machine.clone(),
                num: 1,
            }).done();
        let slot3 = position::craft().size(Coord {x: 16, y: 16}).frame_size(Coord {x: 16, y: 80}).enabled(false).update(
            SlotUpdate{
                machine: machine.clone(),
                num: 2,
            }).done();

        {
            let mut layout = manager.layout.borrow_mut();

            layout.objects.push(
                Object {
                    pos: machine.clone(),
                    frame: machine_frame,
                }
            );
            layout.objects.push(
                Object {
                    pos: slot1.clone(),
                    frame: slot_frame.clone(),
                }
            );
            layout.objects.push(
                Object {
                    pos: slot2.clone(),
                    frame: slot_frame.clone(),
                }
            );
            layout.objects.push(
                Object {
                    pos: slot3.clone(),
                    frame: slot_frame,
                }
            );
        }

        SlotMachine {
            machine,
            slot1,
            slot2,
            slot3
        }
    }

    pub fn main(&mut self, manager: &mut LayoutManager) {
        self.machine.borrow_mut().data.enabled = true;
        self.slot1.borrow_mut().data.enabled = true;
        self.slot2.borrow_mut().data.enabled = true;
        self.slot3.borrow_mut().data.enabled = true;

        let mut inputs = Vec::new();

        'out: loop {
            manager.draw().unwrap();

            manager.fps_input(&mut inputs);

            for input in &inputs {
                if let Input::KeyBoard(e) = input {
                    use crossterm::event::KeyCode::*;

                    match e.code {
                        Esc => {
                            break 'out
                        }
                        Enter => {
                            self.spin(manager);
                            break
                        }
                        _ => {}
                    }
                }
            }
        }

        self.machine.borrow_mut().data.enabled = false;
        self.slot1.borrow_mut().data.enabled = false;
        self.slot2.borrow_mut().data.enabled = false;
        self.slot3.borrow_mut().data.enabled = false;
    }

    fn spin(&mut self, manager: &mut LayoutManager) {
        // get some kind of random numbers.
        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() % 1000;
        let mut digits = vec! [(time / 100) as usize + 1];
        let time = time  % 100;
        digits.push((time / 10) as usize + 1);
        digits.push((time % 10) as usize + 1);

        digits.sort();
        let mut inputs = Vec::new();

        for i in 0..digits[2] {
            for _ in 0..16 {
                self.slot3.borrow_mut().inc_offset(Coord {x: 0, y: 1});

                if i < digits[1] {
                    self.slot2.borrow_mut().inc_offset(Coord {x: 0, y: 1});
                }
                if i < digits[0] {
                    self.slot1.borrow_mut().inc_offset(Coord {x: 0, y: 1});
                }

                manager.draw().unwrap();
                manager.fps_input(&mut inputs);
            }
        }

    }
}