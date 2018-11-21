extern crate quikdecision;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::once;

use quikdecision::{coin,pick,percent,dice,select,oracle};
use quikdecision::Command;

mod help;

type StrVec = Vec<String>;

pub fn parse_args(mut args: std::env::Args) -> Result<Command, String>
{
    let progname = args.next().unwrap();
    let cmd = match args.next()
    {
        Some(c) => c,
        None => return Err(String::from("Missing decision type")),
    };
    let all_docs = vec![
        ("coin",    coin::api_doc()),
        ("flip",    coin::api_doc()),
        ("pick",    pick::api_doc()),
        ("percent", percent::api_doc()),
        ("likely",  percent::api_doc()),
        ("roll",    dice::api_doc()),
        ("select",  select::api_doc()),
        ("oracle",  oracle::api_doc()),
        ("help",    help::help_doc()),
        ("man",     help::man_doc()),
    ];

    match &cmd[..]
    {
        "coin" | "flip" => coin::command(),
        "pick" => pick_command(&mut args),
        "percent" | "likely" => percent::command(int_arg::<u32>(args.next())?),
        "roll"  => dice::command(args_to_string(&mut args)),
        "select" => select_command(&mut args),
        "oracle" => oracle::command(),
        "help" => help::usage(progname, args.next(), all_docs),
        "man" => help::help(progname, args.next(), all_docs),
        _ => Err(String::from("Unknown command")),
    }
}

fn pick_command(args: &mut env::Args) -> Result<Command, String>
{
    match (int_arg::<i32>(args.next()), int_arg::<i32>(args.next()))
    {
        (Ok(low), Ok(high)) => pick::command(low, high),
        (Err(e),  _) => return Err(format!("low arg: {}", e)),
        (_,       Err(e)) => return Err(format!("high arg: {}", e)),
    }
}

fn select_command(args: &mut env::Args) ->Result<Command, String>
{
    let first = match args.next()
    {
        Some(s) => s,
        None => return Err(String::from("Missing required strings")),
    };

    let strvec = if first.starts_with("@")
    {
        list_from_file(&first[1..])?
    }
    else
    {
        once(first).chain(args).collect::<StrVec>()
    };

    select::command(strvec)
}

fn list_from_file(filename: &str) -> Result<StrVec, String>
{
    let mut file = match File::open(filename)
    {
        Ok(f) => f,
        Err(_) => return Err(String::from("Cannot open supplied file")),
    };
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents)
    {
        return Err(String::from("Cannot read supplied file"));
    }
    Ok(contents.split("\n")
               .filter(|line| !line.is_empty())
               .map(|s| s.to_string())
               .collect::<StrVec>())
}

fn args_to_string(args: &mut env::Args) -> String
{
    args.collect::<Vec<String>>().join(" ")
}

pub fn int_arg<T>(opt: Option<String>) -> Result<T, String>
where
T: std::str::FromStr,
{
    match opt
    {
        None => Err(String::from("Missing required parameter")),
        Some(arg) => match arg.parse::<T>()
        {
            Ok(a) => Ok(a),
            Err(_) => Err(String::from("Argument not a valid integer")),
        },
    }
}
