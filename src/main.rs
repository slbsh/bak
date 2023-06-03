use std::process::{Command, exit};
use std::fs;

static HELP_MESSAGE: &str =
"bak - Simple File Backup
Usage: bak <OPTION> [PATHTOFILE]...

Options:
  -h, --help     Show This Message
  -v, --version  Print Version
  -b             Backup the given Files within their Respecitive Directories
  -br            '-b' but removes the original files
  -ba            '-b' but for all files in the Working Directory
  -bar           '-ba' but removes the original files
  -u             Removes the '.bak' extension from all provided files
  -ur            '-u' but removes the original files
  -ua            '-u' but for all '.bak' files within the Working Directory
  -uar           '-ua' but also removes the original files";

static VERSION: &str = "bak 0.3.0";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        eprintln!("{}", HELP_MESSAGE);
        exit(0)
    }
    
    match args[0].to_lowercase().as_str() {
        "-h" | "--help" => eprintln!("{}", HELP_MESSAGE),
        "-v" | "--version" => eprintln!("{}", VERSION),
        "-b" => back("cp", args),
        "-br" => back("mv", args),
        "-ba" => back("cp", detect()),
        "-bar" | "-bra" => back("mv", detect()),
        "-u" => un_back("cp", args),
        "-ur" => un_back("mv", args),
        "-ua" => un_back("cp", detect_bak()),
        "-uar" | "-ura" => un_back("mv", detect_bak()),
         _ => eprintln!("Invalid Option!\nTry 'bak --help' for more Information."),
    }
    exit(0);
}

fn detect() -> Vec<String> {
    let mut args = vec!["".to_string()];
    fs::read_dir(".")
        .expect("Could Not Read Directory!")
        .filter_map(Result::ok)
        .filter_map(|path| path.file_name().into_string().ok())
        .for_each(|file| args.push(file));
    args
}

fn detect_bak() -> Vec<String> {
    let mut args = vec!["".to_string()];
    fs::read_dir(".")
        .expect("Could Not Read Directory!")
        .filter_map(Result::ok)
        .filter_map(|path| path.file_name().into_string().ok().filter(|name| name.ends_with(".bak")))
        .for_each(|file| args.push(file));
    args
}

fn un_back(cmd: &str, args: Vec<String>) {
    for arg in args.iter().skip(1) {
        let out = Command::new(cmd)
            .arg("-v")
            .arg(arg.clone())
            .arg(arg.replace(".bak", ""))
            .output()
            .expect("Shit went down!")
            .stdout; 
        let out = String::from_utf8_lossy(&out);
        print!("{}", out);
    }
}

fn back(cmd: &str, args: Vec<String>) {
    for arg in args.iter().skip(1) {
        let out = Command::new(cmd)
            .arg("-v")
            .arg(arg.clone())
            .arg(arg.clone() + ".bak")
            .output()
            .expect("Shit went down!")
            .stdout; 
        let out = String::from_utf8_lossy(&out);
        print!("{}", out);
    }
}
