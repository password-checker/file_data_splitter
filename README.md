# File Data Splitter ![Rust](https://github.com/password-checker/file_data_splitter/workflows/Rust/badge.svg)

```
File Data Splitter 0.1.0
Author see https://github.com/password-checker/file_data_splitter
Split lines from a source file in folder and file structure.

For example: the line "415ab40ae9b7cc4e66d6769cb2c08106e8293b48" will saved in
the file "<OUTPUT_FOLDER>/415/ab.txt". The used part "415ab" is removed and the
remaining line is written "40ae9b7cc4e66d6769cb2c08106e8293b48".

Hint: The source lines should be sorted. If the file is sorted, each target
file is written only once and does not need to be opened again and again.

Hint: There is no different handling for upper and lower case. All letters have
to be in one case or the file system have to handle both cases.

Limits: The line length have to be at least <directory-length> + <file-length>.
In the example these are 5 signs. The splitting is done by signs, not by 
graphemes!

USAGE:
    file_data_splitter [OPTIONS] <INPUT_FILE> [OUTPUT_FOLDER]

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


OPTIONS:
    -b, --buffer-size <BYTES>                   
            Defines the buffer size for the file write buffer. [default: 32768]

    -d, --directory-length <NUMBER OF CHARS>    
            Sets the length of the directory names [default: 3]

    -e, --eol <EOL SEQUENCE>                    
            Defines the newline sequence. [default: LF]  [possible values: LF, CR+LF, CR, VT, FF, NEL, LS, PS]

    -f, --file-length <NUMBER OF CHARS>         
            Sets the length of the file names [default: 2]


ARGS:
    <INPUT_FILE>       
            Sets the input file to use

    <OUTPUT_FOLDER>    
            Sets the output folder to use. Is created if it does not exist. Must be empty. [default: file_output]
```
