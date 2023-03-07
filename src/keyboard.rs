use std::io::Stdout;

use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    terminal, Result,
};

use crate::editor;

/// entry point for handling the input
pub fn handle_input(
    event: KeyEvent,
    stdout: &mut Stdout,
    editor: &mut editor::Editor,
) -> Result<()> {

    // check if we have any modifiers
    if event.modifiers != KeyModifiers::NONE {
        return handle_modifiers(event, stdout, editor);
    }


    // check keycodes
    match event.code {
        // handle character input
        KeyCode::Char(c) => Ok(()),

        // handle arrowkey input
        KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
            handle_arrow_keys(editor, event)
        }

        // handle page key input
        KeyCode::PageUp | KeyCode::PageDown => handle_page_keys(editor, event),

        // handle enter key input
        KeyCode::Enter => {
            editor.die("Enter is not implmented yet");
            Ok(())
        }
        _ => Ok(()),
    }
}

// match for each individual arrow key
pub fn handle_arrow_keys(editor: &mut editor::Editor, event: KeyEvent) -> Result<()> {
    match event.code {
        KeyCode::Up => editor.cursor_move_up(),
        KeyCode::Down => editor.cursor_move_down(),
        KeyCode::Right => editor.cursor_move_right(),
        KeyCode::Left => editor.cursor_move_left(),
        _ => {}
    }

    Ok(())
}

pub fn handle_page_keys(editor: &mut editor::Editor, event: KeyEvent) -> Result<()> {
    match event.code {
        KeyCode::PageDown => editor.page_down(),
        KeyCode::PageUp => editor.page_up(),
        _ => {}
    }
    Ok(())
}

pub fn handle_modifiers(
    event: KeyEvent,
    stdout: &mut Stdout,
    editor: &mut editor::Editor,
) -> Result<()> {
    match event.modifiers {
        KeyModifiers::CONTROL => handle_ctrl_modifier(event, stdout, editor),
        _ => Ok(()),
    }
}

pub fn handle_ctrl_modifier(
    event: KeyEvent,
    stdout: &mut Stdout,
    editor: &mut editor::Editor,
) -> Result<()> {
    match event.code {
        KeyCode::Char('q') => {
            terminal::disable_raw_mode()?;
            editor.clear_screen(stdout)?;
            std::process::exit(0);
        }
        _ => Ok(()),
    }
}
