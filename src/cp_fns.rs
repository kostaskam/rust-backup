use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{self, Read, Write, BufReader};
use chrono::*;
use fs_extra::{dir};
use indicatif::{ProgressBar, ProgressStyle};

//testing phase begin

pub fn show_progress_bar(total_size: u64, size_written: u64) {
    let progress_bar = ProgressBar::new(total_size);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            //.template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta})")
            .unwrap()
    );

    progress_bar.set_message("Processing...");
    progress_bar.set_position(size_written);
    progress_bar.finish_with_message("Task complete!");
}


pub fn single_file_copy_with_progress_bar(source_path: &Path, destination_path: &Path) -> io::Result<()> {
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
    }
    
    progress_bar.finish_with_message("File copied successfully");
    
    Ok(())
}

pub fn create_timestamped_folder_if_requested(create_new_folder: bool, destination_path: &Path) -> PathBuf {
    if create_new_folder { 
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
            dir::create(&folder_to_create, false);
        }
        folder_to_create
    }
    else {
            destination_path.to_path_buf()
    }
    
}