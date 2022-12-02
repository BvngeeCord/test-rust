use std::{time::Duration, io::stdout};

use crossterm::{Result, event::{poll, KeyEvent, KeyCode, Event, self}, execute, terminal::{EnterAlternateScreen, self, size, LeaveAlternateScreen}};
use display::{screen::Screen, window::Window};

mod util;
mod info_sources;
mod display;

fn main() -> Result<()> {
    // let mut w = std::io::stdout();
    //
    // // let (cols, rows) = size()?;
    // execute!(w, EnterAlternateScreen)?;
    // terminal::enable_raw_mode()?;
    //
    // let (mut cols, mut rows) = size()?;
    // queue!(w, MoveTo(0, 0))?;
    // loop {
    //
    //     let (updated_cols, updated_rows) = size()?;
    //     if updated_cols != cols || updated_rows != rows {
    //         cols = updated_cols;
    //         rows = updated_rows;
    //
    //         // let line_top: String = format!("q{:o^width$}q", "TOP", width = (cols-2) as usize);
    //         // let line_left: String = format!("{:o^height$}q", "LEFT", height = (rows-2) as usize);
    //
    //         queue!(w, Clear(ClearType::All))?;
    //
    //         // let cols_str = cols.to_string();
    //         // let rows_str = rows.to_string();
    //         // queue!(w, 
    //         //         MoveTo(cols/2 - (cols_str.len() + rows_str.len() + 1) as u16 / 2, rows/2), 
    //         //         Print(cols_str + " " + &rows_str),
    //         //         MoveTo(0, 0),
    //         //         Print(line_top),
    //         // )?;
    //         //
    //         // queue!(w, MoveToColumn(0))?;
    //         // for c in line_left.chars() {
    //         //     queue!(w, MoveDown(1), MoveToColumn(0), Print(c))?;
    //         // }
    //         
    //     }
    //
    //     if let Some(char) = read_char() {
    //         if char.eq(&'q') {
    //             break;
    //         }
    //
    //     }
    //
    //     w.flush()?;
    // }
    //
    // queue!(w, LeaveAlternateScreen)?;
    // w.flush()?;
    // terminal::disable_raw_mode()?;

    // let components: Vec<ComponentSection> = vec![
    //     ComponentSection {
    //         width: 0.5f32,
    //         height: 0f32,
    //         formatter: "{}",
    //         sources: vec![Box::new(ListSource::Filenames), Box::new(InfoSource::Preview)],
    //         data: Vec::new(),
    //     },
    //     ComponentSection {
    //         width: 0.2f32,
    //         height: 0f32,
    //         formatter: "format in front???; {}",
    //         sources: vec![Box::new(ListSource::Filenames), Box::new(InfoSource::Preview)],
    //         data: Vec::new(),
    //     }
    // ];
    // let (cols, rows) = size()?;
    // let mut screen: Screen = Screen::new(cols, rows, components);
    //
    // execute!(stdout(), EnterAlternateScreen)?;
    // terminal::enable_raw_mode()?;
    //
    // screen.update()?;
    //
    // loop {
    //     if let Some(char) = read_char() {
    //         if char.eq(&'q') {
    //             break;
    //         }
    //     }
    // }
    // 
    // queue!(stdout(), LeaveAlternateScreen)?;
    // stdout().flush()?;
    // terminal::disable_raw_mode()?;
    
    let mut w = stdout();
    execute!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let windows: Vec<Window> = Vec::new();

    let (columns, rows) = size()?;
    let screen: Screen = Screen {
        columns,
        rows,
        windows,
        buffer: Vec::new(),
    };

    loop {
        if let Some(char) = read_char() {
            if char.eq(&'q') {
                break;
            }
        }

        if let Some(char) = read_char() {
            if char.eq(&' ') {
                // screen.update();
            }
        }
        screen.render()
    }

    execute!(w, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

pub fn read_char() -> Option<char> {

    if events_available().unwrap() {
        
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Some(c);
        }
    }
    None

}

fn events_available() -> Result<bool> {
    poll(Duration::from_secs(0))
}
