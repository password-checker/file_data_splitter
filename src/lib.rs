#![crate_name = "file_data_splitter"]

use std::fs::{create_dir, read_dir, File, OpenOptions};
use std::io::{
    prelude::{BufRead, Write},
    BufReader, BufWriter, Error, ErrorKind,
};
use std::path::Path;
use std::time::Instant;

#[inline(always)]
fn create_folder_path(parent: &str, name: &str) -> String {
    let mut s = parent.to_owned();
    s.push('/');
    s.push_str(name);
    s
}

#[inline(always)]
fn create_file_name(parent: &str, name: &str) -> String {
    let mut s = parent.to_owned();
    s.push('/');
    s.push_str(name);
    s.push_str(".txt");
    s
}

#[inline]
fn create_folder_by_path(folder: &str) -> Result<(), Error> {
    let path = Path::new(&folder);
    if !path.exists() {
        create_dir(path).expect(&format!("Unable to create folder {}", &folder));
        println!("Create folder {}", folder);
    };
    Ok(())
}

#[inline]
fn create_folder(parent: &str, name: &str) -> Result<String, Error> {
    let p = create_folder_path(parent, name);
    create_folder_by_path(&p).expect(&format!("Unable to create folder {}", p));
    Ok(p)
}

#[inline]
fn open_file(parent: &str, name: &str, buffer_size: usize) -> Result<BufWriter<File>, Error> {
    let p = create_file_name(parent, name);
    //println!("Open file {}", p);
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&p)
        .expect(&format!("Unable to create file {}", &p));
    let writer = BufWriter::with_capacity(buffer_size, file);
    Ok(writer)
}

#[inline]
fn directory_is_empty(path: &str) -> Result<(), Error> {
    let mut paths = read_dir(path)?;
    match paths.next() {
        Some(_) => Err(Error::from(ErrorKind::AlreadyExists)),
        None => Ok(()),
    }
}

pub fn validate_usize(value: String) -> Result<(), String> {
    match value.parse::<usize>() {
        Ok(_) => Ok(()),
        _ => Err(format!(r#"Value have to be a number, not "{}"."#, &value)),
    }
}

pub fn map_eol(value: &str) -> &str {
    match value {
        "CR+LF" => "\r\n",
        "LF" => "\n",
        "CR" => "\r",
        "VT" => "\u{000B}",
        "FF" => "\u{000C}",
        "NEL" => "\u{0085}",
        "LS" => "\u{2028}",
        "PS" => "\u{2029}",
        _ => "\n",
    }
}

/// Run file splitting.
///
/// # Arguments
///
/// * `source` - The data source file
///
/// * `target_folder` - The target folder. Is created if it does not exist. Must be empty.
///
/// * `folder_lenght` - The length of the folder names.
///
/// * `file_length` - The length of the file names.
///
/// * `buffer_size` - The buffer size in bytes to write the target files.
///
/// * `eol` - The end of line sign(s).
///
pub fn run(
    source: &str,
    target_folder: &str,
    folder_length: usize,
    file_length: usize,
    buffer_size: usize,
    eol: &str,
) -> Result<(), Error> {
    let start_lookup = Instant::now();

    // create target folder
    create_folder_by_path(target_folder)
        .expect(&format!("Could not create base folder {}", target_folder));
    directory_is_empty(target_folder).expect(&format!("Directory {} is not empty", target_folder));

    // variables for looping
    let mut current_folder = String::new(); // current used folder name
    let mut current_file = String::new(); // current used file name
    let mut opened_file: Option<BufWriter<File>> = Option::None; // BufWriter (opened file)

    // precalculated variables for loop
    let s1 = folder_length;
    let s2 = folder_length + file_length;

    // loop tru every line
    let source_file = File::open(source).expect(&format!("Could not open source file {}", source));
    let reader = BufReader::new(source_file);
    for l in reader.lines() {
        let line = l?;

        // split
        let folder = &line[..s1];
        let file = &line[s1..s2];
        let value = line[s2..].as_bytes();
        let eol = eol.as_bytes();

        // create folder if needed, if folder is changed (from prio loop pass) current_file will be deleted
        let created_folder = if current_folder.eq(folder) {
            // current folder is the same as last loop pass: only build correct path
            create_folder_path(target_folder, folder)
        } else {
            // folder has changed: create (is needed) folder
            current_file = String::new();
            create_folder(target_folder, folder).expect(&format!(
                "Could not create folder {} {}",
                target_folder, folder
            ))
        };

        // open correct file
        let mut writer = if current_file.eq(file) {
            // file is already opened: reuse BufWriter (and add a new line sign)
            let mut w = opened_file.unwrap();
            w.write_all(eol).expect("Unable to write newline");
            w
        } else {
            if opened_file.is_some() {
                // here flush before dropping to find errors; automatic flush before dropping catches all errors
                opened_file.unwrap().flush()?;
            }
            // file is new to open: open
            open_file(&created_folder, file, buffer_size)
                .expect(&format!("Could not open file {} {}", &created_folder, file))
        };

        // write line
        writer
            .write_all(value)
            .expect(&format!("Unable to write data {}", file));

        // set values for next loop pass
        opened_file = Some(writer);
        current_file = String::from(file);
        current_folder = String::from(folder);
    }

    let elapsed = start_lookup.elapsed();
    println!("Duration {:?}", elapsed);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_usize() -> Result<(), String> {
        validate_usize("2".to_string()).unwrap();
        validate_usize("0".to_string()).unwrap();
        validate_usize("99".to_string()).unwrap();
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_validate_usize_err1() {
        validate_usize("".to_string()).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_validate_usize_err2() {
        validate_usize("-1".to_string()).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_validate_usize_err3() {
        validate_usize("A".to_string()).unwrap();
    }
}
