#[cfg(not(windows))]
use termion::{cursor};
#[cfg(windows)]
use termion_win::{cursor};
use std::io::Write;
use std::path::PathBuf;
use clap::Parser;
use std::fs;

pub mod board;
pub mod cli;
pub mod solver;
pub mod io;

use crate::{board::*,cli::*,solver::*,io::*};

fn print_command(lines: &Vec<&str>,cli: &Shared){
    let mut boards: Vec<Board> = Vec::new();
    let mut loaded = 0;
    for (j,line) in lines.iter().enumerate(){
        if cli.amount<=loaded{break}
        match make_board(line,&cli.delimeter_in,&cli.line_end_in){
            Ok(b) => boards.push(b),
            Err(e) => {
                eprintln!{"Line {}: {:?}",j,e};
                continue
            },
        };
        loaded+=1;
    }
    let mut output = String::new();
    for (i,board) in boards.iter().enumerate(){
        if cli.amount<=i{break}
        if cli.formated {
            output.push_str(get_formatted_board(board,&cli.delimeter).as_str());
            output.push_str("----------------------------\n");
        }
        else {
            output.push_str(get_encoded_board(board,&cli.delimeter,&cli.line_end_out).as_str());
        }
    }
    let default = PathBuf::from("./solutions.txt");
    let output_file = match (&cli.output_file_pos,&cli.output_file_opt){
        (Some(f),_) => f,
        (_,Some(f))=> f,
        (None,None) => &default,
    };
    if cli.text_out{
        print!("{}",output);
    }
    else {
        let file =  match std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(output_file) {
            Err(e) => {eprintln!("{}",e.to_string()); return},
            Ok(f) => f,
        };
        match write!(&file,"{}",output){
            Ok(_) =>(),
            Err(e) => {eprintln!("{}",e.to_string()); return}
        }
    }
}

fn solve_command(lines: &Vec<&str>,cli: &Shared){
    let mut len = lines.len();
    let len_digits = 4 as usize;
    let mut output = String::new();
    let mut i = 0;
    for (j,line) in lines.iter().enumerate(){
        if cli.amount<=i{break}
        let board = match make_board(line,&cli.delimeter_in,&cli.line_end_in){
            Ok(b) => {b},
            Err(e) => {
                eprintln!{"Line {}: {:?}",j,e};
                len-= 1;
                continue
            },
        };
        
        let solved_board = match try_solve(&board, &BoardPoss::new()){
            Some(b) => b,
            None => {println!("Something went wrong solving the Board.");continue}
        };

        print!("{:>len_digits$}/{:>len_digits$} solved\n{}",i,len,cursor::Up(1));
        i+=1;
    
        if cli.formated {
            output.push_str(get_formatted_board(&solved_board,&cli.delimeter).as_str());
            output.push_str("----------------------------\n");
        }
        else {
            output.push_str(get_encoded_board(&solved_board,&cli.delimeter,&cli.line_end_out).as_str());
        }
    }
    let default = PathBuf::from("./solutions.txt");
    let output_file = match (&cli.output_file_pos,&cli.output_file_opt){
        (Some(f),_) => f,
        (_,Some(f))=> f,
        (None,None) => &default,
    };
    if cli.text_out{
        print!("{}",output);
    }
    else {
        let file =  match std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&output_file) {
            Err(e) => {eprintln!("{}",e.to_string()); return},
            Ok(f) => f,
        };
        match write!(&file,"{}",output){
            Ok(_) =>(),
            Err(e) => {eprintln!("{}",e.to_string()); return}
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let command_args = match &cli.command {
        Commands::Solve(a) => a,
        Commands::Print(a) => a
    };
    let contents = match (&command_args.input_raw,&command_args.input_file_pos,&command_args.input_file_opt){
        (None,Some(p),None) =>  fs::read_to_string(p)
        .expect("Should have been able to read the file"),
        (None,None,Some(t)) => fs::read_to_string(t)
        .expect("Should have been able to read the file"),
        (Some(s),None,None) => s.to_string(),
        _ => "s".to_string()
    };
    let lines = contents.split('\n').map(|x| x.trim()).collect::<Vec<&str>>();
    
    match cli.command {
        Commands::Solve(a) => solve_command(&lines, &a),
        Commands::Print(a) => print_command(&lines, &a)
    }
}
