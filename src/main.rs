//Here we configure the compilation. We are allowing dead code and unused imports for the dev instance only
//This means that we wont get warning during compilation about unused imports.
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::array;
use std::borrow::Borrow;
use std::fs::{self, File, copy};
use std::path::{Path, PathBuf};
use chrono::{self, Local, DateTime, TimeZone, Datelike, Timelike};
use std::io::{self, Read, Write, BufReader};
use std::thread;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use std::env::{self, current_exe};

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    
    file: String,
    //The following is for multiple json values (values from object)
    //Current_Path: Vec<String>,
    current_file_path: String,
    export_path: String,
    create_new_folder: bool
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

    //Single JSON parsing.
    //let person: Person = serde_json::from_str(&json_data).with_context(|| "Failed to parse JSON")?;
    
    /*Multiple JSON object parsing.
    E.G:
        [
            {
                "file": "a"
            },
            {
                "file": "b",
            }
        ]
    */
    let items: Vec<Item> = serde_json::from_str(&json_data).expect("Failed to parse JSON");

    //Iterate each JSON object key and value.
    for item in items {

        let source_path = Path::new(&item.current_file_path);
        let destination_path = Path::new(&item.export_path);

        if item.create_new_folder { 
            let current_date = Local::now();

            let formatted_year = current_date.year().to_string();
            let formatted_month = format!("{:02}", current_date.month());
            let formatted_day = format!("{:02}", current_date.day());
            let formatted_hour = format!("{:02}", current_date.hour());
            let formatted_minute = format!("{:02}", current_date.minute());
            
            // Create the new folder name with date format dd_mm_yyyy_hh_mm.
            let folder_to_create = destination_path.join( format!("{}_{}_{}_{}_{}", 
                                                                formatted_day, 
                                                                formatted_month, 
                                                                formatted_year,
                                                                formatted_hour,
                                                                formatted_minute));     
            //Check if folder not exists, then create it.
            if !folder_to_create.exists() {
                println!("Folder {:?} does not exist!",folder_to_create);
                println!("Creating folder..");
                create_directory(&folder_to_create)?;

            }

            //check if is individual file.
            if source_path.is_file(){

                // Print info to user about what is gonna copied, where.
                println!("\n\ncopy file {:?} from: {:?} to directory: {:?}", item.file, source_path, folder_to_create);
                println!("{:?}",source_path.file_name().unwrap());
                
                // Problem was the forgotten "thread::sleep", kudos to VangelisP.
                progress_bar(&source_path, &folder_to_create.join(source_path.file_name().unwrap()))?;
                
            // else check if we have a whole directory
            } else if source_path.is_dir() {

                /*  
                    User provided a whole directory to be copied. 
                    Iterate the files of the directory and copy them.
                */
                let files = fs::read_dir(source_path).unwrap();
                for file in files {

                    let file_borrow = file.as_ref().unwrap();
                    let path = file_borrow.path();
                    let filename = path.file_name().unwrap();
                    let file_path = file.unwrap().path();
                    println!("\n\ncopy from: {:?} to directory: {:?}", file_path, folder_to_create.join(filename));
                    progress_bar(&file_path, &folder_to_create.join(filename))?; 
                }
            } else {
                display_wrong_path_error("current_file_path".to_string(), source_path);
            }

        } else {
            println!("copy file {:?} from: {:?} to directory: {:?}", item.file, source_path, destination_path);
            /* TODO:
                BUG: Check why we get 'Error: Access Denied (os error 5)'
                Maybe it has o do with the File::create.
            */
            progress_bar(&source_path, destination_path)?;
        }
        
    }
    Ok(())
}


// !!!!!!! TODO: try to move functions into individual .rs files, so we have a "clean" main.rs file. !!!!!!!


/* 
    Parameters: 
        key: String,
        source_path: &Path

    Description: 
        Below function prints an error message about incorrect values.
*/
fn display_wrong_path_error(key: String, source_path: &Path) {
    
    let error_message = format!(" Make sure {key} value is correct and exists: ");
    let error_message_len = error_message.len();
    let source_path_len = source_path.display().to_string().len();
    let total_str_size = error_message_len+source_path_len+5;
    let row_with_dashes = (0..total_str_size).map(|_| "-").collect::<String>();

    println!("\n\n{}", row_with_dashes);
    println!("|{}{:?} |", error_message, source_path);
    println!("{}", row_with_dashes);
}

/* TODO: 
    BUG: Check why we get 'Error: Access Denied (os error 5)'
    Maybe it has o do with the File::create.
*/
fn progress_bar(source_path: &Path, destination_path: &Path) -> io::Result<()> {
    let source_file = File::open(source_path)?;
    let metadata = fs::metadata(source_path)?;
    let total_size = metadata.len();
    
    let mut source_reader = BufReader::new(source_file);
    let mut destination_file = File::create(destination_path)?;//fs::create_dir(destination_path);//File::create(destination_path)?;
    
    let progress_bar = ProgressBar::new(total_size);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta})")
            .unwrap(),
    );
    
    let mut buffer = [0; 1024]; // Buffer for reading and writing file data
    
    loop {
        let bytes_read = source_reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file reached
        }
        
        destination_file.write_all(&buffer[..bytes_read])?;
        progress_bar.inc(bytes_read as u64);
    }
    
    progress_bar.finish_with_message("File copied successfully");
    
    Ok(())
}

/* TODO: Check if we will use the following 2 fn's or not. */
fn copy_file(source: &Path, destination: &PathBuf) -> Result<()> {
    fs::copy(source, destination)?;
    Ok(())
}

fn create_directory(path: &PathBuf) -> Result<()> {
    fs::create_dir(path)?;
    Ok(())
}