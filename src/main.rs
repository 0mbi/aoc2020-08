extern crate nom;

use nom::do_parse;
use nom::named;
use nom::tag;
use nom::alt;
use nom::combinator::rest;

#[derive(Debug)]
enum STMT {
    NOP,
    ACC,
    JMP,
    ERR,
}

#[derive(Debug)]
pub struct Stmt {
    mnem: STMT,
    param: i32,
}

fn make_stmt(mnem: &[u8], sign: &[u8], param_str: &[u8]) -> Stmt {
    let mut param = String::from_utf8_lossy(param_str).parse::<i32>().unwrap();
    if sign[0] == b'-' {
        param = param * -1;
    }
    let stmt_enum = match mnem {
        b"nop" => STMT::NOP,
        b"jmp" => STMT::JMP,
        b"acc" => STMT::ACC,
        _ => STMT::ERR
    };
    Stmt {
        mnem: stmt_enum,
        param: param,
    }
}

named!(#[inline], pub parse_stmt<Stmt>,
       do_parse!(
           mnem: alt!(tag!("nop")|tag!("acc")|tag!("jmp"))
           >> tag!(" ")
           >> sign: alt!(tag!("+")|tag!("-"))
           >> param: rest
           >> (
                 make_stmt(mnem, sign, param)
              )
       ));

fn load_stmts(stmt_strings: Vec<&[u8]>) -> Vec<Stmt> {
    return stmt_strings
        .iter()
        .map(|stmt| parse_stmt(stmt).unwrap().1)
        .collect::<Vec<Stmt>>();
}

fn run_stmts(stmts: Vec<Stmt>) -> i32 {
    let mut lines_visited: Vec<bool> = vec![false; stmts.len()]; // TODO: use bitset instead
    let mut idx = 0;
    let mut acc = 0;
    while !lines_visited[idx] {
        lines_visited[idx] = true;
        match stmts[idx] {
            Stmt {mnem: STMT::JMP, param} => {
                if param > 0 {
                    idx = idx + param as usize
                } else {
                    idx = idx - (-1*param) as usize;
                }
            },
            Stmt {mnem: STMT::ACC, param} => {
                acc = acc + param;
                idx = idx + 1;
            },
            Stmt {mnem: STMT::NOP, param: _} => {
                idx = idx + 1;
            },
            Stmt {mnem: STMT::ERR, param: _} => {
                panic!("error statement occured")
            },
        }
    }
    return acc
}

fn main() {
    let stmts: Vec<&[u8]> = vec![
        b"nop +0",
        b"acc +1",
        b"jmp +4",
        b"acc +3",
        b"jmp -3",
        b"acc -99",
        b"acc +1",
        b"jmp -4",
        b"acc +6",
    ];
    let stmt_vect = load_stmts(stmts);
    println!("Accumulator is: {:?}", run_stmts(stmt_vect))
}
