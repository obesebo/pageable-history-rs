use std::{env, fs};
use std::path::Path;
use std::process::{exit};

use arboard::Clipboard;
use indexmap::{IndexSet};

use crate::cli::render_cli;
use crate::zsh::read_zsh;

mod zsh;
mod cli;

struct Shell {
    keyword: &'static str,
    history: &'static str,
}

impl Shell {
    pub const BASH: Shell = Shell { keyword: "bash", history: ".bash_history" };
    pub const ZSH: Shell = Shell { keyword: "zsh", history: ".zsh_history" };

    pub const VALUES: [Shell; 2] = [Shell::BASH, Shell::ZSH];
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.is_empty() || args.len() == 1 {
        println!("No keywords provided");
        exit(0);
    }
    args.remove(0);
    // dbg!(&args);

    let hist_file_path = env::var("HISTFILE");

    let (history_file_full_path, shell_type) = match hist_file_path {
        Ok(histfile) => {
            let current_shell = Shell::VALUES.iter()
                .find(|val| { histfile.contains(val.keyword) })
                .expect(format!("Shell not supported {histfile}").as_str());
            (histfile, current_shell.keyword)
        }
        Err(_) => {
            let home_dir = env::var("HOME").expect("could not determine a home directory");
            let shell = env::var("SHELL").expect("could not determine default shell");
            let current_shell = Shell::VALUES.iter()
                .find(|val| { shell.contains(val.keyword) })
                .expect(format!("Shell not supported {shell}").as_str());

            let history_full_path = home_dir.clone() + "/" + current_shell.history;
            (history_full_path, current_shell.keyword)
        }
    };

    println!("History file: {}", history_file_full_path);
    if !Path::new(history_file_full_path.as_str()).exists() {
        panic!("History file {history_file_full_path} is nowhere to be found");
    }

    // TODO refactor
    let history_data: Vec<String> = match shell_type {
        "zsh" => {
            read_zsh(history_file_full_path.as_str())
        }
        "bash" => {
            let data = fs::read_to_string(history_file_full_path.clone()).expect(format!("Error while reading file {history_file_full_path}").as_str());
            let history_lines: Vec<String> = data.split("\n").map(|value| String::from(value)).collect();

            history_lines
        }
        _ => {
            panic!("Not supported shell");
        }
    };

    let mut filtered_history: Vec<String> = history_data.iter()
        .map(|line| { line.trim().to_string() })
        .filter(|line| { args.iter().all(|arg| { line.contains(arg) }) })
        .collect();

    if filtered_history.is_empty() {
        println!("No history entries for filter");
    }

    //reverse history order and remove duplicates while maintaining order
    filtered_history.reverse();
    let filtered_history: IndexSet<&String> = IndexSet::from_iter(filtered_history.iter());
    let filtered_history: Vec<String> = filtered_history.into_iter().cloned().collect();

    let command = render_cli(filtered_history);
    let mut clipboard = Clipboard::new().unwrap();

    clipboard.set_text(command.clone()).expect("Failed to insert into clipboard");
    println!("Command '{}' set to clipboard", command);
}
