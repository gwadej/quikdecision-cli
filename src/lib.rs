extern crate quikdecision;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::once;
use eyre::{eyre,bail,WrapErr};

use quikdecision::{coin,pick,percent,dice,deck,select,shuffle,oracle};
use quikdecision::{Command,ApiDoc};

mod help;

type StrVec = Vec<String>;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_args(mut args: std::env::Args) -> eyre::Result<Command>
{
    let progname = args.next().unwrap();
    let cmd = args.next().ok_or_else(|| eyre!("Missing decision type"))?;

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
        "coin" | "flip" => coin::command().wrap_err("Failed coin flip"),
        "pick" => pick_command(&mut args),
        "percent" | "likely" => percent::command(int_arg::<u32>(args.next())?).wrap_err("Failed percent likely"),
        "roll"  => dice::command(args_to_string(&mut args)).wrap_err("Failed dice roll"),
        "draw"  => deck::command(args_to_string(&mut args).as_str()).wrap_err("Failed card draw"),
        "select" => select::command(args_to_strings(&mut args)?).wrap_err("Failed select"),
        "shuffle" => shuffle::command(args_to_strings(&mut args)?).wrap_err("Failed shuffle"),
        "oracle" => oracle::command().wrap_err("Failed oracle call"),
        "help" => help::usage(progname, args.next(), all_docs),
        "man" => help::help(progname, args.next(), all_docs),
        "version" => version(),
        _ => bail!("Unknown command"),
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

fn pick_command(args: &mut env::Args) -> eyre::Result<Command>
{
    let low  = int_arg::<i32>(args.next()).wrap_err("Bad lower bound")?;
    let high = int_arg::<i32>(args.next()).wrap_err("Bad upper bound")?;
    pick::command(low, high).wrap_err("Failed number pick")
}

fn args_to_strings(args: &mut env::Args) -> eyre::Result<Vec<String>>
{
    let first = args.next().ok_or_else(|| eyre!("Missing required strings"))?;

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

fn list_from_file(filename: &str) -> eyre::Result<StrVec>
{
    let mut file = File::open(filename).wrap_err(format!("Cannot open '{}'", filename))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).wrap_err(format!("Cannot read '{}'", filename))?;
    Ok(contents.split('\n')
               .filter(|line| !line.is_empty())
               .map(|s| s.to_string())
               .collect::<StrVec>())
}

fn args_to_string(args: &mut env::Args) -> String
{
    args.collect::<Vec<String>>().join(" ")
}

pub fn int_arg<T>(opt: Option<String>) -> eyre::Result<T>
where
T: std::str::FromStr,
{
    opt.ok_or_else(|| eyre!("Missing required parameter"))
        .and_then(|arg| arg.parse::<T>()
                        .map_err(|_| eyre!("{} is not a valid {}", arg, std::any::type_name::<T>())))
}
