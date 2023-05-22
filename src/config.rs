static WELCOME_COMMAND: &str = "help";

pub struct Config {
    pub command: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let command =  if args.len() < 2 {
            WELCOME_COMMAND.clone().to_string()
        } else if args.len() == 2 {
            args[1].clone()
        } else {
            return Err("Too many arguments");
        };

        Ok(Config { command })
    }
}
