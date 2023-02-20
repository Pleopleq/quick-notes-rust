// use std::fs::{DirBuilder};
use std::env::current_dir;
use std::path::PathBuf;

fn get_current_dir() -> PathBuf {
    let path = current_dir();
    return path.unwrap();
}

fn main()   {
    let path = get_current_dir();
    let notes_path = path.display();
    println!("{}", notes_path)
}
