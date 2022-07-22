use frames::*;
use frame_types::*;
use layout::Object;
use modifiers::*;
use position::update_types::*;
use prelude::*;

use std::io;
use std::time;

use crossterm::ExecutableCommand;
use crossterm::terminal;

use Color::Rgb;

fn main() {
    io::stdout().execute(terminal::SetTitle("Frames Demo!")).unwrap();

    let background_data = {
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

    let planet_data = {
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

    let moon_data = {
        let b = Pixel::new('█', Rgb{r: 140, g: 140, b: 140}, Rgb{r: 140, g: 140, b: 140});

        with_modifier::new(
            fill::new(b),
            circle_mask::new(false)
        )
    };

    let mut manager = LayoutManager::new().unwrap();
    let planet = position::craft().size(Coord{x: 21, y: 21}).update(PlanetUpdate{}).done();

    {
        let mut layout = manager.layout.borrow_mut();

        layout.objects.push(Object{
            frame: background_data,
            pos: position::craft().update(MatchSize{}).done()
        });
        layout.objects.push(Object{
            frame: planet_data,
            pos: planet.clone()
        });
        layout.objects.push(Object{
            frame: moon_data,
            pos: position::craft().size(Coord{x: 10, y: 10}).update(MoonUpdate{planet: planet.clone()}).done()
        });
    }

    let mut inputs = Vec::new();

    loop {
        manager.draw().unwrap();

        planet.borrow_mut().inc_offset(&Coord{ x: 1, y: 0 });

        manager.inputs_over_duration(&mut inputs,time::Duration::from_millis(10));
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