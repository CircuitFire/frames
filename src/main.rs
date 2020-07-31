
use frames::*;
use frame_types::*;
use crossterm::ExecutableCommand;
use crossterm::event::{poll, read, Event};
use crossterm::terminal;
use std::io;
use std::{thread, time};

fn main() {
    io::stdout().execute(terminal::SetTitle("Frames Demo!")).unwrap();

    let size = {
        let (x, y) = crossterm::terminal::size().unwrap();
        Coord { x: x as i32, y: y as i32 }
    };

    let background_data = {
        let s = Pixle::new('*', Color::Rgb{r: 255, g: 255, b: 255}, Color::Rgb{r: 0, g: 0, b: 0});
        let b = Pixle::new('.', Color::Rgb{r: 255, g: 255, b: 255}, Color::Rgb{r: 0, g: 0, b: 0});
        let x = Pixle::new('x', Color::Rgb{r: 255, g: 255, b: 255}, Color::Rgb{r: 0, g: 0, b: 0});
        let n = Pixle::Clear;

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
        let y = Pixle::new_basic('█', Color::Rgb{r: 224, g: 167, b: 43});
        let w = Pixle::new_basic('█', Color::Rgb{r: 230, g: 230, b: 230});

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

        frame_types::Mask::new(Basic::new(Coord{x: 42, y: 21}, sprite).unwrap(), Pixle::Clear, Circle::new())
    };

    let moon_data = {
        let b = Pixle::new('█', Color::Rgb{r: 140, g: 140, b: 140}, Color::Rgb{r: 140, g: 140, b: 140});

        frame_types::Mask::new(frame_types::Fill::new(b), Pixle::Clear, Circle::new())
    };

    let mut manager = frames::Manager::new(size, &Pixle::new('█', Color::Rgb{r: 0, g: 0, b: 0}, Color::Rgb{r: 0, g: 0, b: 0})).unwrap();

    let background = Object::new(background_data.clone(), Coord {x: 0, y: 0}, size, Coord {x: 0, y: 0}, false, false, false);

    let planet = Object::new(planet_data.clone(), Coord {x: 0, y: 0}, Coord {x: 21, y: 21}, Coord {x: 0, y: 0}, false, false, false);
    planet.borrow_mut().set_center(&planet_pos(&size));

    let moon = Object::new(moon_data.clone(),Coord {x: 0, y: 0}, Coord {x: 7, y: 7}, Coord {x: 0, y: 0}, false, false, false);
    moon.borrow_mut().set_center(&(planet.borrow().get_pos() + Coord{x: 25, y: -5}));

    manager.objects().push(background.clone());
    manager.objects().push(planet.clone());
    manager.objects().push(moon.clone());
    

    manager.add_task(frames::Task::UpdateAll);
    
    loop {
        manager.draw().unwrap();

        if poll(time::Duration::from_millis(0)).unwrap(){
            match read().unwrap() {
                Event::Resize(width, height) => {
                    let size = Coord{x: width as i32, y: height as i32};
                    manager.set_size(&size);
                    background.borrow_mut().set_size(&size);
                    planet.borrow_mut().set_center(&planet_pos(&size));
                    moon.borrow_mut().set_center(&(planet.borrow().get_pos() + Coord{x: 25, y: -5}));
                }
                _ => ()
            }
        }

        planet.borrow_mut().inc_offset(&Coord{ x: 1, y: 0 });
        manager.add_task(planet.borrow().update());

        thread::sleep(time::Duration::from_millis(100));
    }
}

fn planet_pos(screen_size: &Coord) -> Coord {
    let mut new = *screen_size;

    new.x = new.x / 6; 
    new.y = (new.y / 5) * 3;

    new
}