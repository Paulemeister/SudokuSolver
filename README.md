# SudokuSolver

Project for me to learn Rust Syntax.

## How it works
Solves Sudokus with an recursive appproach.

For every Field a boolean array of Possibilities is constructed.
For every iteration the rows, collumns and squares are checked and the Possibilitys are reduced.
Once only one possibility remains, it is selected.

If no new trivial steps are left, the Field with the least possibilities and the most impact on other fields is guessed.
A new Board is constructed and attemped to solve. If the solve fails, the guessed number is removed fom the possibilities of the fields.

## Usage

CLI

A List of sudokus in the form of 81 characters [1-9] for known fields and [.] for unknown ones, 
separated by newlines can be provided in a path to a file or directly as a argument with the flag `-t`

The output can either be in the same format, or formated user readable with `-f`, either printed to STDOUT with `-` or to a file specified.

The number of lines that should be solved can be specified by `-n` the default is 1

two subcommands are available :
- `print` will parse the inputs and only output the given boards
- `solve` will parse, solve and then output

```shell
cargo run --release -- solve -r ........8..3...4...9..2..6.....79.......612...6.5.2.7...8...5...1.....2.4.5.....3 -tf

 2  4  6   7  9  5   1  3  8 
 5  7  3   1  8  6   4  9  2 
 8  9  1   4  2  3   7  6  5 

 1  5  2   8  7  9   3  4  6 
 3  5  7   8  6  1   2  4  9 
 9  6  4   5  3  2   8  7  1 

 6  3  8   2  1  4   5  1  7 
 7  1  9   3  5  8   6  2  4 
 4  2  5   6  1  7   9  8  3 
----------------------------



```
