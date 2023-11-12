pub struct Shelby {
    config: Config,
}

pub(crate) struct Config {
    pub(crate) quiet: bool,
}

impl Shelby {
    #[must_use]
    pub fn new(quiet: bool) -> Shelby {
        Shelby {
            config: Config { quiet },
        }
    }
    #[must_use]
    pub fn parse(&self, data: String) -> String {
        if !self.config.quiet {
            println!("{data:#?}");
        }
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let client = Shelby::new(false);
        let result = client.parse(String::from("test"));
        assert_eq!(result, "test");
    }
}
