use std::{
    fs::File,
    io::{BufRead, BufReader, Stdout},
};

use crossterm::{
    cursor::{self, MoveTo},
    style::{Print, SetBackgroundColor, SetForegroundColor},
    QueueableCommand,
};
use crossterm::{execute, terminal, Result};
use errno::errno;
use std::io::{stdout, Write};

use crate::row::Row;

pub struct Editor {
    // width in columns
    w: u16,
    //height in rows
    h: u16,
    // the cursor position
    cursor: (u16, u16),
    // number of rows
    row_count: u16,
    // the rows
    rows: Vec<Row>,
    // row offset
    rowoff: u16,

    //filename
    filename: String
}

impl Editor {
    pub fn new() -> Result<Self> {
        let (w, h) = terminal::size()?;
        Ok(Self {
            w,
            h,
            cursor: (0, 0),
            row_count: 0,
            rows: vec![],
            rowoff: 10,
            filename: "*None*".to_string()
        })
    }

    pub fn upadate_size(&mut self, w: u16, h: u16, stdout: &mut Stdout) -> Result<()> {
        self.w = w;
        self.h = h;
        execute!(stdout, terminal::SetSize(w, h))?;
        Ok(())
    }

    pub fn get_size(&mut self) -> (u16, u16) {
        return (self.w, self.h);
    }

    pub fn clear_screen(&mut self, stdout: &mut Stdout) -> Result<()> {
        stdout
            .queue(cursor::Hide)?
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?
            .flush()?;
        Ok(())
    }

    pub fn scroll(&mut self) -> Result<()> {
        if self.cursor.1 < self.rowoff {
            self.rowoff = self.cursor.1;
        }

        if self.cursor.1 + 1 >= self.h + self.rowoff {
            self.rowoff += 1
        }

        Ok(())
    }

    pub fn draw_rows(&mut self, stdout: &mut Stdout) -> Result<()> {
        for y in 0..(self.h - 1) {
            let fileoff = y + self.rowoff;
            if fileoff >= self.row_count {
                self.write_line(stdout, "~".to_string())?;
            } else {
                let row = self.rows.get_mut(fileoff as usize).unwrap();
                let mut row_content = row.get_content();
                let mut len = row_content.len() as u16;

                if len > self.w {
                    len = self.w
                }

                row_content = row_content.split_at(len as usize).0.to_string();

                stdout.queue(cursor::MoveTo(0, y))?;

                self.write_line(stdout, row_content)?;
            }
        }

        stdout.queue(cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn draw_bottom_bar(&mut self, stdout: &mut Stdout) -> Result<()> {
        stdout
            .queue(cursor::MoveTo(0, self.h))?
            .queue(SetBackgroundColor(crossterm::style::Color::Grey))?
            .queue(SetForegroundColor(crossterm::style::Color::Black))?;

        for _ in 0..self.w {
            self.write(stdout, " ".to_string())?;
        }

        stdout.queue(cursor::MoveTo(0, self.h))?;

        self.write(
            stdout,
            format!(
                "{:<32}",
                self.filename
            ),
        )?;

        stdout
            .queue(cursor::MoveTo(0, 0))?
            .queue(SetBackgroundColor(crossterm::style::Color::Reset))?
            .queue(SetForegroundColor(crossterm::style::Color::White))?;

        Ok(())
    }

    pub fn write_line(&mut self, stdout: &mut Stdout, s: String) -> Result<()> {
        stdout.queue(Print(s))?.queue(cursor::MoveToNextLine(1))?;

        Ok(())
    }

    pub fn write(&mut self, stdout: &mut Stdout, s: String) -> Result<()> {
        for c in s.chars() {
            stdout.queue(Print(c))?;
        }
        Ok(())
    }

    pub fn refresh(&mut self, stdout: &mut Stdout) -> Result<()> {
        self.scroll()?;
        self.clear_screen(stdout)?;
        self.draw_bottom_bar(stdout)?;
        self.draw_rows(stdout)?;
        stdout
            .queue(cursor::MoveTo(self.cursor.0,self.cursor.1 - self.rowoff))?
            .queue(cursor::Show)?;
        stdout.flush()?;
        Ok(())
    }

    pub fn die<S: Into<String>>(&mut self, msg: S) {
        let mut stdout = stdout();
        let _ = self.clear_screen(&mut stdout);
        let _ = terminal::disable_raw_mode();
        eprintln!("{}: {}", msg.into(), errno());
        std::process::exit(1);
    }

    pub fn get_cursor_pos(&mut self) -> (u16, u16) {
        self.cursor
    }

    pub fn cursor_move_left(&mut self) {
        if self.cursor.0 > 0 {
            self.cursor.0 -= 1
        }
    }

    pub fn cursor_move_right(&mut self) {
        if self.cursor.0 < self.get_size().0 {
            self.cursor.0 += 1
        }
    }

    pub fn cursor_move_up(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1
        }
    }

    pub fn cursor_move_down(&mut self) {
        if self.cursor.1 < self.row_count {
            self.cursor.1 += 1
        }
    }

    pub fn page_up(&mut self) {
        for _ in 0..self.get_size().1 {
            self.cursor_move_up()
        }
    }

    pub fn page_down(&mut self) {
        for _ in 0..self.h - 1 {
            self.cursor_move_down()
        }
    }

    pub fn open_editor(&mut self) {
        self.append_row("Hello World".to_string());
    }

    pub fn append_row(&mut self, s: String) {
        let mut r = Row::new();
        r.set_content(s);
        self.rows.push(r);
        self.row_count += 1;
    }

    pub fn open_file(&mut self, filepath: String) -> Result<()> {
        
        self.filename = filepath.clone();
        let binding = std::fs::read_to_string(filepath).expect("unable to read file");
        let lines: Vec<&str> = binding.split("\n").collect();

        for line in lines {
            self.append_row(line.to_string());
        }

        Ok(())
    }
}
