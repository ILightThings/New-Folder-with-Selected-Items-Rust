use std::env::{args, current_dir};
use std::fs::{rename, DirBuilder};
use std::path::*;
use std::process::Command;

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
                println!("Arg: {}", &i);
                let mut file_path_src = Path::new(&i);
                let mut file_path_dst = PathBuf::new();
                file_path_dst.push(&templocation);
                file_path_dst.push(&file_path_src.file_name().unwrap());
                let _move_agent = rename(&file_path_src, &file_path_dst).expect("Failed");
                
            }
            break;
        }

        //Build new folder
    }
}
