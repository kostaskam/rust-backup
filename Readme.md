# rust-backup is a tool created to keep backups.
It parses the paths of the items that are to be saved from data.json and saves them to the "Export_Path" you give it in the json.
data.json shall be in the same directory with the executable.

data.json: 
    **"file": "a",**  
<pre>        _// the filename or a short description, optional._  </pre>
    **"current_file_path": "C:/Users/User/Documents/test.exe",**  
<pre>        _// the path for the file you want to keep a backup._  </pre>
    **"export_path": "C:/Users/User/Documents/Backups",**  
<pre>        _// where to keep the backup._  </pre>
    **"create_new_folder": 1**  
<pre>        _// decide if it will create a new folder with date, hour and minute timestamp or not._  </pre>
