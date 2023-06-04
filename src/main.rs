//Here we configure the compilation. We are allowing dead code and unused imports for the dev instance only
//This means that we wont get warning during compilation about unused imports.
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use anyhow::{Result, Context};
use chrono::Timelike;
use serde::{Deserialize, Serialize};
use std::array;
use std::fs;
use std::fs::File;
use std::fs::copy;
use std::path::{Path, PathBuf};
use chrono::{Local, DateTime, TimeZone, Datelike};
use chrono;
use std::io::Read;

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

        /*todo:
            1. add new key-value pair: create_new_folder
                Possible values: 1 or 0.
                The purpose of this key will be to create a new folder with date and time stamp and save it inside there.
                Format: mm_dd_yyyy_hh_mm
                E.g: 
                        06_04_2023_04_42
                        06_04_2023_04_43                      
        */
        let source_path = Path::new(&item.current_file_path);
        let destination_path = Path::new(&item.export_path);//.join(source_path.file_name().unwrap());

        if item.create_new_folder == 1 { 
            println!("{:?} -- {:?} -- {:?} -- {:?}", item.file, item.current_file_path, item.export_path, item.create_new_folder);
            
            //let current_date = chrono::Utc::now();
            let current_date = Local::now();

            //let formatted_year = format!("{:04}",current_date.year());
            let formatted_year = current_date.year().to_string();
            let formatted_month = format!("{:02}", current_date.month());
            let formatted_day = format!("{:02}", current_date.day());
            let formatted_hour = format!("{:02}", current_date.hour());
            let formatted_minute = current_date.minute().to_string();

            let create_folder = destination_path.join( format!("{}_{}_{}_{}_{}", formatted_day, 
                                                                formatted_month, 
                                                                formatted_year,
                                                                formatted_hour,
                                                                formatted_minute,
                                                                ));     
            //Check if folder not exists, then create it.
            if !create_folder.exists() {
                println!("Folder {:?} does not exist!",create_folder);
                create_directory(&create_folder)?;

            }

            // Last but not least, copy the items to the destination folder.
            copy_file(source_path, &create_folder.join(source_path.file_name().unwrap()))?;
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