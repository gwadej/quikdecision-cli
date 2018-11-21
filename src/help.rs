use std::iter::once;
use quikdecision::ApiDoc;

fn make_clue(cmd: &str, params: &Vec<&str>) -> String
{
    once(cmd).chain(params.iter().map(|s| *s)).collect::<Vec<&str>>().join(" ")
}

fn print_hint(cmd: &str, doc: &ApiDoc)
{
    let clue = make_clue(cmd, &doc.params);
    if clue.len() < 8
    {
        println!("  {:8}- {}", clue, doc.hint);
    }
    else
    {
        println!("  {}\n  {:8}- {}", clue, "", doc.hint);
    }
}

fn print_help(cmd: &str, doc: &ApiDoc)
{
    let clue = make_clue(cmd, &doc.params);
    println!("  {}", clue);
    for h in &doc.help
    {
        println!(" {:8}{}", "", h);
    }
}

pub fn usage(progname: String, cmd: Option<String>, docs: Vec<(&str, ApiDoc)>) -> !
{
    match cmd
    {
        None => {
            println!("{} {}\n", progname, "{command} [cmd_args ...]");
            println!("{}\n", "where {command} is one of:");
            for (cmd, doc) in docs
            {
                print_hint(cmd, &doc);
            }
        },
        Some(c) => {
            for (com, doc) in find_hints(&docs, c)
            {
                print_hint(&com, &doc);
            }
        },
    }

    std::process::exit(1);
}

fn find_hints<'a>(docs: &'a Vec<(&'a str, ApiDoc)>, cmd: String) -> Vec<&(&'a str, ApiDoc)>
{
    docs.iter()
        .filter(|d| d.0 == cmd)
        .collect::<Vec<&(&'a str, ApiDoc)>>()
}

pub fn help(progname: String, cmd: Option<String>, docs: Vec<(&str, ApiDoc)>) -> !
{
    match cmd
    {
        None => {
            println!("{} {}\n", progname, "{command} [cmd_args ...]");
            println!("{}\n", "where {command} is one of:");
            for (name, doc) in docs
            {
                print_help(name, &doc);
            }
        },
        Some(c) => {
            for (name, doc) in find_hints(&docs, c)
            {
                print_help(name, &doc);
            }
        },
    }

    std::process::exit(1);
}

pub fn help_doc() -> ApiDoc
{
    ApiDoc {
        name: "help",
        params: vec!["[cmd]"],
        hint: "The help screen, or help on a particular command if one is supplied.",
        help: vec!["The help screen, or help on a particular command if one is supplied."],
    }
}

pub fn man_doc() -> ApiDoc
{
    ApiDoc {
        name: "man",
        params: vec!["[cmd]"],
        hint: "The full help description, or full help on a particular command.",
        help: vec![
                "A long form description of the various commands.",
                "If a command name is supplied, provice the full help for that",
                "command only."
            ],
    }
}
