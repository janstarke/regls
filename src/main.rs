use std::path::PathBuf;
use clap::{App, Arg};
use anyhow::Result;
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};
use rwinreg::hive::Hive;
use rwinreg::nk::NodeKey;
use std::fs::File;

struct RegLsApp {
    reg_file: File,
    print_recursive: bool,
}

impl RegLsApp {

    pub fn new(file: File) -> Self {
        Self {
            reg_file: file,
            print_recursive: false
        }
    }
    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.print_recursive = recursive;
        self
    }
    pub fn run(&mut self) -> Result<()> {
        let mut hive = Hive::from_source(&self.reg_file)?;
        let mut root = hive.get_root_node()?;
        self.print_node(root, 0)?;
        Ok(())
    }
    fn print_node(&mut self, mut node: NodeKey, level: usize) -> Result<()> {
        let indent = "  ".repeat(level);
        loop {
            let record = match node.get_next_key(&mut self.reg_file)? {
                None => {break;}
                Some(node) => node
            };
            println!("{}{}", indent, record.key_name());

            if self.print_recursive {
                self.print_node(record, level + 1)?;
            }
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
            )
            .arg(
                Arg::with_name("RECURSIVE")
                    .help("print recursively")
                    .multiple(false)
                    .takes_value(false)
                    .short("R")
                    .long("recursive")
            )
        ;
        let matches = app.get_matches();


        let filename = matches.value_of("REG_FILE").expect("missing hive filename");

        let fp = PathBuf::from(&filename);
        let reg_file = if ! (fp.exists() && fp.is_file()) {
            return Err(anyhow::Error::msg(format!("File {} does not exist", &filename)));
        } else {
            File::open(fp)?
        };

        Ok(RegLsApp::new(reg_file)
            .with_recursive(matches.is_present("RECURSIVE"))
        )
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
