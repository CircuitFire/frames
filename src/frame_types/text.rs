use crate::prelude::*;
use crate::ColorString;

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
    wrap(IText::new())
}

/// Contains a queue of text entries that each have their own color
/// ## Functions
/// - new
/// 
/// ## Methods
pub struct IText {
    pub tab_spaces: usize,
    ///Positive indent is hanging, negative is normal indent.
    pub indent:     Indent,
    pub default:    PixelData,
    pub entries:    VecDeque<Entry>,
}

impl IFrame for IText {
    fn get_draw_data(&self, screenbuf: &mut ScreenBuf, offset: Coord, size: Coord) {
        let mut data = EntryIter::new(&self.entries, self.default, offset, size, &self.indent, self.tab_spaces);

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
            tab_spaces: 4,
            indent:     Indent::Hanging(0),
            default:    PixelData {
                character: ' ',
                fg:        Color::Rgb{r: 255, g: 255, b: 255},
                bg:        Color::Rgb{r:   0, g:   0, b:   0},
            },
            entries:    VecDeque::new(),
        }
    }
}

pub struct Entry {
    text:       ColorString,
    len:        usize,
    new_lines:  usize,
    tabs:       usize,
    pub colors: Option<ColorSet>,
}

impl Entry {
    pub fn new<T: Into<ColorString>>(text: T) -> Self {
        let text: ColorString = text.into();

        Entry {
            len:       text.string.chars().count(),
            new_lines: text.string.matches("\n").count(),
            tabs:      text.string.matches("\t").count(),
            text,
            colors:    None,
        }
    }

    pub fn new_color<T: Into<ColorString>>(text: T, colors: ColorSet) -> Self {
        let text: ColorString = text.into();

        Entry {
            len:       text.string.chars().count(),
            new_lines: text.string.matches("\n").count(),
            tabs:      text.string.matches("\t").count(),
            text,
            colors:    Some(colors),
        }
    }

    pub fn set_text<T: Into<ColorString>>(&mut self, text: T) {
        let text: ColorString = text.into();

        self.text      = text;
        self.len       = self.text.string.chars().count();
        self.new_lines = self.text.string.matches("\n").count();
        self.tabs      = self.text.string.matches("\t").count();
    }

    fn height(&self, width: usize, indent: &Indent, tab_len: usize) -> usize {
        let mut hight = self.new_lines + 1;
        let mut len = self.len + (self.tabs * tab_len) - (self.new_lines + self.tabs);
        
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

    fn get_color(&self, pos: usize) -> Option<ColorSet> {
        if let Some(color) = self.text.get_color(pos) {
            return Some(color)
        }
        else {
            if let Some(color) = self.colors {
                return Some(color)
            }
            else {
                return None
            }
        }
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
                    if let Ok(c) = from_utf8(&entry.text.string.as_bytes()[self.byte_index..byte_end]) {
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

    fn next_pixel(&mut self, default: PixelData) -> Option<PixelData> {
        if let Some(c) = self.next() {
            let color = if let Some(color) = self.entries[self.cur_entry].get_color(self.string_index - 1) {
                color
            }
            else {
                default.get_color_set()
            };

            return Some(PixelData::new_color_set(
                c,
                color
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

struct TabData {
    pub fg:    Color,
    pub bg:    Color,
    pub count: usize,
}

struct EntryIter<'a> {
    char_iter:  CharIter<'a>,
    entry_line: usize,
    default:    PixelData,
    next_pos:   Coord,
    width:      i32,
    indent: &'a Indent,
    new_line:   bool,
    tab_len:    usize,
    cur_tab:    Option<TabData>,
}

impl<'a> EntryIter<'a> {
    fn new(entries: &'a VecDeque<Entry>, default: PixelData, offset: Coord, size: Coord, indent: &'a Indent, tab_len: usize) -> EntryIter<'a> {
        let mut entry_iter = EntryIter {
            char_iter:    CharIter::new(entries, offset.y as usize),
            entry_line:   0,
            default,
            next_pos:     Coord{x: 0, y: 0},
            width:        size.x,
            indent,
            new_line:     false,
            tab_len,
            cur_tab:      None,
        };

        entry_iter.skip(offset.x as usize);
        
        entry_iter
    }


    fn skip(&mut self, mut skip: usize) {
        while skip > 0 {
            if let Some(entry) = self.char_iter.cur_entry() {
                let height = entry.height(self.width as usize, &self.indent, self.tab_len);

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

    fn inc_next_pos(&mut self) {
        self.next_pos.x += 1;

        if self.next_pos.x >= self.width {
            self.next_pos.x  = 0;
            self.next_pos.y += 1;

            //record what line of the current entry we are on for indents.
            self.entry_line += 1;

            if self.new_line {
                self.new_line = false;
            }
            else {
                //move to the next entry
                if !self.char_iter.more_chars() {
                    self.char_iter.next_entry();
                    self.entry_line = 0;
                }
            }
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
            if let Some(d) = self.char_iter.next_pixel(self.default) {
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

    fn next_pixel(&mut self) -> Option<PixelData> {
        //handling in progress tabs.
        if let Some(tab) = self.cur_tab.as_mut() {
            let pix = PixelData {
                character: ' ',
                fg: tab.fg,
                bg: tab.bg,
            };

            tab.count -= 1;

            if tab.count == 0 {
                self.cur_tab = None;
            }

            return Some(pix)
        }

        //try to get next pixel from entry.
        loop {
            if let Some(mut d) = self.char_iter.next_pixel(self.default) {
                match d.character {
                    '\n' => {
                        self.new_line = true;
                        return None
                    }
                    '\t' => {
                        match self.tab_len {
                            0 => {}
                            1 => {
                                d.character = ' ';
                                return Some(d)
                            }
                            _ => {
                                self.cur_tab = Some(
                                    TabData {
                                        fg: d.fg,
                                        bg: d.bg,
                                        count: self.tab_len - 1,
                                    }
                                );
    
                                d.character = ' ';
                                return Some(d)
                            }
                        }
                    }
                    _ => {
                        return Some(d)
                    }
                }
            }
            else {
                return None
            }
        }
    }

    fn get(&mut self, pos: Coord) -> Option<Pixel> {
        if self.next_pos != pos {
            self.go_to(pos)
        }

        let mut data = self.default;

        if let Some(entry) = self.char_iter.cur_entry() {
            if let Some(colors) = entry.colors {
                data.fg = colors.fg;
                data.fg = colors.bg;
            }
        }
        else {
            return None
        }

        //deciding wether to pull data from the entry,
        //handling new line
        if !self.new_line
        //first line indent
        && !((self.entry_line == 0) && (self.next_pos.x < self.indent.normal() as i32))
        //other line indent
        && !((self.entry_line != 0) && (self.next_pos.x < self.indent.hanging() as i32)) {

            if let Some(d) = self.next_pixel() {
                data = d;
            }
        }

        self.inc_next_pos();

        Some(Pixel::Opaque(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_test() {
        let s = Pixel::new(' ', Color::Rgb { r: 255, g: 255, b: 255 }, Color::Rgb { r: 0, g: 0, b: 0 });

        let expected = vec![
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
        ];

        let mut buf = ScreenBuf::new(Coord{x: 10, y: 10});

        let text = new();

        text.borrow().get_draw_data(&mut buf, Coord{x: 0, y: 0}, Coord{x: 10, y: 10});

        //print_buffer(&buf);

        for (i, x) in expected.iter().enumerate() {
            assert_eq!(buf.buffer.get_flat(i), *x)
        }
    }

    #[test]
    fn spacing_test() {
        let x = Pixel::new('x', Color::Rgb { r: 255, g: 255, b: 255 }, Color::Rgb { r: 0, g: 0, b: 0 });
        let y = Pixel::new('y', Color::Rgb { r: 255, g: 255, b: 255 }, Color::Rgb { r: 0, g: 0, b: 0 });
        let s = Pixel::new(' ', Color::Rgb { r: 255, g: 255, b: 255 }, Color::Rgb { r: 0, g: 0, b: 0 });

        let expected = vec![
            x,x,x,x,x,s,s,s,s,s,
            y,y,y,y,y,y,y,y,y,y,
            x,x,x,x,x,x,x,x,x,x,
            x,x,s,s,s,s,s,s,s,s,
            y,y,y,s,s,s,s,s,s,s,
            y,y,y,y,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            x,x,x,x,x,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
        ];

        let mut buf = ScreenBuf::new(Coord{x: 10, y: 10});

        let text = new();
        
        {
            let mut temp = text.borrow_mut();

            temp.entries.push_back(Entry::new("xxxxx"));
            temp.entries.push_back(Entry::new("yyyyyyyyyy"));
            temp.entries.push_back(Entry::new("xxxxxxxxxxxx"));
            temp.entries.push_back(Entry::new("yyy\nyyyy\n"));
            temp.entries.push_back(Entry::new("xxxxx"));
        }

        text.borrow().get_draw_data(&mut buf, Coord{x: 0, y: 0}, Coord{x: 10, y: 10});

        //print_buffer(&buf);

        for (i, x) in expected.iter().enumerate() {
            assert_eq!(buf.buffer.get_flat(i), *x)
        }
    }

    #[test]
    fn indent_test1() {
        let x = Pixel::new('x', Color::Rgb { r: 255, g: 255, b: 255 }, Color::Rgb { r: 0, g: 0, b: 0 });
        let y = Pixel::new('y', Color::Rgb { r: 255, g: 255, b: 255 }, Color::Rgb { r: 0, g: 0, b: 0 });
        let s = Pixel::new(' ', Color::Rgb { r: 255, g: 255, b: 255 }, Color::Rgb { r: 0, g: 0, b: 0 });

        let expected = vec![
            s,x,x,x,x,x,s,s,s,s,
            s,y,y,y,y,y,y,y,y,y,
            y,s,s,s,s,s,s,s,s,s,
            s,x,x,x,x,x,x,x,x,x,
            x,x,x,s,s,s,s,s,s,s,
            s,y,y,y,s,s,s,s,s,s,
            y,y,y,y,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,x,x,x,x,x,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
        ];

        let mut buf = ScreenBuf::new(Coord{x: 10, y: 10});

        let text = new();
        
        {
            let mut temp = text.borrow_mut();

            temp.indent = Indent::Normal(1);

            temp.entries.push_back(Entry::new("xxxxx"));
            temp.entries.push_back(Entry::new("yyyyyyyyyy"));
            temp.entries.push_back(Entry::new("xxxxxxxxxxxx"));
            temp.entries.push_back(Entry::new("yyy\nyyyy\n"));
            temp.entries.push_back(Entry::new("xxxxx"));
        }

        text.borrow().get_draw_data(&mut buf, Coord{x: 0, y: 0}, Coord{x: 10, y: 10});

        //print_buffer(&buf);

        for (i, x) in expected.iter().enumerate() {
            assert_eq!(buf.buffer.get_flat(i), *x)
        }
    }

    #[test]
    fn indent_test2() {
        let x = Pixel::new('x', Color::Rgb { r: 255, g: 255, b: 255 }, Color::Rgb { r: 0, g: 0, b: 0 });
        let y = Pixel::new('y', Color::Rgb { r: 255, g: 255, b: 255 }, Color::Rgb { r: 0, g: 0, b: 0 });
        let s = Pixel::new(' ', Color::Rgb { r: 255, g: 255, b: 255 }, Color::Rgb { r: 0, g: 0, b: 0 });

        let expected = vec![
            x,x,x,x,x,s,s,s,s,s,
            y,y,y,y,y,y,y,y,y,y,
            x,x,x,x,x,x,x,x,x,x,
            s,x,x,s,s,s,s,s,s,s,
            y,y,y,s,s,s,s,s,s,s,
            s,y,y,y,y,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            x,x,x,x,x,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
        ];

        let mut buf = ScreenBuf::new(Coord{x: 10, y: 10});

        let text = new();
        
        {
            let mut temp = text.borrow_mut();

            temp.indent = Indent::Hanging(1);

            temp.entries.push_back(Entry::new("xxxxx"));
            temp.entries.push_back(Entry::new("yyyyyyyyyy"));
            temp.entries.push_back(Entry::new("xxxxxxxxxxxx"));
            temp.entries.push_back(Entry::new("yyy\nyyyy\n"));
            temp.entries.push_back(Entry::new("xxxxx"));
        }

        text.borrow().get_draw_data(&mut buf, Coord{x: 0, y: 0}, Coord{x: 10, y: 10});

        //print_buffer(&buf);

        for (i, x) in expected.iter().enumerate() {
            assert_eq!(buf.buffer.get_flat(i), *x)
        }
    }

    #[test]
    fn offset_test() {
        let x = Pixel::new('x', Color::Rgb { r: 255, g: 255, b: 255 }, Color::Rgb { r: 0, g: 0, b: 0 });
        let y = Pixel::new('y', Color::Rgb { r: 255, g: 255, b: 255 }, Color::Rgb { r: 0, g: 0, b: 0 });
        let s = Pixel::new(' ', Color::Rgb { r: 255, g: 255, b: 255 }, Color::Rgb { r: 0, g: 0, b: 0 });

        let expected = vec![
            y,y,y,y,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            x,x,x,x,x,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
        ];

        let mut buf = ScreenBuf::new(Coord{x: 10, y: 10});

        let text = new();
        
        {
            let mut temp = text.borrow_mut();

            temp.entries.push_back(Entry::new("xxxxx"));
            temp.entries.push_back(Entry::new("yyyyyyyyyy"));
            temp.entries.push_back(Entry::new("xxxxxxxxxxxx"));
            temp.entries.push_back(Entry::new("yyy\nyyyy\n"));
            temp.entries.push_back(Entry::new("xxxxx"));
        }

        text.borrow().get_draw_data(&mut buf, Coord{x: 5, y: 0}, Coord{x: 10, y: 10});

        //print_buffer(&buf);

        for (i, x) in expected.iter().enumerate() {
            assert_eq!(buf.buffer.get_flat(i), *x)
        }
    }

    #[test]
    fn offset_test2() {
        let x = Pixel::new('x', Color::Rgb { r: 255, g: 255, b: 255 }, Color::Rgb { r: 0, g: 0, b: 0 });
        let s = Pixel::new(' ', Color::Rgb { r: 255, g: 255, b: 255 }, Color::Rgb { r: 0, g: 0, b: 0 });

        let expected = vec![
            x,x,x,x,x,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
            s,s,s,s,s,s,s,s,s,s,
        ];

        let mut buf = ScreenBuf::new(Coord{x: 10, y: 10});

        let text = new();
        
        {
            let mut temp = text.borrow_mut();

            temp.entries.push_back(Entry::new("xxxxx"));
            temp.entries.push_back(Entry::new("yyyyyyyyyy"));
            temp.entries.push_back(Entry::new("xxxxxxxxxxxx"));
            temp.entries.push_back(Entry::new("yyy\nyyyy\n"));
            temp.entries.push_back(Entry::new("xxxxx"));
        }

        text.borrow().get_draw_data(&mut buf, Coord{x: 0, y: 4}, Coord{x: 10, y: 10});

        //print_buffer(&buf);

        for (i, x) in expected.iter().enumerate() {
            assert_eq!(buf.buffer.get_flat(i), *x)
        }
    }
}