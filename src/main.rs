use std::io::{self};
extern crate clap;
use clap::{App, Arg};
use file_data_splitter::run;

fn main() -> io::Result<()> {
    let validate_usize = |value: String| match value.parse::<usize>() {
        Ok(_) => Ok(()),
        _ => Err(format!("Value have to be a number, not '{}'.", &value)),
    };
    let version: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    let authors: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
    let matches = App::new("File Data Splitter")
        .version(version.unwrap())
        .author(authors.unwrap())
        .about("Split lines from a source file in folder and file structure.")
        .long_about(
            &format!(
                "{}\n\n{}\n{}\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}",
                "Split lines from a source file in folder and file structure.",
                "For example: the line '415ab40ae9b7cc4e66d6769cb2c08106e8293b48'",
                "will saved in the file '<OUTPUT_FOLDER>/415/ab.txt'.",
                "The used part '415ab' is removed and the remaining line",
                "is written '40ae9b7cc4e66d6769cb2c08106e8293b48'.",
                "Hint: The source lines should be sorted. If the file is sorted,",
                "each target file is written only once and does not need to be opened again and again.",
                "Hint: There is no different handling for upper and lower case.",
                "All letters have to be in one case or the file system have to handle both cases.",
                "Limits: The line length have to be at least <directory-length> + <file-length>.",
                "In the example these are 5 signs. The splitting is done by signs, not by graphemes!"
            )[..],
        )
        .arg(
            Arg::with_name("directory-length")
                .short("d")
                .long("directory-length")
                .value_name("NUMBER OF CHARS")
                .help("Sets the length of the directory names")
                .takes_value(true)
                .default_value("3")
                .validator(validate_usize),
        )
        .arg(
            Arg::with_name("file-length")
                .short("f")
                .long("file-length")
                .value_name("NUMBER OF CHARS")
                .help("Sets the length of the file names")
                .takes_value(true)
                .default_value("2")
                .validator(validate_usize),
        )
        .arg(
            Arg::with_name("buffer-size")
                .short("b")
                .long("buffer-size")
                .value_name("BYTES")
                .help("Defines the buffer size for the file write buffer.")
                .takes_value(true)
                .default_value("32768")
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
                .index(2)
                .default_value("file_output"),
        )
        .get_matches();

    let target_folder = matches.value_of("OUTPUT_FOLDER").unwrap();
    let source = matches.value_of("INPUT_FILE").unwrap();
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
    let buffer_size = matches
        .value_of("buffer-size")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    run(
        &source,
        &target_folder,
        folder_length,
        file_length,
        buffer_size,
    )
}
