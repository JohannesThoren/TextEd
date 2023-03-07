use std::{io::Stdout, time::Duration};

use crossterm::{event::{Event, read, poll}, Result};

use crate::{keyboard::handle_input, editor};

pub fn handle_event(stdout: &mut Stdout, editor: &mut editor::Editor) -> Result<()> {
    // `poll()` waits for an `Event` for a given time period
    if poll(Duration::from_millis(100))? {
        // It's guaranteed that the `read()` won't block when the `poll()`
        // function returns `true`
        match read()? {
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(event) => handle_input(event, stdout, editor)?,
            Event::Mouse(event) => {}
            #[cfg(feature = "bracketed-paste")]
            Event::Paste(data) => {}
            Event::Resize(width, height) => editor.upadate_size(width, height, stdout)?,
            e => {
                println!("{:?}", e)
            }
        }
    } else {
        // Timeout expired and no `Event` is available
    }
    Ok(())
}