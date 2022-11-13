use std::io::{stdout, Stdout, Write};

use super::super::sources::Source;
use crossterm::{style::Print, queue, terminal::{Clear, ClearType}, Result, cursor::MoveTo};

pub struct ComponentSection {
    pub width: f32,
    pub height: f32,
    pub formatter: &'static str,
    pub sources: Vec<Box<dyn Source>>,
    pub data: Vec<String>,
}

pub struct Screen {
    cols: u16,
    rows: u16,

    w: Stdout,
    sections: Vec<ComponentSection>,
}

impl Screen {
    pub fn update(&mut self) -> Result<()> {
        queue!(self.w, Clear(ClearType::All))?;

        let mut x: u16 = 0;
        for section in &self.sections {
            queue!(self.w, MoveTo(x, 0))?;
            // for line in &section.data {
            //     queue!(self.w, Print(line))?;
            // }
            queue!(self.w, Print(section.formatter))?;
            x += (section.width * self.cols as f32).round() as u16;
        }

        self.w.flush()?;
        Ok(())
    }

    pub fn set_size(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
    }
}

impl Screen {
    pub fn new(cols: u16, rows: u16, components: Vec<ComponentSection>) -> Self {
        Self {
            cols,
            rows,
            w: stdout(),
            sections: components,
        }
    }
}
