use crate::prelude::*;

use std::collections::VecDeque;
use std::str::from_utf8;

pub enum Indent {
    Normal(usize),
    Hanging(usize),
}

impl Indent {
    pub fn normal(&self) -> usize {
        match self {
            Indent::Normal(x)  => *x,
            Indent::Hanging(_) =>  0,
        }
    }

    pub fn hanging(&self) -> usize {
        match self {
            Indent::Normal(_)  =>  0,
            Indent::Hanging(x) => *x,
        }
    }
}

pub type Text = Rc<RefCell<IText>>;

pub fn new() -> Text {
    Rc::new(RefCell::new(IText::new()))
}

/// Contains a queue of text entries that each have their own color
/// ## Functions
/// - new
/// - new_sized
/// 
/// ## Methods
/// - push
/// - push_color
/// - len
/// - set_text
/// - get_text
/// - append_entry
/// - set_fg
/// - get_fg
/// - set_bg
/// - get_bg
/// - insert
/// - insert_color
/// - remove
/// - clear
/// - truncate
pub struct IText {
    pub tab_spaces: String,
    ///Positive indent is hanging, negative is normal indent.
    pub indent:     Indent,
    pub default:    PixelData,
    entries:        VecDeque<Entry>,
    pub max:        Option<usize>,
}

impl IFrame for IText {
    fn get_draw_data(&self, screenbuf: &mut ScreenBuf, offset: Coord, size: Coord) {
        let mut data = EntryIter::new(&self.entries, self.default.character, offset, size, &self.indent);

        for pos in screenbuf.draw_to() {
            if let Some(pixle) = data.get(pos) {
                screenbuf.set(pos, pixle);
            }
            else {
                screenbuf.set(pos, Pixel::Opaque(self.default));
            }
        }

    }
}

impl IText {
    pub fn new() -> Self {
        IText {
            tab_spaces: "    ".to_string(),
            indent:     Indent::Hanging(0),
            default:    PixelData {
                character: ' ',
                fg:        Color::Rgb{r: 255, g: 255, b: 255},
                bg:        Color::Rgb{r:   0, g:   0, b:   0},
            },
            entries:    VecDeque::new(),
            max:        None,
        }
    }

    pub fn push(&mut self, text: String) {
        self.push_color(text, ColorSet{fg: self.default.fg, bg: self.default.bg});
    }

    pub fn push_color(&mut self, text: String, colors: ColorSet) {
        let entry = Entry::new(text, colors);

        if let Some(size) = self.max {
            if size == self.entries.len() { self.entries.pop_front(); }
        }
        
        self.entries.push_back(entry);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn set_text(&mut self, index: usize, text: String) {
        self.entries[index].set_text(text);
    }

    pub fn get_text(&self, index: usize) -> &str {
        &self.entries[index].text
    }

    pub fn append_entry(&mut self, index: usize, text: String) {
        self.entries[index].append(text);
    }

    pub fn set_fg(&mut self, index: usize, color: Color) {
        self.entries[index].main_colors.fg = color;
    }

    pub fn get_fg(&self, index: usize) -> Color {
        self.entries[index].main_colors.fg
    }

    pub fn set_bg(&mut self, index: usize, color: Color) {
        self.entries[index].main_colors.bg = color;
    }

    pub fn get_bg(&self, index: usize) -> Color {
        self.entries[index].main_colors.bg
    }

    pub fn insert(&mut self, index: usize, text: String) {
        self.insert_color(index, text, self.default.fg, self.default.bg);
    }

    pub fn insert_color(&mut self, index: usize, text: String, fg: Color, bg: Color) {
        let entry = Entry::new(text, ColorSet{fg: fg, bg: bg});

        if let Some(size) = self.max {
            if size == self.entries.len() { self.entries.pop_front(); }
        }
        
        self.entries.insert(index, entry);
    }

    pub fn remove(&mut self, index: usize) {
        self.entries.remove(index);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn truncate(&mut self, len: usize) {
        self.entries.truncate(len);
    }

    fn sanitize(&self, mut text: String) -> String {
        replace(&mut text, "\r", "");
        replace(&mut text, "\t", &self.tab_spaces);
        text
    }
}

#[derive(Copy, Clone)]
struct ColorSlice {
    start:  usize,
    end:    usize,
    colors: ColorSet,
}

struct Entry {
    text:          String,
    len:           usize,
    new_lines:     usize,
    main_colors:   ColorSet,
    colors_slices: Vec<ColorSlice>,
}

impl Entry {
    fn new(mut text: String, color_set: ColorSet) -> Self {
        Entry {
            len:           text.chars().count(),
            new_lines:     text.matches("\n").count(),
            text:          text,
            main_colors:   color_set,
            colors_slices: Vec::new(),
        }
    }

    fn set_text(&mut self, text: String) {
        self.text      = text;
        self.len       = self.text.chars().count();
        self.new_lines = self.text.matches("\n").count();
    }

    fn append(&mut self, text: String) {
        self.text.push_str(&text);
    }

    fn height(&self, width: usize, indent: &Indent) -> usize {
        let mut hight = self.new_lines + 1;
        let mut len = self.len - self.new_lines;
        
        //length of first line
        let first_len = width - indent.normal();
        if len > first_len {
            hight += 1;
            len -= first_len;
        }
        else { return hight }

        //length of all other lines.
        let other_len = width - indent.hanging();
        hight + (len / other_len)
    }

    fn get_color(&self, pos: usize) -> ColorSet {
        for slice in &self.colors_slices {
            if pos >= slice.start && pos < slice.end {
                return slice.colors
            }
            if pos < slice.start { break }
        }

        self.main_colors
    }
}

fn replace(text: &mut String, search: &str, replace: &str) {
    if text.contains(search) {
        *text = text.replace(search, replace);
    }
}

//need to check if this is faster then copying the string into char vec.
struct CharIter<'a> {
    entries:  &'a VecDeque<Entry>,
    cur_entry:    usize,
    byte_index:   usize,
    string_index: usize,
}

impl<'a> CharIter<'a> {
    fn new(entries: &'a VecDeque<Entry>, offset: usize) -> CharIter<'a> {
        CharIter {
            entries: entries,
            cur_entry: offset,
            byte_index: 0,
            string_index: 0,
        }
    }

    ///trying to iterate over all of the chars of the current entry without having to scan through each char each time.
    fn next(&mut self) -> Option<char> {
        if let Some(entry) = self.entries.get(self.cur_entry) {
            //only check for chars if there a some left in the string.
            if self.string_index < entry.len {
                //get a byte slice of the string.
                let mut byte_end = self.byte_index + 1;
                loop {
                    //if the byte slice contains a full char return or extend the slice and check until it does.
                    if let Ok(c) = from_utf8(&entry.text.as_bytes()[self.byte_index..byte_end]) {
                        self.byte_index = byte_end;
                        self.string_index += 1;

                        return Some(c.chars().next().unwrap())
                    }
                    else {
                        byte_end += 1;
                    }
                }
                
            }
        }
        
        None
    }

    fn next_pixel(&mut self) -> Option<PixelData> {
        if let Some(c) = self.next() {
            return Some(PixelData::new_color_set(
                c,
                self.entries[self.cur_entry].get_color(self.string_index - 1)
            ))
        }

        None
    }

    fn more_chars(&self) -> bool {
        if let Some(entry) = self.entries.get(self.cur_entry) {
            if self.string_index < entry.len {
                return true
            }
        }

        false
    }

    fn cur_entry(&mut self) -> Option<&Entry> {
        self.entries.get(self.cur_entry)
    }

    fn next_entry(&mut self) {
        self.cur_entry += 1;
        self.byte_index = 0;
        self.string_index = 0;
    }
}

struct EntryIter<'a> {
    char_iter:     CharIter<'a>,
    entry_line:    usize,
    default_char:  char,
    next_pos:      Coord,
    width:         i32,
    indent:    &'a Indent,
    line_finished: bool,
}

impl<'a> EntryIter<'a> {
    fn new(entries: &'a VecDeque<Entry>, default_char: char, offset: Coord, size: Coord, indent: &'a Indent) -> EntryIter<'a> {
        let mut entry_iter = EntryIter {
            char_iter:     CharIter::new(entries, offset.y as usize),
            entry_line:    0,
            default_char:  default_char,
            next_pos:      Coord{x: 0, y: 0},
            width:         size.x,
            indent:        indent,
            line_finished: false,
        };

        entry_iter.skip(offset.x as usize);
        
        entry_iter
    }


    fn skip(&mut self, mut skip: usize) {
        while skip > 0 {
            if let Some(entry) = self.char_iter.cur_entry() {
                let height = entry.height(self.width as usize, &self.indent);

                if height <= skip {
                    skip -= height;
                    self.char_iter.cur_entry += 1;
                }
                else {
                    //skipping into somewhere inside an entry.
                    self.go_to(Coord{y: skip as i32, x: 0});
                    self.next_pos = Coord{x: 0, y: 0}; //reset current position because it is changed in go_to.
                    return
                }
            }
            else {
                //skipped all of the entries.
                return
            }
        }
    }

    ///when moving to a new line is when to check if it is time to move to the next entry.
    fn check_inc_entry(&mut self) {
        if self.char_iter.more_chars() {
            //record what line we are on local to the entry for indents.
            self.entry_line += 1;
        }
        else {
            //move to the next entry
            self.char_iter.next_entry();
            self.entry_line = 0;
        }
    }

    fn inc_next_pos(&mut self) {
        self.next_pos.x += 1;

        if self.next_pos.x >= self.width {
            self.next_pos.x  = 0;
            self.next_pos.y += 1;

            self.line_finished = false;

            self.check_inc_entry()
        }
    }

    fn skip_line(&mut self, pos: Coord) {
        if self.next_pos.y == pos.y {
            self.next_pos.x = pos.x - 1;
        }
        else {
            self.next_pos.x = self.width;
        }
    }

    fn go_to(&mut self, pos: Coord) {
        while self.next_pos != pos {
            if let Some(d) = self.char_iter.next_pixel() {
                if d.character == '\n' {
                    self.skip_line(pos);
                }
            }
            else {
                self.skip_line(pos);
            }

            self.inc_next_pos();
        }
    }

    fn get(&mut self, pos: Coord) -> Option<Pixel> {
        if self.next_pos != pos {
            self.go_to(pos)
        }

        let mut data;

        if let Some(entry) = self.char_iter.cur_entry() {
            data = PixelData::new(self.default_char, entry.main_colors.fg, entry.main_colors.bg);
        }
        else {
            return None
        }

        //handling new line
        if !self.line_finished
        //first line indent
        && !((self.entry_line == 0) && (self.next_pos.x < self.indent.normal() as i32))
        //other line indent
        && !((self.entry_line != 0) && (self.next_pos.x < self.indent.hanging() as i32)) {

            if let Some(d) = self.char_iter.next_pixel() {
                if d.character == '\n' {
                    self.line_finished = true;
                }
                else {
                    data = d;
                }
            }
        }

        self.inc_next_pos();

        Some(Pixel::Opaque(data))
    }
}