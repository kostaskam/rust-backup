# rust-backup is a tool created to keep backups.
It parses the paths of the items that are to be saved from data.json and saves them to the "Export_Path" you give it in the json.
data.json shall be in the same directory with the executable.

data.json:  
    **"file": "a",**  
&nbsp;&nbsp;&nbsp;&nbsp; _the filename or a short description, optional._  
&nbsp;&nbsp;  
    **"current_file_path": "C:/Users/User/Documents/test.exe",**  
&nbsp;&nbsp;&nbsp;&nbsp; _the path for the file you want to keep a backup._  
&nbsp;&nbsp;  
    **"export_path": "C:/Users/User/Documents/Backups",**  
&nbsp;&nbsp;&nbsp;&nbsp; _where to keep the backup._  
&nbsp;&nbsp;  
    **"create_new_folder": 1**  
&nbsp;&nbsp;&nbsp;&nbsp; _decide if it will create a new folder with date, hour and minute timestamp or not._  
