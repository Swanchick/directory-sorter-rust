use std::{thread, time::Duration};
use std::{fs, path::PathBuf};
use std::io;

use crate::file_format::FileFormat;

pub struct Sorter {
    path: PathBuf,
    work: bool,
    formats: Vec<FileFormat>
}

impl Sorter {
    pub fn new(path: &str) -> Sorter {
        let path = PathBuf::from(path);
        
        Sorter {
            path: path,
            work: true,
            formats: vec![]
        }
    }

    fn create_folder_path(&self, folder: &str) -> Option<String> {
        let path = self.path.to_str()?;
        let mut path_string = path.to_string();
        path_string.push('\\');
        path_string.push_str(folder);

        Some(path_string)
    }

    fn create_folder_path_with_file(&self, folder: &str, file: &str) -> Option<String> {
        let path = self.path.to_str()?;
        let mut path_string = path.to_string();
        path_string.push('\\');
        path_string.push_str(folder);
        path_string.push('\\');
        path_string.push_str(file);

        Some(path_string)
    }

    fn get_file_name<'a>(&self, file_path: &'a str) -> Option<&'a str> {
        let file_splitted: Vec<&str> = file_path.split('\\').collect();
        let file_name = file_splitted.last().copied()?;

        Some(file_name)
    }

    fn get_format_from_file<'a>(&self, file_name: &'a str) -> Option<&'a str> {
        let file_splitted: Vec<&str> = file_name.split(".").collect();
        let file_format = file_splitted.last().copied()?;

        Some(file_format)
    }

    fn move_file(&self, file_path: &str) {
        let file_name = self.get_file_name(&file_path)
            .expect("Cannot get file name!");

        let file_format = self.get_format_from_file(file_name)
            .expect("Cannot get format!");

        match self.get_format(file_format) {
            Some(format) => {
                let folder_name = &format.folder_name;
                let folder_path_string = self.create_folder_path(&folder_name)
                    .expect("Cannot create path!");

                let folder_path = PathBuf::from(&folder_path_string);

                if !folder_path.is_dir() {
                    let _ = fs::create_dir(&folder_path_string);
                }
                
                let new_file_path = self.create_folder_path_with_file(&folder_name, file_name)
                    .expect("Cannot create path!");

                let _ = fs::rename(file_path, new_file_path);
            }
            None => {

            }
        }
    }

    fn check_files(&self) -> Result<(), io::Error> {
        for file in fs::read_dir(&self.path)? {
            let file = file?;
            
            match file.path().to_str() {
                Some(path) => {
                    self.move_file(path);
                },
                None => {}
            }
        }

        Ok(())
    }

    pub fn start(&mut self) -> Result<(), io::Error> {
        if self.path.is_dir() {
            while self.work {
                self.check_files()?;
    
                thread::sleep(Duration::from_millis(1000));
            }
    
            Ok(())
        } else {
            println!("Directory is not found!");
            
            Err(io::Error::new(io::ErrorKind::NotFound, "Directory not found!"))
        }
    }

    fn check_file_format(&self, file_format: &str) -> bool {
        for f in &self.formats {
            if f.format == file_format {
                return true;
            }
        }
        
        false
    }

    fn get_format(&self, file_format: &str) -> Option<&FileFormat> {
        for f in &self.formats {
            if f.format == file_format {
                return Some(f);
            }
        }

        None
    }

    pub fn register_file_format(&mut self, file_format: &str, folder_name: &str) -> Result<(), io::Error> {
        if self.check_file_format(file_format) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "This file is already exists!"));
        }
        
        let f = FileFormat {
            format: file_format.to_string(),
            folder_name: folder_name.to_string()
        };

        self.formats.push(f);

        Ok(())
    }
}
