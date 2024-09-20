pub struct Error<'a> {
    line: u32,
    source: &'a str,
    text: &'a str
}

impl<'a> Error<'a> {
    pub fn new(text: &'a str, source: &'a str, line: u32) -> Self {
        Error {
            line: line,
            source: source,
            text: text
        }
    }

    pub fn print(&self) {
        println!("[Error] in {} at line {}:\n\t {}", self.source, self.line, self.text);
    }
}

