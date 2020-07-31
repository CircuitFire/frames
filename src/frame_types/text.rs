use crate::shared::*;

use std::collections::VecDeque;

pub struct Text {
    size: Option<usize>,
    fill: PixleData,
    entries: VecDeque<Entry>,
}

impl Frame for Text {

    fn size(&self) -> Coord {
        let size = if let Some(size) = self.size {
            size as i32
        }
        else { 0 };

        Coord {
            x: self.entries.len() as i32,
            y: size,
        }
    }

    fn get_draw_data(&self, area: &Vec<Drawsegment>, offset: Coord, size: Coord) -> Vec<DrawData> {
        let mut datasegments: Vec<DrawData> = Vec::with_capacity(area.len());
        let mut entry_iter = EntryIter::new(&self.entries, self.fill, offset.y as usize, offset.x as usize, size.x as usize);

        for seg in area {
            datasegments.push(entry_iter.get_drawdata(seg));
        }
        
        datasegments
    }

}

impl Text {
    pub fn new(fill: PixleData) -> Rc<RefCell<Text>> {
        Rc::new(RefCell::new(
            Text {
                size: None,
                fill: fill,
                entries: VecDeque::new(),
            }
        ))
    }

    pub fn new_sized(fill: PixleData, total_entry_num: usize) -> Rc<RefCell<Text>> {
        Rc::new(RefCell::new(
            Text {
                size: Some(total_entry_num),
                fill: fill,
                entries: VecDeque::new(),
            }
        ))
    }

    pub fn add_entry(&mut self, text: &str) {
        self.add_entry_color(text, self.fill.fg, self.fill.bg);
    }

    pub fn add_entry_color(&mut self, text: &str, fg: Color, bg: Color) {
        let entry = Entry::new(text, fg, bg);

        if let Some(size) = self.size {
            if size == self.entries.len() { self.entries.pop_front(); }
        }
        
        self.entries.push_back(entry);
    }

    pub fn entrys_len(&self) -> usize {
        self.entries.len()
    }

    pub fn set_entry_text(&mut self, index: usize, text: &str) {
        self.entries[index].set_text(text);
    }

    pub fn get_entry_text(&self, index: usize) -> &str {
        &self.entries[index].text
    }

    pub fn append_entry(&mut self, index: usize, text: &str) {
        self.entries[index].append(text);
    }

    pub fn set_entry_fg(&mut self, index: usize, color: Color) {
        self.entries[index].fg = color;
    }

    pub fn get_entry_fg(&self, index: usize) -> Color {
        self.entries[index].fg
    }

    pub fn set_entry_bg(&mut self, index: usize, color: Color) {
        self.entries[index].bg = color;
    }

    pub fn get_entry_bg(&self, index: usize) -> Color {
        self.entries[index].bg
    }
}

struct Entry {
    text: String,
    fg: Color,
    bg: Color,
}

impl Entry {
    fn new(text: &str, fg: Color, bg: Color) -> Self {
        Entry {
            text: sanitize(String::from(text)),
            fg: fg,
            bg: bg,
        }
    }

    fn set_text(&mut self, text: &str) {
        self.text = sanitize(String::from(text));
    }

    fn append(&mut self, text: &str) {
        self.text.push_str(&sanitize(String::from(text)));
    }
}

fn sanitize(mut text: String) -> String {
    replace(&mut text, "\r", "");
    replace(&mut text, "\t", "    ");
    text
}

fn replace(text: &mut String, search: &str, replace: &str) {
    if text.contains(search) {
        *text = text.replace(search, replace);
    }
}

struct EntryIter<'a> {
    entries: &'a VecDeque<Entry>,
    char_iter: Option<CharIter<'a>>,
    fill: PixleData,
    index: usize,
    cur_line: usize,
    width: usize,
}

impl<'a> EntryIter<'a> {
    fn new(entries: &'a VecDeque<Entry>, fill: PixleData, index_skip: usize, line_skip: usize, width: usize) -> EntryIter<'a> {
        let mut entry_iter = EntryIter {
            entries: entries,
            char_iter: None,
            fill: fill,
            index: index_skip,
            cur_line: 0,
            width: width,
        };

        entry_iter.go_to_line(line_skip);
        
        entry_iter
    }

    fn go_to_line(&mut self, line: usize) {
        loop {
            if let Some(iter) = &mut self.char_iter {
                let jump = if line > self.cur_line { line - self.cur_line }
                else { 0 };

                let left = iter.jump_line(jump, self.width);

                if left > 0 {
                    self.char_iter = None;
                    self.index += 1;
                }

                self.cur_line += line - left;
            }
            else{
                if self.index < self.entries.len() {
                    self.char_iter = Some(CharIter::new(&self.entries[self.index], self.fill.character));
                }
                else {
                    self.cur_line = line;
                }
            }
            if self.cur_line >= line { break }
        }
    }

    fn get_drawdata(&mut self, seg: &Drawsegment) -> DrawData {
        let mut drawdata = DrawData::from_drawsemgnet(seg);
        self.go_to_line(seg.start.y as usize);
    
        if let Some(iter) = &mut self.char_iter {
            let mut len = seg.len;
            if iter.jump(seg.start.x as usize) != '\n' {
    
                let mut next = ' ';
                for _ in 0..seg.len {
                    next = iter.next();
                    
                    if next != '\n' {
                        len -= 1;
    
                        drawdata.data.push(Pixle::new(
                            next,
                            self.entries[self.index].fg,
                            self.entries[self.index].bg,
                        ))
                    }
                    else { break }
                }

                if next != '\n'  { iter.jump(self.width - seg.end_pos() as usize); }
            }
    
            for _ in 0..len {
                drawdata.data.push(Pixle::new(
                    self.fill.character,
                    self.entries[self.index].fg,
                    self.entries[self.index].bg,
                ))
            }
    
            if !iter.data_left() {
                self.char_iter = None;
                self.index += 1;
            }
        }
        else {
            for _ in 0..seg.len {
                drawdata.data.push(Pixle::Opaque(self.fill));
            }
        }
        

        self.cur_line += 1;
        drawdata
    }
}

struct CharIter<'a> {
    char_iter: Option<std::str::Chars<'a>>,
    fill: char,
}

impl<'a> CharIter<'a> {
    fn new(some_entry: &'a Entry, fill: char) -> CharIter<'a> {

        CharIter {
            char_iter: Some(some_entry.text.chars()),
            fill: fill,
        }
    }

    fn next(&mut self) -> char {

        let mut character = self.fill;

        if let Some(iter) = &mut self.char_iter {

            if let Some(pix) = iter.next() {
                character = pix;
            }
            else{
                self.char_iter = None;
            }
            
        }

        character
    }

    fn jump(&mut self, chars: usize) -> char {
        let mut last = ' ';

        for _ in 0..chars {
            last = self.next();
            if last == '\n' { break }
        }

        last
    }

    fn jump_line(&mut self, mut lines: usize, width: usize) -> usize {
        for _ in 0..lines {
            if self.data_left() {
                self.jump(width);
                lines -= 1;
            }
            else { break }
        }
        lines
    }

    fn data_left(&self) -> bool {
        if let Some(_) = self.char_iter {
            true
        }
        else { false }
    }
}