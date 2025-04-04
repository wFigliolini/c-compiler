use std::fs;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let _file_content =
        fs::read_to_string(&config.file_path).map_err(|_| "Failed to read the file")?;

    Ok(())
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

    #[test]
    fn test_run() {
        let config = Config {
            file_path: "test_files/test_file_1.c".to_string(),
        };
        let result = run(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_no_file_path() {
        let config = Config {
            file_path: "".to_string(),
        };
        let result = run(config);
        assert!(result.is_err());
    }
}
