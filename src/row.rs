use std::cmp;
use termion::color;
use unicode_segmentation::UnicodeSegmentation;
use crate::SearchDirection;
use crate::highlighting;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
    highlighting: Vec<highlighting::Type>,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Self {
            string: String::from(value),
            len: value.graphemes(true).count(),
            highlighting: Vec::new(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();
        #[allow(clippy::integer_arithmetic)]
        for (index, grapheme) in self.string[..]
            .graphemes(true)
            .enumerate()
            .skip(start)
            .take(end - start)
        {
            if let Some(c) = grapheme.chars().next() {
                let highlighting_type = self
                .highlighting
                .get(index)
                .unwrap_or(&highlighting::Type::None);

                let start_highlight = format!("{}", termion::color::Fg(highlighting_type.to_color()));
                result.push_str(&start_highlight[..]);

                if c == '\t' {
                    result.push_str(" ");
                } else {
                    result.push(c);
                }

                let end_highlight = format!("{}", termion::color::Fg(color::Reset));
                result.push_str(&end_highlight[..]);
            }
        }
        result
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
            self.len += 1;
            return;
        }

        let mut result: String = String::new();
        let mut length = 0; 
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            length += 1;
            if index == at {
                length += 1;
                result.push(c);
            }
            result.push_str(grapheme);
        }
        self.len = length; 
        self.string = result;
    }


    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        } else {
            let mut result: String = String::new();
            let mut length = 0; 
            for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
                if index != at {
                    length += 1;
                    result.push_str(grapheme);
                }
            }
            self.len = length; 
            self.string = result;
        }
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    pub fn split(&mut self, at: usize) -> Self {
        let mut row: String = String::new(); 
        let mut length = 0; 
        let mut split_row: String = String::new();
        let mut split_length = 0; 
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index < at {
                length += 1;
                row.push_str(grapheme);
            } else {
                split_length += 1;
                split_row.push_str(grapheme);
            }
        }

        self.string = row; 
        self.len = length;

        Self {
            string: split_row,
            len: split_length,
            highlighting: Vec::new(),
        }
    }

    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
      
        if at > self.len {
            return None; 
        }

        let start = if direction == SearchDirection::Forward {
            at
        } else {
            0
        };

        let end = if direction == SearchDirection::Forward {
            self.len
        } else {
            at
        };

        let substring: String = self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();

        let matching_byte_index = if direction == SearchDirection::Forward {
            substring.find(query)
        } else {
            substring.rfind(query)
        };



        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in 
                substring[..].grapheme_indices(true).enumerate()
            {
                if matching_byte_index == byte_index {
                    #[allow(clippy::integer_arithmetic)]
                    return Some(start + grapheme_index);
                }
            }
        }
        None
    }

    pub fn highlight(&mut self) {
        let mut highlighting = Vec::new();
        for c in self.string.chars() {
            if c.is_ascii_digit() {
                highlighting.push(highlighting::Type::Number);
            } else {
                highlighting.push(highlighting::Type::None);
            }
        }

        self.highlighting = highlighting;
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn len(&self) -> usize {
        // self.string.len()
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }

}
