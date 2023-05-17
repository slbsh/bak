use std::process::Command;
use regex::Regex;
use std::fs;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    if args.len() == 0 || args[0] == "-h" || args[0] == "--help" {
        println!(
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
  -uar           '-ua' but also removes the original files");
    } else if args[0] == "-v" || args[0] == "--version" {
        println!("bak 0.2.0");
    } else if args[0] == "-b" {
        args.remove(0);
        back("cp", args);
    } else if args[0] == "-br" {
        args.remove(0);
        back("mv", args);
    } else if args[0] == "-ba" {
        back("cp", detect(false));
    } else if args[0] == "-bar" {
        back("mv", detect(false));
    } else if args[0] == "-u" {
        args.remove(0);
        un_back("cp", args);
    } else if args[0] == "-ur" {
        args.remove(0);
        un_back("mv", args);
    } else if args[0] == "-ua" {
        un_back("cp", detect(true));
    } else if args[0] == "-uar" {
        un_back("mv", detect(true));
    } else {
        eprintln!("Invalid Option!\nTry 'bak --help' for more Information. ");
        std::process::exit(2);
    }
    std::process::exit(0);
}

fn detect(und: bool) -> Vec<String> {
    let mut args = vec![];
    if und {
        let paths = fs::read_dir("./")
            .expect("Could Not Read Directory!");
        let re = Regex::new("\\.bak$").unwrap();

        for path in paths {
            if re.is_match(&path.as_ref().unwrap().path().display().to_string()) {
                args.insert(args.len(), path.unwrap().path().display().to_string()); 
            }
        }
    } else {
        let paths = fs::read_dir("./")
            .expect("Could Not Read Directory!");
        for path in paths {
            args.insert(args.len(), path.unwrap().path().display().to_string());
        }
    }
    args
}

fn un_back(cmd: &str, args: Vec<String>) {
    for arg in args {
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
    for arg in args {
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
