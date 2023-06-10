//Here we configure the compilation. We are allowing dead code and unused imports for the dev instance only
//This means that we wont get warning during compilation about unused imports.
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod error_msgs;
mod cp_fns;
use error_msgs::*;
use cp_fns::*;
use anyhow::{Result, Context};
use fs_extra::dir::CopyOptions;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Read;
use std::env::{current_exe};
use fs_extra::*;

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    current_file_path: String,
    export_path: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    create_new_folder: bool,
    files: Vec<Item>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
    }
}

fn run() -> Result<()> {
    
    println!("current working path: {:?}",  current_exe());
    let mut file = File::open("data.json").with_context(|| "Failed to open file")?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).with_context(|| "Failed to read file")?;
    
    let _data: Vec<Data> = serde_json::from_str(&json_data).expect("Failed to parse JSON_data");

    // Create a new instance of the options that we gonna pass to copy.
    let mut options = CopyOptions::new();
    options.overwrite = true;                
    options.buffer_size = 64000;

    let _dir_handle = |process_info: fs_extra::dir::TransitProcess|  {
        println!("File name: {:?}", process_info.file_name);
        show_progress_bar(process_info.total_bytes, process_info.file_bytes_copied);
        dir::TransitProcessResult::ContinueOrAbort
    };

    // Create the Progress Handler. 
    let _norm_handle = |process_info: TransitProcess|  {
        show_progress_bar(process_info.total_bytes, process_info.file_bytes_copied);
        dir::TransitProcessResult::ContinueOrAbort
    };

    //Iterate each JSON object key and value.
    for data in _data {
        for _file in &data.files{

            let source_path = Path::new(&_file.current_file_path);
            let mut destination_path = Path::new(&_file.export_path);

            let override_path = create_timestamped_folder_if_requested(data.create_new_folder, destination_path);
            destination_path = &override_path;

            //check if is individual file.
            if source_path.is_file(){

                // Print info to user about what is gonna copied, where.
                println!("\n\ncopy file {:?} from: {:?} to directory: {:?}", source_path.file_name().unwrap(), source_path, destination_path);
                
                // Problem was the forgotten "thread::sleep", kudos to VangelisP.
                single_file_copy_with_progress_bar(&source_path, &destination_path.join(source_path.file_name().unwrap()))?;
                
            // else check if we have a whole directory
            } else if source_path.is_dir() {

                /*  
                    User provided a whole directory to be copied. 
                    Print info to user about what is gonna copied, where and
                    iterate the files of the directory and copy them recursively.

                */
                println!("\n\ncopying content of Folder: {:?} \n    from: {:?} to directory: {:?}", source_path.file_name().unwrap(), source_path, destination_path);
                copy_items_with_progress(&[source_path], &destination_path, &options, _norm_handle)?;

            } else {
                // If it is not a file or a directory, then surely we have a wrong dir path.
                display_wrong_path_error("current_file_path".to_string(), source_path);
            }
        }   
    }
    Ok(())
}