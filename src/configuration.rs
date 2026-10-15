use std::{
    env,
    fs::{self, DirEntry, ReadDir},
    path::{Path, PathBuf},
};

use crate::arguments::Arguments;

const CONFIG_FILE_NAME: &str = ".sqlfmt";

pub struct Configuration {
    pub newlines: bool,
    pub comment_pre_space: bool,
    pub align_text_groups: bool,
    pub case: ConfigCase,
    pub tabs: ConfigTab,
    pub chars: u8,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            newlines: false,
            comment_pre_space: false,
            align_text_groups: false,
            case: ConfigCase::Unchanged,
            tabs: ConfigTab::Space(4),
            chars: 80,
        }
    }

    pub fn from(args: &Arguments) -> Configuration {
        let mut config: Configuration = Configuration::new();

        if let Ok(cwd) = env::current_dir()
            && let Some(file_config) = find_config(cwd.as_path())
        {
            config = file_config;
        }

        if args.newlines {
            config.newlines = true;
        }

        if args.comment_pre_space {
            config.comment_pre_space = true;
        }

        if args.align_text_groups {
            config.align_text_groups = true;
        }

        if args.upper {
            config.case = ConfigCase::Uppercase;
        } else if args.lower {
            config.case = ConfigCase::Lowercase;
        }

        if args.tabs {
            config.tabs = ConfigTab::Tab;
        }

        if let Some(spaces) = args.spaces {
            config.tabs = ConfigTab::Space(spaces);
        }

        if let Some(chars) = args.chars {
            config.chars = chars;
        }

        config
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

    if let Some(parent) = path.parent() {
        find_config(parent)
    } else {
        None
    }
}

fn get_file_config(file: PathBuf) -> Option<Configuration> {
    let content: Result<String, std::io::Error> = fs::read_to_string(file);
    if content.is_err() {
        return None;
    }
    let content: String = content.unwrap();

    let mut config: Configuration = Configuration::new();

    for line in content.lines() {
        let (key, value): (&str, Option<&str>) = match line.split_once('=') {
            Some((k, v)) => (k.trim(), Some(v.trim())),
            None => (line.trim(), None),
        };

        match key {
            "newlines" => config.newlines = true,
            "comment_pre_space" => config.comment_pre_space = true,
            "align_text_groups" => config.align_text_groups = true,
            "upper" => config.case = ConfigCase::Uppercase,
            "lower" => config.case = ConfigCase::Lowercase,
            "tabs" => config.tabs = ConfigTab::Tab,
            "spaces" => {
                if let Some(spaces) = value.and_then(|v| v.parse::<u8>().ok()) {
                    config.tabs = ConfigTab::Space(spaces);
                }
            }
            "chars" => {
                if let Some(chars) = value.and_then(|v| v.parse::<u8>().ok()) {
                    config.chars = chars;
                }
            }
            _ => (),
        }
    }

    Some(config)
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
        assert!(arguments.is_ok());
        let arguments: Arguments = arguments.unwrap();

        let config: Configuration = Configuration::from(&arguments);
        assert!(!config.newlines);
        assert!(!config.comment_pre_space);
        assert!(!config.align_text_groups);
        assert_eq!(config.case, ConfigCase::Unchanged);
        assert_eq!(config.tabs, ConfigTab::Space(4));
        assert_eq!(config.chars, 80);
    }

    #[test]
    fn test_get_configuration_comment_pre_space() {
        let args: Vec<String> = vec![String::from("--comment-pre-space")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert!(arguments.is_ok());
        let arguments: Arguments = arguments.unwrap();

        let config: Configuration = Configuration::from(&arguments);
        assert!(!config.newlines);
        assert!(config.comment_pre_space);
        assert!(!config.align_text_groups);
        assert_eq!(config.case, ConfigCase::Unchanged);
        assert_eq!(config.tabs, ConfigTab::Space(4));
        assert_eq!(config.chars, 80);
    }

    #[test]
    fn test_get_configuration_align_text_groups() {
        let args: Vec<String> = vec![String::from("--align-text-groups")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert!(arguments.is_ok());
        let arguments: Arguments = arguments.unwrap();

        let config: Configuration = Configuration::from(&arguments);
        assert!(!config.newlines);
        assert!(!config.comment_pre_space);
        assert!(config.align_text_groups);
        assert_eq!(config.case, ConfigCase::Unchanged);
        assert_eq!(config.tabs, ConfigTab::Space(4));
        assert_eq!(config.chars, 80);
    }

    #[test]
    fn test_get_configuration_newlines() {
        let args: Vec<String> = vec![String::from("-n")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert!(arguments.is_ok());
        let arguments: Arguments = arguments.unwrap();

        let config: Configuration = Configuration::from(&arguments);
        assert!(config.newlines);
        assert!(!config.comment_pre_space);
        assert!(!config.align_text_groups);
        assert_eq!(config.case, ConfigCase::Unchanged);
        assert_eq!(config.tabs, ConfigTab::Space(4));
        assert_eq!(config.chars, 80);
    }

    #[test]
    fn test_get_configuration_upper() {
        let args: Vec<String> = vec![String::from("-u")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert!(arguments.is_ok());
        let arguments: Arguments = arguments.unwrap();

        let config: Configuration = Configuration::from(&arguments);
        assert!(!config.newlines);
        assert!(!config.comment_pre_space);
        assert!(!config.align_text_groups);
        assert_eq!(config.case, ConfigCase::Uppercase);
        assert_eq!(config.tabs, ConfigTab::Space(4));
        assert_eq!(config.chars, 80);
    }

    #[test]
    fn test_get_configuration_lower() {
        let args: Vec<String> = vec![String::from("-l")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert!(arguments.is_ok());
        let arguments: Arguments = arguments.unwrap();

        let config: Configuration = Configuration::from(&arguments);
        assert!(!config.newlines);
        assert!(!config.comment_pre_space);
        assert!(!config.align_text_groups);
        assert_eq!(config.case, ConfigCase::Lowercase);
        assert_eq!(config.tabs, ConfigTab::Space(4));
        assert_eq!(config.chars, 80);
    }

    #[test]
    fn test_get_configuration_tabs() {
        let args: Vec<String> = vec![String::from("-t")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert!(arguments.is_ok());
        let arguments: Arguments = arguments.unwrap();

        let config: Configuration = Configuration::from(&arguments);
        assert!(!config.newlines);
        assert!(!config.comment_pre_space);
        assert!(!config.align_text_groups);
        assert_eq!(config.case, ConfigCase::Unchanged);
        assert_eq!(config.tabs, ConfigTab::Tab);
        assert_eq!(config.chars, 80);
    }

    #[test]
    fn test_get_configuration_spaces() {
        let args: Vec<String> = vec![String::from("-s"), String::from("2")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert!(arguments.is_ok());
        let arguments: Arguments = arguments.unwrap();

        let config: Configuration = Configuration::from(&arguments);
        assert!(!config.newlines);
        assert!(!config.comment_pre_space);
        assert!(!config.align_text_groups);
        assert_eq!(config.case, ConfigCase::Unchanged);
        assert_eq!(config.tabs, ConfigTab::Space(2));
        assert_eq!(config.chars, 80);
    }

    #[test]
    fn test_get_configuration_chars() {
        let args: Vec<String> = vec![String::from("-c"), String::from("120")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert!(arguments.is_ok());
        let arguments: Arguments = arguments.unwrap();

        let config: Configuration = Configuration::from(&arguments);
        assert!(!config.newlines);
        assert!(!config.comment_pre_space);
        assert!(!config.align_text_groups);
        assert_eq!(config.case, ConfigCase::Unchanged);
        assert_eq!(config.tabs, ConfigTab::Space(4));
        assert_eq!(config.chars, 120);
    }
}
