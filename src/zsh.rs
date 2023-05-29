use std::fs;
use std::fs::File;
use std::io::Read;

use regex::Regex;

/*
    Credit: code copied and altered from
    https://github.com/overclockworked64/hstr-rs/blob/master/src/hstr.rs
*/

pub fn read_zsh(filepath: &str) -> Vec<String> {
    let mut f = File::open(&filepath).expect("no file found");
    let metadata = fs::metadata(&filepath).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    return process_history(buffer);
}

pub fn process_history(history: Vec<u8>) -> Vec<String> {
    let bytes = unmetafy(history)
        .split(|byte| *byte == 10) // split on newline
        .map(|line| String::from_utf8(line.to_vec()).unwrap())
        .collect();
    return remove_timestamps(bytes);
}

fn unmetafy(mut bytestring: Vec<u8>) -> Vec<u8> {
    /* Unmetafying zsh history requires looping over the bytestring, removing
     * each encountered Meta character, and XOR-ing the following byte with 32.
     *
     * For instance:
     *
     * Input: ('a', 'b', 'c', Meta, 'd', 'e', 'f')
     * Wanted: ('a', 'b', 'c', 'd' ^ 32, 'e', 'f')
     */
    const ZSH_META: u8 = 0x83;

    for index in (0..bytestring.len()).rev() {
        if bytestring[index] == ZSH_META {
            bytestring.remove(index);
            bytestring[index] ^= 32;
        }
    }
    bytestring
}

fn remove_timestamps(history: Vec<String>) -> Vec<String> {
    /* The preceding metadata needs to be stripped
     * because zsh history entries look like below:
     *
     * `: 1330648651:0;sudo reboot`
     */
    let r = Regex::new(r"^: \d{10}:\d;").unwrap();
    history
        .iter()
        .map(|line| r.replace(line, "").into_owned())
        .collect()
}