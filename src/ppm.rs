#[derive(Debug)]
pub struct Ppm {
    lines: Vec<String>,
}

impl Ppm {
    pub fn new(width: usize, height: usize) -> Ppm {
        let mut lines = vec![];
        lines.push("P3\n".into());
        lines.push(format!("{} {}\n", width, height));
        lines.push("255\n".into());

        Ppm { lines }
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.push(line);
    }

    #[allow(dead_code)]
    pub fn lines_range(&self, start: usize, end: usize) -> String {
        self.lines[start..end].join("")
    }

    pub fn whole_file(&self) -> String {
        self.lines.join("")
    }
}
