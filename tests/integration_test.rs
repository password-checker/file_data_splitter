#[cfg(test)]
mod tests {
    use file_data_splitter::*;
    use std::fs;
    use std::io::Error;
    use std::path::Path;
    use std::result::Result;

    fn check_file_content(file_path: &str, expected: &str) -> Result<(), Error> {
        let content = fs::read_to_string(file_path).unwrap();
        assert_eq!(content, expected);
        Ok(())
    }

    fn check_exists(path: &str) -> Result<(), Error> {
        assert!(Path::new(path).exists());
        Ok(())
    }

    fn check_folder_count(path: &str, expected: usize) -> Result<(), Error> {
        let folders = fs::read_dir(path).unwrap();
        assert_eq!(folders.count(), expected);
        Ok(())
    }

    #[test]
    fn it_works() -> Result<(), Error> {
        if Path::new("target/test_output").exists() {
            fs::remove_dir_all("target/test_output")?;
        }
        run("tests/test2.txt", "target/test_output", 3, 2)?;

        // check folders
        check_folder_count("target/test_output", 2)?;
        check_exists("target/test_output/000")?;
        check_exists("target/test_output/001")?;

        // check folder 000
        check_folder_count("target/test_output/000", 2)?;
        check_exists("target/test_output/000/14.txt")?;
        check_file_content(
            "target/test_output/000/14.txt",
            "75f955b2594ef549e0a8a32d621063e7a07\n75f955b2594ef549e0a8a32d621063e7a08",
        )?;
        check_exists("target/test_output/000/15.txt")?;
        check_file_content(
            "target/test_output/000/15.txt",
            "75f955b2594ef549e0a8a32d621063e7a08",
        )?;

        // check folder 001
        check_folder_count("target/test_output/001", 1)?;
        check_exists("target/test_output/001/e8.txt")?;
        check_file_content(
            "target/test_output/001/e8.txt",
            "e063a0cea119d0e685d3304b5cb18e6435a",
        )?;

        fs::remove_dir_all("target/test_output")?;
        Ok(())
    }
}
