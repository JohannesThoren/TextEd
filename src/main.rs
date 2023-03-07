mod row;
mod editor;
mod event;
mod keyboard;

use crossterm::{terminal, Result};
use editor::Editor;
use std::io::{stdout, Stdout};

use std::env;

fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    let mut stdout = stdout();
    let mut editor = Editor::new()?;


    if args.get(1).is_some() {
        editor.open_file(args.get(1).unwrap().to_string())?;
    }
    else {
        editor.open_editor();
    
    }

    terminal::enable_raw_mode()?;
    if main_loop(&mut stdout, &mut editor).is_err() {
        editor.die("something whent wrong! ")
    };
    terminal::disable_raw_mode()?;
    return Ok(());
}

fn main_loop(stdout: &mut Stdout, editor: &mut Editor) -> Result<()> {
    loop {
        event::handle_event(stdout, editor)?;
        editor.refresh(stdout)?;
    }
    Ok(())
}
