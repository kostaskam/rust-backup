//Here we configure the compilation. We are allowing dead code and unused imports for the dev instance only
//This means that we wont get warning during compilation about unused imports.
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::array;
use std::fs::{self, File, copy};
use std::path::{Path, PathBuf};
use chrono::{self, Local, DateTime, TimeZone, Datelike, Timelike};
use std::io::{self, Read, Write, BufReader};
use std::thread;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    
    file: String,
    //The following is for multiple json values (values from object)
    //Current_Path: Vec<String>,
    current_file_path: String,
    export_path: String,
    create_new_folder: u32
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
    }
}

fn run() -> Result<()> {
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
        let destination_path = Path::new(&item.export_path);//.join(source_path.file_name().unwrap());

        if item.create_new_folder == 1 { 
            let current_date = Local::now();

            //let formatted_year = format!("{:04}",current_date.year());
            let formatted_year = current_date.year().to_string();
            let formatted_month = format!("{:02}", current_date.month());
            let formatted_day = format!("{:02}", current_date.day());
            let formatted_hour = format!("{:02}", current_date.hour());
            let formatted_minute = current_date.minute().to_string();

            let folder_to_create = destination_path.join( format!("{}_{}_{}_{}_{}", formatted_day, 
                                                                formatted_month, 
                                                                formatted_year,
                                                                formatted_hour,
                                                                formatted_minute,
                                                                ));     
            //Check if folder not exists, then create it.
            if !folder_to_create.exists() {
                println!("Folder {:?} does not exist!",folder_to_create);
                println!("Creating folder..");
                create_directory(&folder_to_create)?;

            }

            // Leave here for now, but check why it takes so long to copy a 1mb file.
            //progress_bar(&source_path, &folder_to_create.join(source_path.file_name().unwrap()))?;
            // Last but not least, copy the items to the destination folder.
            println!("copy file {:?} from: {:?} to directory: {:?}", item.file, source_path, destination_path);
            copy_file(source_path, &folder_to_create.join(source_path.file_name().unwrap()))?;
        } else {
            println!("copy file {:?} from: {:?} to directory: {:?}", item.file, source_path, destination_path);
            copy_file(source_path, &destination_path.join(source_path.file_name().unwrap()))?;
        }
        
    }
    Ok(())
}

fn copy_file(source: &Path, destination: &PathBuf) -> Result<()> {
    fs::copy(source, destination)?;
    Ok(())
}

fn create_directory(path: &PathBuf) -> Result<()> {
    fs::create_dir(path)?;
    Ok(())
}

// Check why it takes so long to copy-paste a 1mb file.
fn progress_bar(source_path: &Path, destination_path: &PathBuf) -> io::Result<()> {
    let source_file = File::open(source_path)?;
    let metadata = fs::metadata(source_path)?;
    let total_size = metadata.len();
    
    let mut source_reader = BufReader::new(source_file);
    let mut destination_file = File::create(destination_path)?;
    
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
        
        // Simulate delay for each chunk of data copied
        thread::sleep(Duration::from_millis(10));
    }
    
    progress_bar.finish_with_message("File copied successfully");
    
    Ok(())
}