mod sorter;
mod file_format;

use std::path::Path;
use sorter::Sorter;

 
const DOWNLOAD_FOLDER: &str = ".\\Download";

fn main() {
    let path = Path::new(DOWNLOAD_FOLDER);
    if !path.is_dir() {
        return;
    }

    let mut sorter = Sorter::new(DOWNLOAD_FOLDER);
    sorter.register_file_format("txt", "Text")
        .expect("Cannot register a new file format!");


    let _ = sorter.start();
}
