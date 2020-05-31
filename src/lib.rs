use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::path::Path;
use std::time::Instant;

#[inline(always)]
fn create_folder_by_path(folder: &str) -> Result<(), io::Error> {
    let path = Path::new(&folder);
    if !path.exists() {
        fs::create_dir(path).expect(&format!("Unable to create folder {}", &folder));
        println!("Create folder {}", folder);
    };
    Ok(())
}

#[inline(always)]
fn create_folder_path(parent: &str, name: &str) -> String {
    format!("{}/{}", parent, name)
}

#[inline(always)]
fn create_folder(parent: &str, name: &str) -> Result<String, io::Error> {
    let p = create_folder_path(parent, name);
    create_folder_by_path(&p).expect(&format!("Unable to create folder {}", &p));
    Ok(p)
}

#[inline(always)]
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

    // create target folder
    create_folder_by_path(base_folder)
        .expect(&format!("Could not create base folder {}", base_folder));

    // variables for looping
    let mut current_folder = String::from(""); // current used folder name
    let mut current_file = String::from(""); // current used file name
    let mut opened_file: Option<BufWriter<File>> = Option::None; // BufWriter (opened file)

    // loop tru every line
    let source_file = File::open(source).expect(&format!("Could not open source file {}", source));
    let reader = BufReader::new(source_file);
    for l in reader.lines() {
        let line = l?;

        // spit
        let folder = String::from(line.get(..folder_length).unwrap());
        let file = String::from(
            line.get(folder_length..(folder_length + file_length))
                .unwrap(),
        );
        let value = line.get(5..).unwrap();

        // create folder if needed, if folder is changed (from prio loop pass) current_file will be deleted
        let created_folder = if current_folder == folder {
            // current folder is the same as last loop pass: only build correct path
            create_folder_path(base_folder, &folder)
        } else {
            // folder has changed: create (is needed) folder
            current_file = String::from("");
            create_folder(base_folder, &folder).expect(&format!(
                "Could not create folder {} {}",
                base_folder, &folder
            ))
        };

        // open correct file
        let add_newline;
        let mut writer = if current_file == file {
            // file is already opened: reuse BufWriter (and add a new line sign)
            add_newline = true;
            opened_file.unwrap()
        } else {
            // file is new to open: open
            add_newline = false;
            open_file(&created_folder, &file).expect(&format!(
                "Could not open file {} {}",
                &created_folder, &file
            ))
        };

        // add newline (if file is already opened)
        if add_newline {
            writer.write_all(b"\n").expect("Unable to write newline");
        }
        // write line
        writer
            .write_all(value.as_bytes())
            .expect(&format!("Unable to write data {}", &file));

        // set values for next loop pass
        opened_file = Some(writer);
        current_file = file;
        current_folder = folder;
    }

    let elapsed = start_lookup.elapsed();
    println!("Duration {:?}", elapsed);
    Ok(())
}
