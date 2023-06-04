# rust-backup is a tool created to keep backups.
It parses the paths of the items that are to be saved from data.json and saves them to the "Export_Path" you give it in the json.
data.json shall be in the same directory with the executable.

data.json:  
    **"file": "a",**  
        _the filename or a short description, optional._  
    **"current_file_path": "C:/Users/User/Documents/test.exe",**  
        _the path for the file you want to keep a backup._  
    **"export_path": "C:/Users/User/Documents/Backups",**  
        _where to keep the backup._  
    **"create_new_folder": 1**  
        _decide if it will create a new folder with date, hour and minute timestamp or not._  
