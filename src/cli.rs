use std::io;
use std::io::Write;
use std::process::exit;

use crossterm::{cursor, ExecutableCommand, QueueableCommand, terminal};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub fn render_cli(values: Vec<String>) -> String {
    const PAGE_SIZE: i32 = 5;

    let mut stdout = io::stdout();
    stdout.execute(cursor::Hide).expect("TODO: panic message");
    enable_raw_mode().expect("TODO: panic message");

    let mut current_page = 0;
    let total_pages = values.len() as i32 / PAGE_SIZE;

    loop {
        let first_index_page = (current_page * PAGE_SIZE) as usize;
        let last_index_page = if current_page * PAGE_SIZE + PAGE_SIZE < values.len() as i32 { (current_page * PAGE_SIZE + PAGE_SIZE) as usize } else { values.len() - 1 };
        let page = &values[first_index_page..last_index_page];

        for (index, value) in page.iter().enumerate() {
            let index = ((index + 1).to_string() + ".").dark_blue();
            stdout.write_all(format!("{} {} \n\r", index, value).as_bytes()).expect("TODO");
        }
        stdout.write_all(format!("Page {} of {}, {} - next page, {} - previous page \n\r",
                                 current_page + 1,
                                 total_pages,
                                 "'n'".dark_blue(),
                                 "'p'".dark_blue()).as_bytes()).expect("TODO");
        stdout.flush().unwrap();

        match read().unwrap() {
            Event::Key(KeyEvent { code: KeyCode::Char('n'), .. }) => {
                if current_page != (total_pages - 1) as i32 {
                    current_page += 1;
                }
            }
            Event::Key(KeyEvent { code: KeyCode::Char('p'), .. }) => {
                if current_page > 0 {
                    current_page -= 1;
                }
            }
            Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, .. }) => {
                disable_raw_mode().expect("TODO: panic message");
                stdout.execute(cursor::Show).unwrap();
                exit(0);
            }
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Char(char) => {
                        if char.is_digit(10) {
                            let page_item_index = char.to_digit(10).expect("This should be digit") as usize;
                            let real_page_item_index = page_item_index - 1;
                            if real_page_item_index >= 0 && real_page_item_index < page.len() {
                                stdout.execute(cursor::MoveUp((page.len() + 1) as u16)).expect("TODO");
                                stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).expect("TODO panic");
                                disable_raw_mode().expect("TODO: panic message");
                                stdout.execute(cursor::Show).unwrap();

                                return page[real_page_item_index].clone();
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        stdout.execute(cursor::MoveUp((page.len() + 1) as u16)).expect("TODO");
        stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).expect("TODO panic");
    }
}