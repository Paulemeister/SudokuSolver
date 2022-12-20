use clap::{ArgGroup, Args, Parser, Subcommand};
use shellexpand;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Solve(Shared),
    Print(Shared),
}
#[derive(Args)]
#[clap(group(ArgGroup::new("input").required(true)))]
#[clap(group(ArgGroup::new("output").required(true)))]
pub struct Shared {
    #[arg(group="input",value_parser=check_input_file)]
    pub input_file_pos: Option<std::path::PathBuf>,
    #[arg(short,long, group="input",value_parser=check_input_file)]
    pub input_file_opt: Option<std::path::PathBuf>,
    #[arg(short = 'r', long, group = "input")]
    pub input_raw: Option<String>,

    #[arg(group = "output",value_parser=check_output_file)]
    pub output_file_pos: Option<std::path::PathBuf>,
    #[arg(short,long, group = "output",value_parser=check_output_file)]
    pub output_file_opt: Option<std::path::PathBuf>,
    #[arg(short, long, group = "output")]
    pub text_out: bool,

    #[arg(short, long)]
    pub formated: bool,

    #[arg(short='n',long,default_value_t=1,allow_hyphen_values=true,value_parser=check_amount)]
    pub amount: usize,

    #[arg(short, long, default_value_t = '.')]
    pub delimeter: char,

    #[arg(short = 'l', long, default_value_t = '.')]
    pub delimeter_in: char,

    #[arg(short='e',long,default_value_t=String::from(""))]
    pub line_end_in: String,

    #[arg(long,default_value_t=String::from(""))]
    pub line_end_out: String,
}

fn check_amount(s: &str) -> Result<usize, String> {
    let num1: i32 = match s.parse() {
        Ok(u) => u,
        Err(e) => return Err(e.to_string()),
        //_ => usize::MAX
    };
    if num1 < 0 {
        Ok(usize::MAX)
    } else {
        match usize::try_from(num1) {
            Ok(n) => Ok(n),
            Err(e) => Err(e.to_string()),
        }
    }
}

fn check_input_file(s: &str) -> Result<std::path::PathBuf, String> {
    let path = match shellexpand::full(s) {
        Ok(s) => PathBuf::from(s.to_string()),
        Err(e) => return Err(e.to_string()),
    };
    if path.is_file() {
        Ok(path)
    } else {
        Err(String::from("not a file"))
    }
}

fn check_output_file(s: &str) -> Result<std::path::PathBuf, String> {
    let path = match shellexpand::full(s) {
        Ok(s) => PathBuf::from(s.to_string()),
        Err(e) => return Err(e.to_string()),
    };

    let filename = match path.is_dir() {
        true => path.join("solutions.txt"),
        false => path,
    };
    match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&filename)
    {
        Ok(_) => Ok(filename),
        Err(e) => Err(e.to_string()),
    }
}
