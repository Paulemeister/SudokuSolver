use std::{fs, path::PathBuf};
use termion::{color,cursor};
use clap::{ArgGroup,Parser,Subcommand,Args};
use shellexpand;
use std::io::Write;
#[derive(Debug)]

#[derive(Copy,Clone)]
struct PossField{
    fields: [Option<bool>;9],
    amount: i32
}


impl PossField {
    fn get(self, idx: usize) -> Option<bool>{
        self.fields[idx-1]
    }
    fn set(&mut self, idx: usize, val: bool){
        if idx >9{
            print!("")
        }
        self.fields[idx-1] = Some(val);
        PossField::calc_amount(self);
    }
    fn new() -> PossField{
        PossField { fields: [None;9],amount: 0}
    }
    fn combine(&mut self, other: PossField){
        for i in 0..9{
            match (self.fields[i] ,other.fields[i]){
                (Some(_),Some(_)) => self.fields[i] = Some(self.fields[i].unwrap() && other.fields[i].unwrap()),
                (None,Some(_))=> self.fields[i] = other.fields[i],
                _ => (),
            }
        }
        PossField::calc_amount(self);
    }
    fn calc_amount(&mut self){
        self.amount = 0;
        for i in 0..9{
            match self.fields[i]{
                None => (),
                _ => self.amount += 1,
            }
        }
    }
}

#[derive(Copy,Clone)]
struct Board {
    fields: [Option<u32>;81]
}
impl Board {
    fn new() -> Board{
        Board{
            fields:[None;81],
        }
    }
}

#[derive(Copy,Clone)]
struct BoardPoss {
    fields: [PossField ;81]
}
    
impl BoardPoss {
    fn new() -> BoardPoss{
        BoardPoss{
            fields:[PossField::new() ;81]
        }
    }
}

#[derive(Debug)]
enum BoardReadError{
    WrongLength,
    UnexpectedCharacter
}


fn make_board(input: &str,del: &char)-> Result<Board,BoardReadError>{
    let mut board = Board::new();
    
    if input.len() != 81{
        return Err(BoardReadError::WrongLength);
    }
    for (i,c) in input.chars().enumerate() {
        match c {
            '1'..='9' => board.fields[i] = Some(c.to_digit(10).unwrap()),
            c if c==*del => board.fields[i] = None,
            _ => return Err(BoardReadError::UnexpectedCharacter),
        }
    }
    return Ok(board);
}

fn check_rows(board: &Board,poss: &BoardPoss) -> BoardPoss{
    let mut new_poss = *poss;
    for i in 0..9 {
        let mut not_possible = PossField::new();
        for j in 0..9 {
            let num = board.fields[i*9 + j];
            match num {
                Some(a)=> not_possible.set(a as usize,false),
                None => (),
            }
        }
        for j in 0..9 {
            new_poss.fields[i*9 + j].combine(not_possible);
        }
    }
    return new_poss;
}

fn check_columns(board: &Board,poss: &BoardPoss) -> BoardPoss{
    let mut new_poss = *poss;
    for i in 0..9 {
        let mut not_possible = PossField::new();
        for j in 0..9 {
            let num = board.fields[i + j*9];
            match num {
                Some(a)=> not_possible.set(a as usize,false),
                None => (),
            }
        }
        for j in 0..9 {
            new_poss.fields[i + j*9].combine(not_possible);
        }
    }
    return new_poss;
}

fn check_squares(board: &Board,poss: &BoardPoss) -> BoardPoss{
    let mut new_poss = *poss;
    for i in 0..3{
        for j in 0..3{
            let mut square = PossField::new();
            for l in 0..3{
                for k in 0..3{
                    let num = board.fields[3*j +k + 9*(3*i+l)];
                    match num {
                        Some(a)=> square.set(a as usize,false),
                        None => (),
                    }       
                }
            }
            for l in 0..3{
                for k in 0..3{
                    new_poss.fields[3*j +k + 9*(3*i+l)].combine(square);
                }
            }
        }
    }
    return new_poss;
}

fn get_formatted_board(board:&Board,del: &char) -> String{
    let mut out = String::new();
    for i in 0..9{ // cols
        if i==3 || i==6{out.push_str("\n")}
        for j in 0..9{ //rows
            if j==3 || j==6{out.push_str(" ")}
            let val = board.fields[i*9 +j];
            match val{
                Some(v) => out.push_str(format!{" {} ",v}.as_str()),
                None => out.push_str(&format!(" {} ",del)),
            }
        }
        out.push_str("\n");
    }
    out
}

#[allow(dead_code)]
fn print_checked(poss: &BoardPoss){
    for i in 0..9{
        if i%3==0{println!{""};} //Delim Rows Squares
        for j in 0..3{
            for k in 0..9{
                if k%3==0{print!{"  "};}//Delim Cols Squares
                for l in 0..3{
                    let val = poss.fields[9*i+k].get(l+3*j+1);
                    match val {
                       Some(false)=> print!{" {1}{0}",l+3*j +1,color::Bg(color::Red)},
                       Some(true)=> print!{" {1}{0}",l+3*j +1,color::Bg(color::Green)},
                       None=> print!{" {1}{0}",l+3*j +1,color::Bg(color::Reset)},
                    }
                }
                print!{"{} ",color::Bg(color::Reset)} //Delim Cols Neigbour
            }
            println!{""} //Delim Rows Neigbour
        }
        //println!{""}
    }
}

fn replace_known(board: &mut Board, poss: &BoardPoss) -> Option<bool>{
    let mut replaced = false;
    for (i,f) in poss.fields.iter().enumerate(){
        match board.fields[i]{
            Some(_) => continue,
            None => (),
        }
        let mut amount = 0;
        for j in 1..10 {
            match f.get(j) {
                Some(false) => amount+=1,
                _ => (),
                }
        }
        if amount==8 {
            for j in 1..10 {
                match f.get(j) {
                    None => {
                        board.fields[i] = Some(j as u32);
                        replaced = true;
                        break
                    },
                    _ => (),
                    }
            }
        }
        else if amount==9 {
            return None
        }
    }
    return Some(replaced)
}

fn has_none(board: &Board) -> bool{

    for f in board.fields.iter(){
        match f {
            None => return true,
            _ => (),
        }
    }
    return false
}

fn calc_dependence(board: &Board) -> [i32;81]{
    let mut dependence = [0;81];

    let mut row_amount = [0;9];
    let mut column_amount = [0;9];
    // Rows and Columns
    for i in 0..9{
        for j in 0..9{
            match board.fields[9*i+j] {
                Some(_) => row_amount[i]+=1,
                _ => (),
            }
            match board.fields[i+j*9] {
                Some(_) => column_amount[j]+=1,
                _ => (),
            }
        }
    }
    // Squares, don't count double, 
    //TODO
    for i in 0..9{
        for j in 0..9{
            dependence[9*i+j] = row_amount[i]+column_amount[j];
        }
    }

    return dependence
}

fn calc_guess(board: &Board, poss: &BoardPoss) -> usize{
    let dependence = calc_dependence(&board);
    let mut max_i = 0;
    let mut max = -1;
    let mut max_depend = -1;
    for (i,f) in board.fields.iter().enumerate(){
        match f {
            Some(_) => continue,
            _ => (),
        }
        
        let amount = poss.fields[i].amount;
        let depend = dependence[i];
        // most impossible vals and most dependence
        if (amount > max) || (amount == max && depend>max_depend) {
            max = amount;
            max_i = i;
            max_depend = depend;
        }
    }
    if max ==-1{
        print!("");
    }
    return max_i
}


fn try_solve(input_board:&Board,input_poss: &BoardPoss) -> Option<Board>{
    //print_board(&input_board);
    let mut board = *input_board;
    let mut poss = *input_poss;
    let mut exchanged;
    while has_none(&board){
        poss = check_rows(&board,&poss);
        poss = check_columns(&board,&poss);
        poss = check_squares(&board,&poss);
        match replace_known(&mut board, &poss){
            Some(a) => exchanged = a,
            None => return None,
        }
        if !exchanged {
            let guess_index = calc_guess(&board,&poss);
            let mut guess: u32 = 0;
            for i in 0..9{
                let field_poss = poss.fields[guess_index].fields[i];
                match field_poss{
                    None => {
                        guess= i as u32 +1;
                        break;
                    },
                    _ => (),
                }
            }
            let mut new_board = board;
            new_board.fields[guess_index] = Some(guess);
            match try_solve(&new_board, &poss.clone()){
                Some(_board) => return Some(_board),
                None => poss.fields[guess_index].set(guess as usize,false),
            }
        }
    }
    
    return Some(board)
}


fn get_encoded_board(board: &Board,del: &char) -> String{
    let mut out = String::new();
    for field in board.fields{
        match field{
            Some(n) => out.push_str(format!("{}",n).as_str()),
            None => out.push_str(&del.to_string())
        }
    }
    out.push_str("\n");
    out
}

#[derive(Parser)]
struct Cli{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands{
    Solve(Shared),
    Print(Shared)
}
#[derive(Args)]
#[clap(group(ArgGroup::new("input").required(true)))]
#[clap(group(ArgGroup::new("output").required(true)))]
struct Shared {
    #[arg(group="input",value_parser=check_input_file)]
    input_file_pos: Option<std::path::PathBuf>,
    #[arg(short,long, group="input",value_parser=check_input_file)]
    input_file_opt: Option<std::path::PathBuf>,
    #[arg(short='r',long,group="input")]
    input_raw: Option<String>,

    #[arg(group = "output",value_parser=check_output_file)]
    output_file_pos: Option<std::path::PathBuf>,
    #[arg(short,long, group = "output",value_parser=check_output_file)]
    output_file_opt: Option<std::path::PathBuf>,
    #[arg(short,long,group = "output")]
    text_out: bool,

    #[arg(short,long)]
    formated: bool,

    #[arg(short='n',long,default_value_t=1,allow_hyphen_values=true,value_parser=check_amount)]
    amount: usize,

    #[arg(short,long,default_value_t='.')]
    delimeter: char,

    #[arg(short='l',long,default_value_t='.')]
    delimeter_in: char,
}

fn check_amount(s: &str) -> Result<usize, String> {
    let num1: i32 = match s.parse(){
        Ok(u)=> u,
        Err(e) => return Err(e.to_string()),
        //_ => usize::MAX
    };
    if num1 < 0 {
        return Ok(usize::MAX);
    }
    else {
        return match usize::try_from(num1){
            Ok(n) => Ok(n),
            Err(e) => Err(e.to_string())
        }
    }
}

fn check_input_file(s: &str) -> Result<std::path::PathBuf, String> {
    let path = match shellexpand::full(s){
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
    let path = match shellexpand::full(s){
        Ok(s) => PathBuf::from(s.to_string()),
        Err(e) => return Err(e.to_string()),
    };
    
    let filename = match path.is_dir(){
        true => path.join("solutions.txt"),
        false => path,
    };
    match std::fs::OpenOptions::new().write(true).create(true).open(&filename){
        Ok(_) => Ok(filename),
        Err(e) => Err(e.to_string())
    }
        
    
}

fn print_command(lines: &Vec<&str>,cli: &Shared){
    let mut boards: Vec<Board> = Vec::new();
    let mut loaded = 0;
    for (j,line) in lines.iter().enumerate(){
        if cli.amount<=loaded{break}
        match make_board(line,&cli.delimeter_in){
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
            output.push_str(get_encoded_board(board,&cli.delimeter).as_str());
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
        let board = match make_board(line,&cli.delimeter_in){
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
            output.push_str(get_encoded_board(&solved_board,&cli.delimeter).as_str());
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

#[cfg(test)]
#[test]
fn poss_field() {
    let mut a = PossField::new();
    a.set(1, false);
    assert_eq!(a.get(1),Some(false));
}

