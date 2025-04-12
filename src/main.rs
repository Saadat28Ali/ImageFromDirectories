// Built-in crates
use std::io::{self, BufRead};
use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::env;

// Global Variables
static FILTERED_EXTENSIONS: [&str; 10] = [
    "bmp", 
    "jpg", 
    "jpeg", 
    "png", 
    "tiff", 
    "svg", 
    "heif", 
    "gif", 
    "eps", 
    "webp", 
];

fn getFilesFromDirectory(dir: &str) -> Result<Vec::<PathBuf>, String> {

    let dir_entries: fs::ReadDir = match fs::read_dir(dir) {
        Ok(entries) => {
            entries
        }
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let mut subpaths: Vec::<PathBuf> = Vec::<PathBuf>::new();

    for entry in dir_entries {

        let entry_path: PathBuf = match entry {
            Ok(dir_entry) => {
                dir_entry.path()
            }
            Err (_) => {
                return Err("Error while reading dictionaries (4)".to_string());
            }
        };
        
        match entry_path.is_file() {
            true => {
                
                let extension: &OsStr = match entry_path.extension() {
                    Some(ext_os_str) => {ext_os_str}
                    None => {OsStr::new("")}
                };

                let extension: &str = match extension.to_str() {
                    Some(ext_str) => {ext_str}
                    None => {""}
                };

                match FILTERED_EXTENSIONS.contains(&extension) {
                    true => {
                        subpaths.push(entry_path);
                    }
                    false => {}
                }

            }
            false => {
                let entry_path_str: &str = match entry_path.to_str() {
                    Some(s) => {s}
                    None => {""}
                };

                let subsubpaths: Vec::<PathBuf> = match getFilesFromDirectory(entry_path_str) {
                    Ok(paths) => {paths}
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };

                for path in subsubpaths {
                    subpaths.push(path);
                }

            }
        }

    }

    return Ok(subpaths);
}

fn copyFilesToDirectory(filepaths: &Vec::<PathBuf>, output_dir: &PathBuf) -> Result<bool, String>{

    for file in filepaths {
        let filename: &OsStr = match file.file_name() {
            Some(name) => {name}
            None => {&OsStr::new("")}
        };

        let filename: String = filename.to_string_lossy().to_string();
        let mut output_filepath: PathBuf = output_dir.clone();
        output_filepath.push(filename);

        match output_dir.try_exists() {
            Ok(b) => {
                
                match b {
                    true => {}
                    false => {
                        match fs::create_dir(output_dir) {
                            Ok(()) => {}
                            Err(e) => {
                                return Err(e.to_string());
                            }
                        }
                    }
                }

            }
            Err(e) => {
                return Err(e.to_string());
            }
        }

        match fs::copy(file, output_filepath) {
            Ok(_) => {}
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    return Ok(true);

}

fn readLine(hint: &str) -> Result<String, String> {

    println!("{}", hint);

    let stdin: io::Stdin = io::stdin();
    let mut handle: io::StdinLock = stdin.lock();
    let mut read_bytes: Vec::<u8> = Vec::<u8>::new();
    let mut read_string: String = String::new();

    match handle.read_until(b'\n', &mut read_bytes) {
        Ok(_bytes_read) => {
            read_string = match String::from_utf8(read_bytes) {
                    Ok(s) => {s}
                    Err(_) => {
                        return Err("Error occurred while reading stdin (1)".to_string());
                    }
            };
        }
        Err(_) => {
            return Err("Error occurred while reading stdin (2)".to_string());
        }
    }

    read_string = read_string.trim_end().to_string();
    return Ok(read_string);

}

fn main() {

    let search_dir_string: String = match readLine("Searching this directory: ") {
        Ok(s) => {s}
        Err(e) => {
            eprintln!("{}", e);
            "".to_string()
        }
    };
    let output_dir_string: String = match readLine("Output into this directory: ") {
        Ok(s) => {s}
        Err(e) => {
            eprintln!("{}", e);
            "".to_string()
        }
    };

    let output_dir: PathBuf = PathBuf::from(&output_dir_string);

    let files: Vec::<PathBuf> = match getFilesFromDirectory(&search_dir_string) {
        Ok(filepaths) => {
            filepaths
        }
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };


    match copyFilesToDirectory(&files, &output_dir) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e.to_string());
        }
    }
}