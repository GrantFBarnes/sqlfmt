use crate::arguments::Arguments;

pub struct Configuration {
    pub newlines: bool,
    pub case: ConfigCase,
    pub tabs: ConfigTab,
}

impl Configuration {
    #[allow(dead_code)]
    pub fn new() -> Configuration {
        Configuration {
            newlines: false,
            case: ConfigCase::Unchanged,
            tabs: ConfigTab::Space(4),
        }
    }

    pub fn from(args: &Arguments) -> Configuration {
        Configuration {
            newlines: false,
            case: if args.upper {
                ConfigCase::Uppercase
            } else if args.lower {
                ConfigCase::Lowercase
            } else {
                ConfigCase::Unchanged
            },
            tabs: if args.tabs {
                ConfigTab::Tab
            } else {
                ConfigTab::Space(args.spaces)
            },
        }
    }
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
