use std::env;
use std::fs;

#[derive(Debug)]
enum Command {
    USCC,
    CCUS
}

impl Command {
    fn new(s: &str) -> Result<Command, &str> {
        match s.to_ascii_lowercase().as_str() {
            "uscc" => Ok(Command::USCC),
            "ccus" => Ok(Command::CCUS),
            _ => Err("Unknown command")
        }
    }
}

#[derive(Debug)]
struct Config {
    command: Command,
    inp_file: String,
    out_file: String
}

impl Config {

    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 1 {
            return Err("Not enough arguments")
        }
        let mut it = args.iter();
        let option = it.next().unwrap().as_str();

        let command = match Command::new(option) {
            Ok(i) => i,
            Err(i) =>  return Err(i)
        };

        let inp_file = match it.next() {
            Some(i) => i.to_string(),
            None => String::from("in.txt")
        };

        let out_file = match it.next() {
            Some(i) => i.to_string(),
            None => String::from("out.txt")
        };

        Ok(Config {
            command,
            inp_file,
            out_file
        })
    }
}
fn ccus(contents: String) -> String {
    contents
}

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn uscc(contents: String) -> String {
    println!("{}", contents);
    let mut res :Vec<String> = Vec::new();
    let i = contents.lines()
        .enumerate()
        .filter(|(_i,l)| { !l.is_empty() })
        .filter(|(i,l)| {
            let has_whitespace = l.trim().contains(char::is_whitespace);
            if has_whitespace {
                println!("Ignoring line because of whitespace: {} {}", i, l);
            }
            !has_whitespace
        })
        .map(|(_i, mut l)| {
            l = l.trim();
            let mut processed: String = String::new();
            for (i, s) in l.split("_").enumerate()  {
                if i == 0 { processed.push_str(s.to_lowercase().as_str()) }
                else {
                    let (first, rest) = s.split_at(1);
                    processed.push_str(format!("{}{}", first.to_uppercase(), rest.to_lowercase()).as_str())
                }
            }
            res.push(processed);
        })
        .count();
    println!("{} lines processed.", i);
    res.join(LINE_ENDING)
}


fn run(args: Vec<String>) -> String {
    let conf = Config::new(&args);

    let config = match conf {
        Ok(i) => i,
        Err(i) => return String::from(i)
    };

    println!("{:?}", config);

    let contents = match fs::read_to_string(config.inp_file) {
        Ok(c) => c,
        Err(e) => return format!("Unable to read file : {}", e)
    };

    let out = match config.command {
        Command::CCUS => ccus(contents),
        Command::USCC => uscc(contents)
    };

    match fs::write(config.out_file.clone(), out) {
        Ok(()) =>  "Ok".to_string(),
        Err(e) => format!("Error while writing output file: {}, err: {}", config.out_file, e)
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let res = run(args);
    println!("{}", res)
}

// Tests

#[test]
fn test_run_should_return_err() {
    let mut test_args = vec![];
    assert_eq!(run(test_args), String::from("Not enough arguments"));

    test_args = vec![
        "UnknownCommand".to_string()
    ];

    assert_eq!(run(test_args), String::from("Unknown command"));
}

#[test]
fn test_run_should_return_ok() {
    let test_args = vec![
        String::from("USCC")
    ];
    assert_eq!(run(test_args), String::from("Ok"));
}
