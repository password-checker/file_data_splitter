use std::io::{self};
extern crate clap;
use clap::{App, Arg};
use file_data_splitter::run;

fn main() -> io::Result<()> {
    let validate_usize = |value: String| match value.parse::<usize>() {
        Ok(_) => Ok(()),
        _ => Err(format!("Value have to be a number, not '{}'.", &value)),
    };
    let matches = App::new("File Data Splitter")
        .version("0.1.0")
        .author("Ariel Kuechler <github.ariel@kuechler.info>")
        .about("Splitt lines from a file in folder and file structure.")
        .arg(
            Arg::with_name("directory-length")
                .short("d")
                .long("directory-length")
                .value_name("LENGTH")
                .help("Sets the length of the directory names")
                .takes_value(true)
                .default_value("3")
                .validator(validate_usize),
        )
        .arg(
            Arg::with_name("file-length")
                .short("f")
                .long("file-length")
                .value_name("LENGTH")
                .help("Sets the length of the file names")
                .takes_value(true)
                .default_value("2")
                .validator(validate_usize),
        )
        .arg(
            Arg::with_name("INPUT_FILE")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT_FOLDER")
                .help("Sets the output folder to use")
                .required(false)
                .index(2),
        )
        .get_matches();

    let target_folder = matches.value_of("OUTPUT_FOLDER").unwrap_or("file_output");
    let source = matches.value_of("INPUT_FILE").unwrap_or("test.txt");
    let folder_length = matches
        .value_of("directory-length")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let file_length = matches
        .value_of("file-length")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    run(&source, &target_folder, folder_length, file_length)
}
