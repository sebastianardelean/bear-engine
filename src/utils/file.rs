use std::path::Path;
use std::{fs, io};

pub fn check_if_file_exists(file_path: &Path) -> io::Result<bool> {
    match fs::metadata(file_path) {
        Ok(meta) => {
            if meta.is_file() {
                Ok(true) //file exists
            } else {
                Ok(false) // path exists but no file!
            }
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false), //file does not exist
        Err(e) => Err(e),                                           //other error
    }
}
