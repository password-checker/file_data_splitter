use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::path::Path;
use std::time::Instant;

#[inline]
fn create_folder_by_path(folder: &str) -> Result<(), io::Error> {
    let path = Path::new(&folder);
    if !path.exists() {
        fs::create_dir(path).expect(&format!("Unable to create folder {}", &folder));
        // println!("Create folder {}", folder);
    };
    Ok(())
}

#[inline]
fn create_folder_path(parent: &str, name: &str) -> String {
    format!("{}/{}", parent, name)
}

#[inline]
fn create_folder(parent: &str, name: &str) -> Result<String, io::Error> {
    let p = create_folder_path(parent, name);
    create_folder_by_path(&p).expect(&format!("Unable to create folder {}", &p));
    Ok(p)
}

#[inline]
fn open_file(parent: &str, name: &str) -> Result<BufWriter<File>, io::Error> {
    let p = format!("{}/{}.txt", parent, name);
    //println!("Open file {}", p);
    let file = File::create(&p).expect(&format!("Unable to create file {}", &p));
    let writer = BufWriter::new(file);
    Ok(writer)
}

pub fn run(
    base_folder: &str,
    source: &str,
    folder_length: usize,
    file_length: usize,
) -> io::Result<()> {
    let start_lookup = Instant::now();
    create_folder_by_path(&base_folder)
        .expect(&format!("Could not create base folder {}", &base_folder));
    let source_file =
        File::open(&source).expect(&format!("Could not open source file {}", &source));
    let reader = BufReader::new(source_file);

    let mut current_folder: Option<String> = Option::None;
    let mut current_file: Option<String> = Option::None;
    let mut opened_file: Option<BufWriter<File>> = Option::None;
    for l in reader.lines() {
        let line = l?;
        //line.make_ascii_uppercase();
        let folder = String::from(line.get(..folder_length).unwrap());
        let file = String::from(
            line.get(folder_length..(folder_length + file_length))
                .unwrap(),
        );
        let value = line.get(5..).unwrap();
        // println!("{} {} {}", folder, file, value);
        let created_folder = match current_folder {
            Some(f) => {
                if f == folder {
                    create_folder_path(&base_folder, &folder)
                } else {
                    current_file = None;
                    create_folder(&base_folder, &folder).expect(&format!(
                        "Could not create folder {} {}",
                        &base_folder, &folder
                    ))
                }
            }
            None => {
                current_file = None;
                create_folder(&base_folder, &folder).expect(&format!(
                    "Could not create folder {} {}",
                    &base_folder, &folder
                ))
            }
        };
        let add_newline;
        let mut writer = match current_file {
            Some(f) => {
                if f == file {
                    add_newline = true;
                    opened_file.unwrap()
                } else {
                    add_newline = false;
                    open_file(&created_folder, &file).expect(&format!(
                        "Could not open file {} {}",
                        &created_folder, &file
                    ))
                }
            }
            None => {
                add_newline = false;
                open_file(&created_folder, &file).expect(&format!(
                    "Could not open file {} {}",
                    &created_folder, &file
                ))
            }
        };
        if add_newline {
            writer.write_all(b"\n").expect("Unable to write newline");
        }
        writer
            .write_all(value.as_bytes())
            .expect(&format!("Unable to write data {}", &file));
        opened_file = Some(writer);
        current_file = Some(file);
        current_folder = Some(folder);
    }
    let elapsed = start_lookup.elapsed();
    println!("Duration {:?}", elapsed);
    Ok(())
}
