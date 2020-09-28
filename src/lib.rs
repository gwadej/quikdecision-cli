extern crate quikdecision;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::once;

use quikdecision::{coin,pick,percent,dice,deck,select,shuffle,oracle};
use quikdecision::{Command,ApiDoc};

mod help;

type StrVec = Vec<String>;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_args(mut args: std::env::Args) -> Result<Command, String>
{
    let progname = args.next().unwrap();
    let cmd = args.next().ok_or_else(|| "Missing decision type".to_string())?;

    let all_docs = vec![
        ("coin",    coin::api_doc()),
        ("flip",    coin::api_doc()),
        ("pick",    pick::api_doc()),
        ("percent", percent::api_doc()),
        ("likely",  percent::api_doc()),
        ("roll",    dice::api_doc()),
        ("draw",    deck::api_doc()),
        ("select",  select::api_doc()),
        ("select",  select_other_doc()),
        ("shuffle", shuffle::api_doc()),
        ("shuffle", shuffle_other_doc()),
        ("oracle",  oracle::api_doc()),
        ("help",    help::help_doc()),
        ("man",     help::man_doc()),
        ("version", version_doc()),
    ];

    match &cmd[..]
    {
        "coin" | "flip" => coin::command().map_err(|e| String::from(e)),
        "pick" => pick_command(&mut args),
        "percent" | "likely" => percent::command(int_arg::<u32>(args.next())?).map_err(|e| String::from(e)),
        "roll"  => dice::command(args_to_string(&mut args)).map_err(|e| String::from(e)),
        "draw"  => deck::command(args_to_string(&mut args).as_str()).map_err(|e| String::from(e)),
        "select" => select::command(args_to_strings(&mut args)?).map_err(|e| String::from(e)),
        "shuffle" => shuffle::command(args_to_strings(&mut args)?).map_err(|e| String::from(e)),
        "oracle" => oracle::command().map_err(|e| String::from(e)),
        "help" => help::usage(progname, args.next(), all_docs),
        "man" => help::help(progname, args.next(), all_docs),
        "version" => version(),
        _ => Err("Unknown command".to_string()),
    }
}

fn select_other_doc() -> ApiDoc
{
    ApiDoc
    {
        name: "select",
        params: vec!["@{filename}"],
        hint: "Select one of two or more strings supplied in a file.",
        help: vec![
             "Loads a series of strings from the specified file. (Each line is one string.)",
             "Selects one of the supplied strings with equal probability. There must be",
             "at least two strings to choose between.",
        ],
    }
}

fn shuffle_other_doc() -> ApiDoc
{
    ApiDoc
    {
        name: "shuffle",
        params: vec!["@{filename}"],
        hint: "Randomly order the strings supplied in a file.",
        help: vec![
             "Loads a series of strings from the specified file. (Each line is one string.)",
             "Randomly change the order of the supplied strings. There must",
             "be at least two strings to shuffle.",
        ],
    }
}

fn version_doc() -> ApiDoc
{
    ApiDoc
    {
        name: "version",
        params: vec![],
        hint: "Display the version.",
        help: vec!["Display version information."],
    }
}

fn pick_command(args: &mut env::Args) -> Result<Command, String>
{
    let low  = int_arg::<i32>(args.next()).map_err(|e| format!("low arg: {}", e))?;
    let high = int_arg::<i32>(args.next()).map_err(|e| format!("high arg: {}", e))?;
    pick::command(low, high).map_err(|e| String::from(e))
}

fn args_to_strings(args: &mut env::Args) -> Result<Vec<String>,String>
{
    let first = args.next().ok_or_else(|| "Missing required strings".to_string())?;

    let strvec = if first.starts_with('@')
    {
        list_from_file(&first[1..])?
    }
    else
    {
        once(first).chain(args).collect::<StrVec>()
    };

    Ok(strvec)
}

fn version() -> !
{
    println!("quikdecision v{}", VERSION);
    println!("   lib v{}", quikdecision::version());

    std::process::exit(1);
}

fn list_from_file(filename: &str) -> Result<StrVec, String>
{
    let mut file = File::open(filename).map_err(|_| "Cannot open supplied file".to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|_| "Cannot read supplied file".to_string())?;
    Ok(contents.split('\n')
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
    opt.ok_or_else(|| "Missing required parameter".to_string())
        .and_then(|arg| arg.parse::<T>().map_err(|_| "Argument not a valid integer".to_string()))
}
