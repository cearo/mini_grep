struct Config {
    query: String,
    file_path: String,
    file_content: String,
} impl Config {
    fn build(query: String, file_path: String) -> Result<Config, std::io::Error> {
        let read_file = std::fs::read_to_string(&file_path);
        match read_file {
            Ok(file_content) => Ok(Config { query, file_path, file_content }),
            Err(err) => Err(err)
        }
    }
    fn run(&self) -> Vec<&str> {
        self.file_content.lines().into_iter().filter(|&line| line.contains(&self.query)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE_PATH: &str = "../test/resources/test_poem.txt";
    const QUERY_MOCK: &str = "Found";
    const FILE_CONTENT_MOCK: &str = "You Found me! 
        \n You can't find me! \n I've been found!";
    
    // #[test]
    // fn successful_build() {
    //     let test_build = Config::build(query_mock.to_owned(), test_file_path.to_owned());
    //     assert!(test_build.is_ok())
    // }

    #[test]
    fn succesful_run() {
        let test_config = Config {
            query: QUERY_MOCK.to_owned(),
            file_content: FILE_CONTENT_MOCK.to_owned(),
            file_path: TEST_FILE_PATH.to_owned(),
        };

        let test_result = test_config.run();
        assert_eq!(1, test_result.len())
    }
}
