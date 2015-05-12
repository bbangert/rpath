extern crate rustc_serialize;
extern crate docopt;

use std::env;
use docopt::Docopt;

// Write the Docopt usage string.
static USAGE: &'static str = "
Usage:
    rpath (-h | --help)
    rpath list
    rpath remove <location>
    rpath insert <location> <path>
    rpath replace <location> <path>

Options:
    -h --help     Show this screen.
    -v            Verbose.
";

#[derive(RustcDecodable, Debug)]
struct Args {
    cmd_remove: bool,
    cmd_list: bool,
    cmd_insert: bool,
    cmd_replace: bool,
    arg_location: Option<usize>,
    arg_path: Option<String>,
    flag_v: bool
}

fn print_path(path: &str) {
    println!("Location:  Path");
    for (i, part) in path.split(":").enumerate() {
        println!("{:>8}:  {}", i, part)
    }
}


fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let path = env::var("PATH").unwrap();
    let mut parts: Vec<&str> = path.split(":").collect();
    match args {
        Args { arg_location: Some(loc), .. } if loc >= parts.len() =>
            println!("Error: location is not valid. For a list of locations, run: rpath list"),
        Args { cmd_list: true, .. } => print_path(&path),
        Args { cmd_remove: true, arg_location: Some(loc), .. } => {
            parts.remove(loc);
            print!("{}", parts.connect(":"))
        },
        // Note that this handles both insert/replace
        Args { cmd_replace: is_replace, arg_location: Some(loc), arg_path: Some(ref ipath), .. } => {
            if is_replace {
                parts.remove(loc);
            }
            parts.insert(loc, ipath);
            print!("{}", parts.connect(":"))
        },
        _ => println!("Must have forgotten an option..."),
    }
}
