/*
Donald Whitehead
CS-339R-601 Fall 2023
Portfolio Project

Rust File Manager

Lib - Contains rust native structs and functions for storing and accessing file system data.

icons provided by By GNOME Project, CC BY-SA 3.0 us, https://commons.wikimedia.org/w/index.php?curid=4339610
*/

use slint::Image;
use std::ffi::OsStr;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

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
    // This is completely necessary. I found a strange bug with Slint's Image implementation in rust
    // in which if I tried to dynamically load the icons at the time of discovery down in my match
    // statment it would crash with caching errors ever second directory change, the only fix I could
    // find for this was statically loading all possible  icons at the start of directory build.
    let img = Image::load_from_path(Path::new("icons/image.png")).unwrap();
    let vid = Image::load_from_path(Path::new("icons/video.png")).unwrap();
    let aud = Image::load_from_path(Path::new("icons/audio.png")).unwrap();
    let exc = Image::load_from_path(Path::new("icons/executable.png")).unwrap();
    let doc = Image::load_from_path(Path::new("icons/document.png")).unwrap();
    let arc = Image::load_from_path(Path::new("icons/archive.png")).unwrap();
    let file = Image::load_from_path(Path::new("icons/file_icon.png")).unwrap();
    let dir = Image::load_from_path(Path::new("icons/dir_icon.png")).unwrap();
    // buid directory
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
                    icon = dir.clone();
                    true
                } else {
                    match path.extension().and_then(OsStr::to_str) {
                        Some(x) => match x {
                            "png" | "jpeg" | "jpg" | "gif" | "bmp" | "tif" | "tiff" => {
                                icon = img.clone()
                            }
                            "mp4" | "mov" | "avi" | "wmv" | "web" | "mkv" => icon = vid.clone(),
                            "m4a" | "mp3" | "wav" | "wma" | "aac" | "ogg" | "pcm" | "alac"
                            | "flac" => icon = aud.clone(),
                            "exe" | "bat" | "cmd" | "app" | "bin" | "osx" | "run" | "AppImage" => {
                                icon = exc.clone()
                            }
                            "doc" | "docx" | "odt" | "pdf" | "ppt" | "pptx" | "xls" | "xlsx"
                            | "ods" | "txt" | "md" => icon = doc.clone(),
                            "zip" | "rar" | "7z" | "tar" | "gz" | "tgz" | "bz2" | "dmg" => {
                                icon = arc.clone()
                            }
                            _ => icon = file.clone(),
                        },
                        None => icon = file.clone(),
                    }

                    false
                };
                FSComponent {
                    name,
                    icon,
                    parent: path.parent().map(|p| p.to_path_buf()),
                    dir,
                }
            })
            .collect());
    } else {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "File System Object is of an unknown type",
        ))
    }
}
