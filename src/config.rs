static WELCOME_COMMAND: &str = "help";

pub struct Config {
    pub command: String,
    pub file_name: Option<String>,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let command = match args.next() {
            Some(arg) => arg,
            None => WELCOME_COMMAND.to_string(),
        };

        let file_name = match args.next() {
            Some(arg) => Some(arg),
            None => None,
        };

        match args.next() {
            Some(_) => return Err("Too many arguments"),
            None => (),
        };

        Ok(Config { command, file_name })
    }
}
