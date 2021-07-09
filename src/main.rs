use std::env::{args, current_dir};
use std::fs::{rename, DirBuilder};
use std::io;
use std::path::*;

fn main() {
    let currentpath = current_dir().unwrap();
    let mut index: u32 = 0;
    let foldername = String::from("New Folder");

    //currentpath.push(&foldername);
    loop {
        let mut templocation = currentpath.clone();
        let new_folder_name = format!("{} ({})", foldername, index);
        templocation.push(new_folder_name);
        if templocation.exists() == true {
            index = index + 1;
        } else {
            let _builder = DirBuilder::new().create(&templocation);
            let arg_files: Vec<String> = args().collect();
            for i in &arg_files[1..] {
                let filemover = move_files(&templocation, i);
                match filemover {
                    Ok(_) => println!("File {} has been moved", i),
                    Err(e) => println!("Error Moving {}. {:?}", i, e),
                }
            }
            break;
        }
    }
}

fn move_files(location_of_folder: &PathBuf, file_to_move: &String) -> Result<(), io::Error> {
    let file_path_src = Path::new(&file_to_move);
    let mut file_path_dst = PathBuf::new();
    file_path_dst.push(&location_of_folder);
    file_path_dst.push(&file_path_src.file_name().unwrap());
    rename(&file_path_src, &file_path_dst)
}
