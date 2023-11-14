/*
Donald Whitehead
CS-339R-601 Fall 2023
Portfolio Project: First checkin

Rust File Manager

Lib - Contains rust native structs and functions for storing and accessing file system data.
*/


use std::{
    fs,
    path::{Path, PathBuf},
    io,
};
use slint::Image;

// Rust native version of the FSComponent struct
#[derive(Clone, Debug)]
pub struct FSComponent {
    pub name: String,
    pub icon: Image,
    pub parent: Option<PathBuf>,
    pub dir: bool,
}


// Reads a directory and biulds a vector of FSComponents that represent the items in the directory
pub fn get_dir_conts(p: &Path) -> Result<Vec<FSComponent>, io::Error> {
    if fs::metadata(p)?.is_dir() {
        return Ok(fs::read_dir(p)?
            .filter(|x| x.is_ok())
            .map(|x| {
                let icon: Image;
                let path = x.unwrap().path();
                let name = path
                    .file_name()
                    .expect("Couldn't read file name")
                    .to_str()
                    .expect("to stirng failed")
                    .to_owned();
                let dir = if fs::metadata(&path).unwrap().is_dir() { 
                    icon = Image::load_from_path(Path::new("icons/dir_icon.png")).unwrap();
                    true 
                } else { 
                    icon = Image::load_from_path(Path::new("icons/file_icon.png")).unwrap();
                    false 
                };
                FSComponent {
                    name,
                    icon,
                    parent: path.parent().and_then(|p| Some(p.to_path_buf())),
                    dir,
                }
            })
            .collect());
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "File System Object is of an unknown type",
        ));
    }
}
