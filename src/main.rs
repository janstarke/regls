use std::path::PathBuf;
use clap::{App, Arg};
use anyhow::Result;
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};
use rwinreg::hive::Hive;
use std::fs::File;

struct RegLsApp {
    reg_file: File,
}

impl RegLsApp {

    pub fn new(file: File) -> Self {
        Self {
            reg_file: file
        }
    }
    pub fn run(&mut self) -> Result<()> {
        let mut hive = Hive::from_source(&self.reg_file)?;
        let mut root = hive.get_root_node()?;

        loop {
            let record = match root.get_next_key(&mut self.reg_file)? {
                None => {break;}
                Some(node) => node
            };
            println!("{}", record.key_name());
        }
        Ok(())
    }
    pub fn parse_options() -> Result<RegLsApp> {
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
        let reg_file = if ! (fp.exists() && fp.is_file()) {
            return Err(anyhow::Error::msg(format!("File {} does not exist", &filename)));
        } else {
            File::open(fp)?
        };

        Ok(RegLsApp::new(reg_file))
    }
}

fn main() -> Result<()> {
    let _ = TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto);
    
    let mut app = RegLsApp::parse_options()?;
    app.run()
}
