
use frames::*;
use frame_types::*;
use crossterm::ExecutableCommand;
use crossterm::event::{poll, read, Event};
use crossterm::terminal::{self, EnterAlternateScreen};
use std::io;
use std::{thread, time};

fn main() {
    io::stdout().execute(terminal::SetTitle("Frames Demo!")).unwrap();

    let background_data = {
        let s = Pixel::new('*', Color::Rgb{r: 255, g: 255, b: 255}, Color::Rgb{r: 0, g: 0, b: 0});
        let b = Pixel::new('.', Color::Rgb{r: 255, g: 255, b: 255}, Color::Rgb{r: 0, g: 0, b: 0});
        let x = Pixel::new('x', Color::Rgb{r: 255, g: 255, b: 255}, Color::Rgb{r: 0, g: 0, b: 0});
        let n = Pixel::Clear;

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

        Basic::new(Coord{x: 14, y: 14}, sprite)
    }.unwrap();

    let planet_data = {
        let y = Pixel::new('█', Color::Rgb{r: 224, g: 167, b: 43}, Color::Rgb{r: 0, g: 0, b: 0});
        let w = Pixel::new('█', Color::Rgb{r: 230, g: 230, b: 230}, Color::Rgb{r: 0, g: 0, b: 0});

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

        Mask::new(Basic::new(Coord{x: 42, y: 21}, sprite).unwrap(), Pixel::Clear, mask_rules::Circle::new(), false)
    };

    let moon_data = {
        let b = Pixel::new('█', Color::Rgb{r: 140, g: 140, b: 140}, Color::Rgb{r: 140, g: 140, b: 140});

        Mask::new(Fill::new(b), Pixel::Clear, mask_rules::Circle::new(), false)
    };

    let mut manager = Manager::new().unwrap();

    let background = Object::new_basic(background_data.clone(), manager.get_size());
    background.borrow_mut().size_update = Some(Box::new(BackGroundUpdate{}));

    let planet = Object::new_basic(planet_data.clone(), Coord{x: 21, y: 21});
    planet.borrow_mut().size_update = Some(Box::new(PlanetUpdate{}));

    let moon = Object::new_basic(moon_data.clone(), Coord{x: 10, y: 10});
    moon.borrow_mut().size_update = Some(Box::new(MoonUpdate{planet: planet.clone()}));

    manager.objects.push(background.clone());
    manager.objects.push(planet.clone());
    manager.objects.push(moon.clone());
    
    manager.add_task(Task::UpdateAll);

    io::stdout().execute(EnterAlternateScreen).unwrap();

    loop {
        while poll(time::Duration::from_millis(1)).unwrap() {
            if let Event::Resize(x, y) = read().unwrap() {
                manager.resize(x, y);
            }
        }

        planet.borrow_mut().inc_offset(&Coord{ x: 1, y: 0 });
        manager.add_task(planet.borrow().update());

        manager.draw().unwrap();

        thread::sleep(time::Duration::from_millis(100));
    }

    //io::stdout().execute(LeaveAlternateScreen).unwrap();
}

struct PlanetUpdate {}

impl SizeUpdate for PlanetUpdate {
    fn size_update(&mut self, new_size: &Coord, pos: &mut Coord, size: &mut Coord, _offset: &mut Coord, _enabled: &mut bool){
        let temp = Coord{x: (new_size.x / 6), y: ((new_size.y / 5) * 3)};
        *pos = temp - (*size / Coord{ x: 2, y: 2 });
    }
}

struct MoonUpdate {
    planet: Rc<RefCell<Object>>,
}

impl SizeUpdate for MoonUpdate {
    fn size_update(&mut self, _new_size: &Coord, pos: &mut Coord, size: &mut Coord, _offset: &mut Coord, _enabled: &mut bool){
        let temp = self.planet.borrow().pos + Coord{x: 25, y: -5};
        *pos = temp - (*size / Coord{ x: 2, y: 2 });
    }
}

struct BackGroundUpdate {}

impl SizeUpdate for BackGroundUpdate {
    fn size_update(&mut self, new_size: &Coord, _pos: &mut Coord, size: &mut Coord, _offset: &mut Coord, _enabled: &mut bool){
        *size = *new_size;
    }
}