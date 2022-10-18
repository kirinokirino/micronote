#![feature(fs_try_exists)]

use dirs::data_local_dir;

use std::fs;
use std::io::{Read, Write};
use std::os::unix::prelude::CommandExt;
use std::path::PathBuf;
use std::{env, process};

fn main() {
    let dir = data_local_dir().unwrap_or_else(|| PathBuf::from("/home/k/.local/share"));
    assert!(fs::try_exists(dir.clone()).is_ok());
    let notes_path = dir.join("notes");

    let mut args = pico_args::Arguments::from_env();

    if args.contains(["-e", "--edit"]) {
        let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
        let error = process::Command::new(editor).arg(notes_path).exec();
        panic!("{}", error);
    }

    if args.contains(["-h", "--help"]) {
    	println!("{}", HELP);
    	process::exit(0);
    }

    if args.contains(["-f", "--file"]) {
    	println!("{}", notes_path.display());
    	process::exit(0);
    }

    let mut notes_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open(notes_path)
        .unwrap();

    let note = args.finish();
    if note.is_empty() {
        let mut contents = String::new();
        let result = notes_file.read_to_string(&mut contents);
        assert!(result.is_ok());
        println!("{}", contents);
    } else {
        let mut note = note
            .iter()
            .map(|s| s.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");
        note.push('\n');

        let result = notes_file.write_all(note.as_bytes());
        assert!(result.is_ok());
        let result = notes_file.flush();
        assert!(result.is_ok());
    }
}

static HELP: &str = "Usage:
  micronote 				Print notes
  micronote	<note text> 	Save note text
  micronote -f 				Print notes file path
  micronote -e 				Opens notes file in the editor
  micronote -h  			Print this help text";
