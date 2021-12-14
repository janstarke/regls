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
    print_values: bool,
    path: Option<Vec<String>>
}

impl RegLsApp {

    pub fn new(file: File) -> Self {
        Self {
            reg_file: file,
            print_recursive: false,
            print_values: false,
            path: None
        }
    }

    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.print_recursive = recursive;
        self
    }

    pub fn with_print_values(mut self, values: bool) -> Self {
        self.print_values = values;
        self
    }

    pub fn with_path(mut self, path: Vec<String>) -> Self {
        self.path = Some(path);
        self
    }

    pub fn run(&mut self) -> Result<()> {
        let mut hive = Hive::from_source(&self.reg_file)?;
        let mut root = hive.get_root_node()?;

        if let Some(path) = self.path.clone() {
            root = match self.find_node(root, &path)? {
                None => { return Ok(())},
                Some(n) => n
            }
        }

        if self.print_values {
            self.print_node_values(root, 0)?;
        } else {
            self.print_node(root, 0)?;
        }
        Ok(())
    }

    fn find_node(&mut self, mut root: NodeKey, path: &Vec<String>) -> Result<Option<NodeKey>> {
        for entry in path {
            let mut found_entry = false;

            loop {
                let record = match root.get_next_key(&mut self.reg_file)? {
                    None => {break;}
                    Some(node) => node
                };

                if record.key_name() == entry {
                    root = record;
                    found_entry = true;
                    break;
                }
            }

            if ! found_entry {
                return Ok(None);
            }
        }
        Ok(Some(root))
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

    fn print_node_values(&mut self, mut node: NodeKey, level: usize) -> Result<()> {
        let indent = "  ".repeat(level);
        loop {
            let value = match node.get_next_value(&mut self.reg_file)? {
                None => {break;}
                Some(node) => node
            };

            println!("{}{} = ", indent, value.get_name());

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
                Arg::with_name("PATH")
                    .help("registry path")
                    .required(false)
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
            .arg(
                Arg::with_name("VALUES")
                    .help("print only values")
                    .multiple(false)
                    .takes_value(false)
                    .short("V")
                    .long("values")
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

        let mut app = RegLsApp::new(reg_file)
            .with_recursive(matches.is_present("RECURSIVE"))
            .with_print_values(matches.is_present("VALUES"));

        if let Some(path) = matches.value_of("PATH") {
            let parts: Vec<String> = path.split("/").map(|x|x.to_owned()).collect();
            app = app.with_path(parts);
        }
        Ok(app)
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
