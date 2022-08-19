use crate::prelude::ColorSet;


#[derive(Copy, Clone)]
pub struct ColorSlice {
    pub start:  usize,
    pub end:    usize,
    pub colors: ColorSet,
}

#[derive(Clone)]
pub struct ColorString {
    pub string: String,
    pub colors: Vec<ColorSlice>
}

impl ColorString {
    pub fn get_color(&self, pos: usize) -> Option<ColorSet> {
        for slice in &self.colors {
            if pos >= slice.start && pos < slice.end {
                return Some(slice.colors)
            }
            if pos < slice.start { break }
        }

        None
    }
}

impl From<String> for ColorString {
    fn from(s: String) -> Self {
        ColorString { string: s, colors: Vec::new() }
    }
}

impl From<&str> for ColorString {
    fn from(s: &str) -> Self {
        ColorString { string: s.to_string(), colors: Vec::new() }
    }
}