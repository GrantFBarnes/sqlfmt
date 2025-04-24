use crate::arguments::Arguments;

pub struct Configuration {
    pub case: ConfigCase,
    pub tabs: ConfigTab,
}

impl Configuration {
    pub fn from(args: &Arguments) -> Configuration {
        Configuration {
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

pub enum ConfigCase {
    Uppercase,
    Lowercase,
    Unchanged,
}

pub enum ConfigTab {
    Tab,
    Space(u8),
}
