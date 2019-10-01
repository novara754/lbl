use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

static HELP_TEXT: &'static str =
    "O [n]\t- Open file with name\tL [l-l]\t- List contents in range\n\
     A\t- Append\t\tE <l>\t- Edit line\n\
     S [n]\t- Save with name\tQ\t- Quit without saving\n\
     C\t- Start new file\tH\t- Display this help\n";

/// A very simply line-based text editor.
#[derive(StructOpt)]
#[structopt(name = "lbl")]
struct Config {
    /// File to edit (if any).
    #[structopt(name = "FILE")]
    file: Option<PathBuf>,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let mut cfg = Config::from_args();
    let mut buffer = String::new();
    if let Some(path) = cfg.file.clone() {
        let mut f = File::open(path)?;
        f.read_to_string(&mut buffer)?;
    }

    println!("{}", HELP_TEXT);

    loop {
        print!("! ");

        io::stdout().flush()?;
        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }
        if input.is_empty() {
            continue;
        }

        let args: Vec<_> = input.trim().split_whitespace().collect();
        let command = args.get(0).and_then(|s| Some(*s));
        let arg = args.get(1).and_then(|s| Some(*s));
        match (command, arg) {
            (Some("Q"), None) => break,
            (Some("H"), None) => println!("{}", HELP_TEXT),
            (Some("O"), Some(path)) => open_file(&mut buffer, path, &mut cfg)?,
            (Some("S"), name) => save_file(&buffer, name, &cfg)?,
            (Some("A"), None) => append_file(&mut buffer)?,
            (Some("L"), range) => list_file(&buffer, range),
            (Some("E"), Some(line)) => edit_file(&mut buffer, line)?,
            (Some("C"), None) => buffer = String::new(),
            _ => eprintln!("Invalid command"),
        }
    }

    Ok(())
}

fn open_file(buffer: &mut String, path: &str, cfg: &mut Config) -> io::Result<()> {
    match File::open(path) {
        Ok(mut f) => {
            *buffer = String::new();
            f.read_to_string(buffer)?;
            cfg.file = Some(PathBuf::from(path));
        },
        _ => eprintln!("Could not open file"),
    }
    Ok(())
}

fn save_file(buffer: &str, name: Option<&str>, cfg: &Config) -> io::Result<()> {
    let name = name.map(|n| PathBuf::from(n));
    let name = match (name, cfg.file.clone()) {
        (Some(path), _) => path,
        (_, Some(path)) => path,
        _ => {
            eprintln!("No file name specified");
            return Ok(());
        },
    };
    fs::write(name, &buffer)
}

fn append_file(buffer: &mut String) -> io::Result<()> {
    let mut line_count = buffer.lines().count();
    loop {
        print!("{:4}| ", line_count);
        io::stdout().flush()?;

        let mut line = String::new();
        if io::stdin().read_line(&mut line)? == 0 {
            println!();
            break;
        }

        buffer.push_str(&line);
        line_count += 1;
    }

    Ok(())
}

fn edit_file(buffer: &mut String, line: &str) -> io::Result<()> {
    let mut line_count: usize = match line.parse() {
        Ok(line) => line,
        _ => {
            eprintln!("Invalid line");
            return Ok(());
        },
    };

    let mut lines: Vec<_> = buffer.lines().map(|l| l.to_owned()).collect();
    let max_line_count = lines.len();
    while line_count < max_line_count {
        print!("{:4}| ", line_count);
        io::stdout().flush()?;

        let mut line = String::new();
        if io::stdin().read_line(&mut line)? == 0 {
            println!();
            *buffer = lines.join("\n");
            return Ok(());
        }

        lines[line_count] = line.trim().to_owned();
        line_count += 1;
    }
    *buffer = lines.join("\n");

    append_file(buffer)
}

fn list_file(buffer: &str, range: Option<&str>) {
    if buffer.is_empty() {
        println!("<empty>");
        return;
    }

    if let Some(range) = range {
        let range: Vec<_> = range.split('-').collect();
        let lower = range.get(0);
        let upper = range.get(1);

        if lower.is_none() || upper.is_none() {
            eprintln!("Invalid range");
            return;
        }

        let lower: Result<usize, _> = lower.unwrap().parse();
        let upper: Result<usize, _> = upper.unwrap().parse();

        if lower.is_err() || upper.is_err() {
            eprintln!("Invalid range");
            return;
        }

        let lower = lower.unwrap();
        let upper = upper.unwrap();

        buffer
            .lines()
            .skip(lower)
            .take(upper - lower)
            .enumerate()
            .for_each(|(c, l)| println!("{:4}| {}", c, l));
    } else {
        buffer
            .lines()
            .enumerate()
            .for_each(|(c, l)| println!("{:4}| {}", c, l));
    }
}
