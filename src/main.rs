use std::fs::create_dir;
use std::path::Path;

fn main() {
    let notes_folder_path = Path::new("./notes");

    if !notes_folder_path.exists() {
       create_dir("notes").unwrap();
    }
    
    println!("folder is already created");
}
