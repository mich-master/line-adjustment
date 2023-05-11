use core::slice::Iter;
use std::str;

#[derive(PartialEq, Eq, Debug)]
pub enum DocError {
    WordTooLong,
}

struct Line<'a> {
    words: Vec<&'a str>,
    char_counter: u32,
}

impl<'a> Line<'a> {
    fn new_with_word(word: &'a str) -> Line<'a> {
        let char_counter: u32 = word.len() as u32;
        Line::<'_>{
            words: vec![word],
            char_counter,
        }
    }
    fn add_word(&mut self, word: &'a str) {
        self.char_counter += word.len() as u32;
        self.words.push(word);
    }
    fn word_fits(&self, word: &'a str, line_width: u32) -> bool {
        self.char_count() + self.word_count() + word.len() as u32 <= line_width
    }
    fn char_count(&self) -> u32 {
        self.char_counter
    }
    fn word_count(&self) -> u32 {
        self.words.len() as u32
    }
    fn iter(&self) -> Iter<'a, &str> {
        self.words.iter()
    }
}


pub struct Document<'a> {
    lines: Vec<Line<'a>>,
    line_width: u32,
}

impl<'a> Document<'a> {
    fn add_word(&mut self, word: &'a str) {
        let create_new_line: bool = self.lines.last().map_or(true, |line| !line.word_fits(word, self.line_width));
        if create_new_line {
            self.lines.push( Line::new_with_word(word) );
        } else if let Some(line) = self.lines.last_mut() {
            line.add_word(word);
        }
    }
    pub fn from_str(input: &str, line_width: u32) -> Result<Document, DocError> {
        let mut doc: Document =
            Document {
                lines: Vec::new(),
                line_width,
            };
        
        for word in input.split_whitespace() {
            if word.len() as u32 > line_width {
                return Err(DocError::WordTooLong)
            }
            doc.add_word(word);
        };

        Ok(doc)
    }
    pub fn format_to_string(&self) -> String {
        
        let add_whitespaces = |s: &mut String, count| {
            for _ in 0..count {
                s.push(' ');
            }
        };

        let text_capacity: usize =
            match self.lines.len() {
                0 => 0,
                1 => self.line_width as usize * 2,                                              // Умножаем на 2 - предусматриваем место для Utf-8 символов, пренебрегая экзотическими символами
                _ => self.lines.len() * self.line_width as usize * 2 + self.lines.len() - 1,    // Умножаем на 2 - предусматриваем место для Utf-8 символов, пренебрегая экзотическими символами
            };
        let mut text: String = String::with_capacity(text_capacity);                            // Заранее выделяем достаточно места, чтобы избежать лишнего релоцирования данных

        for (line_number, line) in self.lines.iter().enumerate() {
            if line.word_count() > 1 {
                let (base_witespace_width, extra_witespace, gap_count): (u32,u32,u32) = {
                    let whitespace_count: u32 = self.line_width - line.char_count();
                    let gap_count: u32 = line.word_count() - 1;
                    (whitespace_count / gap_count, whitespace_count % gap_count, gap_count)
                };

                for (word_number, word) in line.iter().enumerate() {
                    text.push_str(word);

                    if (word_number as u32) < gap_count {
                        let whitespaces: u32 =
                            if (word_number as u32) < extra_witespace {
                                base_witespace_width + 1
                            } else {
                                base_witespace_width
                            };
                        add_whitespaces(&mut text, whitespaces);
                    }
                }
            } else {
                let whitespaces: u32 = self.line_width - line.char_count();
                if let Some(word) = line.iter().next() {
                    text.push_str(word);
                }
                add_whitespaces(&mut text, whitespaces);
            }
            if line_number < self.lines.len() - 1 {
                text.push('\n');
            }
        }

        text
    }
}


pub fn transform(input: &str, line_width: u32) -> Result<String, DocError> {
    Document::from_str(input, line_width)
        .map(|document|
            document.format_to_string()
        )
}

#[cfg(test)]
mod tests {
    use crate::DocError;

    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
             "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      "),
            ("Lorem     ipsum    dolor", 17, "Lorem ipsum dolor"),
        ];

        for &(input, line_width, expected) in &test_cases {
            println!("input: '{}'", input);
            assert_eq!(transform(input, line_width), Ok(expected.to_string()));
        }
        assert_eq!(transform("abc_abc_abc_abc_abc", 12), Err(DocError::WordTooLong));
    }
}
