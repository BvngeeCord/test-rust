use std::{io::Write, time::Duration};

use crossterm::{
    Result,
    event::{self, Event, KeyCode, KeyEvent, poll},
    queue,
    style::Print, cursor::{MoveTo, MoveDown, MoveToColumn}, terminal::{self, size, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType}, execute,
};

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();
    run(&mut stdout)
}

fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    // let (cols, rows) = size()?;
    execute!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    // let mut sys_time = std::time::SystemTime::now();
    // loop {
    //     if let Some(char) = read_char() {
    //         queue!(w, Print(char))?;
    //         if char.eq(&'q') {
    //             break;
    //         }
    //
    //     }
    //
    //     if sys_time.elapsed().unwrap() >= Duration::new(1, 0) {
    //         queue!(w, Print("NOW"), MoveToNextLine(1))?;
    //         sys_time = std::time::SystemTime::now();
    //     } else {
    //         // queue!(w, Print(std::time::SystemTime::now().duration_since(elapsed).unwrap().as_millis()), MoveToNextLine(1))?;
    //         queue!(w, MoveToColumn(0), Print(sys_time.elapsed().unwrap().as_millis()))?;
    //     }
    //
    //     w.flush()?;
    // }

    let (mut cols, mut rows) = size()?;
    queue!(w, MoveTo(0, 0))?;
    loop {

        let (updated_cols, updated_rows) = size()?;
        if updated_cols != cols || updated_rows != rows {
            cols = updated_cols;
            rows = updated_rows;

            let line_top: String = format!("q{:o^width$}q", "TOP", width = (cols-2) as usize);
            let line_left: String = format!("{:o^height$}q", "LEFT", height = (rows-2) as usize);

            // queue!(w, MoveTo(0, 0), Clear(ClearType::CurrentLine))?;
            // queue!(w, MoveTo(0, 1), Clear(ClearType::CurrentLine))?;
            queue!(w, Clear(ClearType::All))?;

            let cols_str = cols.to_string();
            let rows_str = rows.to_string();
            queue!(w, 
                    MoveTo(cols/2 - (cols_str.len() + rows_str.len() + 1) as u16 / 2, rows/2), 
                    Print(cols_str + " " + &rows_str),
                    MoveTo(0, 0),
                    Print(line_top),
            )?;

            queue!(w, MoveToColumn(0))?;
            for c in line_left.chars() {
                queue!(w, MoveDown(1), MoveToColumn(0), Print(c))?;
            }
        }

        if let Some(char) = read_char() {
            if char.eq(&'q') {
                break;
            }

        }

        w.flush()?;
    }

    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
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
