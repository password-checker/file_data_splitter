# File Data Splitter ![Rust](https://github.com/password-checker/file_data_splitter/workflows/Rust/badge.svg)

```
File Data Splitter 0.1.0
Author see https://github.com/password-checker/file_data_splitter
Split lines from a source file in folder and file structure.

For example: the line '415ab40ae9b7cc4e66d6769cb2c08106e8293b48'
will saved in the file '<OUTPUT_FOLDER>/415/ab.txt'.
The used part '415ab' is removed and the remaining line
is written '40ae9b7cc4e66d6769cb2c08106e8293b48'.

Limit: The source lines must be sorted.
When the file is opened for writing, the file is recreated.

Limit: There is not handling for upper and lower case.
All letters have to be in one case or the file system have to handle both cases.

Limit: The line length have to be at least <directory-length> + <file-length>.
In the example these are 5 signs.

USAGE:
    file_data_splitter [OPTIONS] <INPUT_FILE> [OUTPUT_FOLDER]

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


OPTIONS:
    -d, --directory-length <LENGTH>    
            Sets the length of the directory names [default: 3]

    -f, --file-length <LENGTH>         
            Sets the length of the file names [default: 2]


ARGS:
    <INPUT_FILE>       
            Sets the input file to use

    <OUTPUT_FOLDER>    
            Sets the output folder to use [default: file_output]
```
