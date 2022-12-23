use figlet_rs::FIGfont;
use std::error::Error;
use std::path::PathBuf;
use std::time::SystemTime;
use std::{fmt, fs};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "dir_manager",
    about = "An example CLI for managing a directory"
)]
struct Opt {
    #[structopt(short, long, help = "List directory contents", default_value = ".")]
    ls: String,
    #[structopt(short, long, help = "Create a directory")]
    mkdir: Option<String>,
    #[structopt(short, long, help = "Create a file")]
    touch: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    ls: String,
    mkdir: Option<String>,
    touch: Option<String>,
}

// Create a Result to represent an Ok value for any type T or some Err value that implements the Error trait
type MainResult<T> = Result<T, Box<dyn Error>>;

struct CustomTime(SystemTime);
impl fmt::Display for CustomTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self
            .0
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
            .round();
        write!(f, "{}", secs)
    }
}

fn list_dir_contents(filepath: &str) {
    let files = fs::read_dir(filepath).unwrap();
    let detailed_files = files
        .map(|file| {
            let file = file.unwrap();
            let metadata = file.metadata().unwrap();
            (file.path(), metadata.len(), metadata.created().unwrap())
        })
        .collect::<Vec<_>>();

    println!("file\t\t\tsize\t\tcreated_at");
    for (file, size, created_at) in detailed_files {
        println!("{:?}\t\t{}\t\t{}", file, size, CustomTime(created_at));
    }
}

fn create_dir(filepath: &str) {
    if !PathBuf::from(filepath).exists() {
        fs::create_dir(filepath).unwrap();
        println!("The directory has been created successfully");
    }
}

fn create_file(filepath: &str) {
    fs::File::create(filepath).unwrap();
    println!("An empty file has been created");
}

fn print_ascii() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Dir Manager");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}

pub fn get_args() -> MainResult<Config> {
    let opt = Opt::from_args();
    Ok(Config {
        ls: opt.ls,
        mkdir: opt.mkdir,
        touch: opt.touch,
    })
}

pub fn run(config: Config) -> MainResult<()> {
    print_ascii();

    if !config.ls.is_empty() {
        list_dir_contents(&config.ls);
    }
    if let Some(mkdir) = config.mkdir {
        create_dir(&mkdir);
    }
    if let Some(touch) = config.touch {
        create_file(&touch);
    }

    Ok(())
}
