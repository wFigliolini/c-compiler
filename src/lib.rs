pub struct Config{
    pub file_path: String,
}

impl Config{
    pub fn build(mut args :impl Iterator<Item=String>) -> Result<Config, &'static str>{
        args.next();
        let file_path = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        Ok(Config{file_path})
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_build() {
        let args = vec!["program_name".to_string(), "file_path".to_string()];
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config.file_path, "file_path");
    }

    #[test]
    fn test_config_build_no_file_path() {
        let args = vec!["program_name".to_string()];
        let config = Config::build(args.into_iter());
        assert!(config.is_err());
    }
}