use std::path::PathBuf;
use clap::{App, Arg};
use anyhow::Result;
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};

struct RegLsApp {
    reg_file: PathBuf,
}

impl RegLsApp {

    pub fn new() -> Self {
        Self {
            reg_file: PathBuf::new()
        }
    }
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
    fn parse_options(&mut self) -> Result<()> {
        let app = App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(
                Arg::with_name("REG_FILE")
                    .help("path to registry hive file")
                    .required(true)
                    .multiple(false)
                    .takes_value(true),
            );
        let matches = app.get_matches();


        let filename = matches.value_of("REG_FILE").expect("missing hive filename");

        let fp = PathBuf::from(&filename);
        if ! (fp.exists() && fp.is_file()) {
            return Err(anyhow::Error::msg(format!("File {} does not exist", &filename)));
        } else {
            self.reg_file = fp;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let _ = TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto);
    
    let mut app = RegLsApp::new();
    app.parse_options()?;
    app.run()
}
