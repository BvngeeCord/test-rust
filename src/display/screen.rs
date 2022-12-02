//Main visual outlet for the application. Made up of at least one Window, or multiple arranged in
//tabs or splits.

use crate::display::window::Window;

enum WindowMode {
    Single,
    Tabs,
    Splits
}

pub struct Screen<'a> {
    pub columns: u16,
    pub rows: u16,
    pub windows: Vec<Window<'a>>,
    pub buffer: Vec<Vec<&'a str>>
}

impl<'a> Screen<'a> {
    ///formats (with borders) and draws the buffer to screen
    pub fn render(&self) {

        let window1: Vec<&str> = self.windows.get(0).unwrap().update_and_fetch(0, self.columns/3, 0, self.rows);
        println!("{}", format!("{}", window1.get(0).unwrap()));

    }

    // ///windows' sources update and each section stores relevant info, then info is sent back through to the screen buffer
    // pub fn update(&mut self) {
    //
    //     for window in &mut self.windows {
    //         self.buffer.push(
    //                 window.update_and_fetch(0, self.columns, 0, self.rows)
    //             );
    //     }
    //
    // }
}
