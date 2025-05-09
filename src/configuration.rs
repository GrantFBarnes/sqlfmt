use std::{
    env,
    fs::{self, DirEntry, ReadDir},
    path::{Path, PathBuf},
};

use crate::arguments::Arguments;

const CONFIG_FILE_NAME: &str = ".sqlfmt";

pub struct Configuration {
    pub newlines: bool,
    pub case: ConfigCase,
    pub tabs: ConfigTab,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            newlines: false,
            case: ConfigCase::Unchanged,
            tabs: ConfigTab::Space(4),
        }
    }

    pub fn from(args: &Arguments) -> Configuration {
        let mut config: Configuration = Configuration::new();

        if let Ok(cwd) = env::current_dir() {
            if let Some(file_config) = find_config(cwd.as_path()) {
                config = file_config;
            }
        }

        if args.newlines {
            config.newlines = true;
        }

        if args.upper {
            config.case = ConfigCase::Uppercase;
        } else if args.lower {
            config.case = ConfigCase::Lowercase;
        }

        if args.tabs {
            config.tabs = ConfigTab::Tab;
        }

        if args.spaces.is_some() {
            config.tabs = ConfigTab::Space(args.spaces.unwrap());
        }

        return config;
    }
}

fn find_config(path: &Path) -> Option<Configuration> {
    if !path.is_dir() {
        return None;
    }

    let entries: Result<ReadDir, std::io::Error> = path.read_dir();
    if entries.is_err() {
        return None;
    }

    for entry in entries.unwrap() {
        if entry.is_err() {
            continue;
        }
        let entry: DirEntry = entry.unwrap();
        if entry.file_name() == CONFIG_FILE_NAME {
            return get_file_config(entry.path());
        }
    }

    let parent: Option<&Path> = path.parent();
    if parent.is_none() {
        return None;
    }

    return find_config(parent.unwrap());
}

fn get_file_config(file: PathBuf) -> Option<Configuration> {
    let content: Result<String, std::io::Error> = fs::read_to_string(file);
    if content.is_err() {
        return None;
    }
    let content: String = content.unwrap();

    let mut config: Configuration = Configuration::new();

    for line in content.lines() {
        if line.contains("newlines") {
            config.newlines = true;
        } else if line.contains("upper") {
            config.case = ConfigCase::Uppercase
        } else if line.contains("lower") {
            config.case = ConfigCase::Lowercase
        } else if line.contains("tabs") {
            config.tabs = ConfigTab::Tab;
        } else if line.starts_with("spaces") {
            if let Some(n) = line.split("=").last() {
                if let Ok(spaces) = n.parse::<u8>() {
                    config.tabs = ConfigTab::Space(spaces);
                }
            }
        }
    }

    return Some(config);
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigCase {
    Uppercase,
    Lowercase,
    Unchanged,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigTab {
    Tab,
    Space(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_configuration_empty() {
        let args: Vec<String> = vec![];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();

        let config: Configuration = Configuration::from(&arguments);
        assert_eq!(config.newlines, false);
        assert_eq!(config.case, ConfigCase::Unchanged);
        assert_eq!(config.tabs, ConfigTab::Space(4));
    }

    #[test]
    fn test_get_configuration_newlines() {
        let args: Vec<String> = vec![String::from("-n")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();

        let config: Configuration = Configuration::from(&arguments);
        assert_eq!(config.newlines, true);
        assert_eq!(config.case, ConfigCase::Unchanged);
        assert_eq!(config.tabs, ConfigTab::Space(4));
    }

    #[test]
    fn test_get_configuration_upper() {
        let args: Vec<String> = vec![String::from("-u")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();

        let config: Configuration = Configuration::from(&arguments);
        assert_eq!(config.newlines, false);
        assert_eq!(config.case, ConfigCase::Uppercase);
        assert_eq!(config.tabs, ConfigTab::Space(4));
    }

    #[test]
    fn test_get_configuration_lower() {
        let args: Vec<String> = vec![String::from("-l")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();

        let config: Configuration = Configuration::from(&arguments);
        assert_eq!(config.newlines, false);
        assert_eq!(config.case, ConfigCase::Lowercase);
        assert_eq!(config.tabs, ConfigTab::Space(4));
    }

    #[test]
    fn test_get_configuration_tabs() {
        let args: Vec<String> = vec![String::from("-t")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();

        let config: Configuration = Configuration::from(&arguments);
        assert_eq!(config.newlines, false);
        assert_eq!(config.case, ConfigCase::Unchanged);
        assert_eq!(config.tabs, ConfigTab::Tab);
    }

    #[test]
    fn test_get_configuration_spaces() {
        let args: Vec<String> = vec![String::from("-s"), String::from("2")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();

        let config: Configuration = Configuration::from(&arguments);
        assert_eq!(config.newlines, false);
        assert_eq!(config.case, ConfigCase::Unchanged);
        assert_eq!(config.tabs, ConfigTab::Space(2));
    }
}
