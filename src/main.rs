extern crate rustc_serialize;
extern crate docopt;

use std::env;
use docopt::Docopt;

// Write the Docopt usage string.
static USAGE: &'static str = "
Usage:
    rpath (-h | --help)
    rpath list
    rpath rm <location>

Options:
    -h --help     Show this screen.
    -v            Verbose.
";

#[derive(RustcDecodable, Debug)]
struct Args {
    cmd_rm: bool,
    cmd_list: bool,
    arg_location: Option<usize>,
    flag_v: bool
}

fn print_path(path: &str) {
    for (i, part) in path.split(":").enumerate() {
        println!("{}: {}", i, part)
    }
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    match args {
        Args { cmd_list: true, .. } => {
            let path = env::var("PATH").unwrap();
            print_path(&path)
        },
        Args { cmd_rm: true, arg_location: Some(loc), .. } => {
            let path = env::var("PATH").unwrap();
            let mut parts: Vec<&str> = path.split(":").collect();
            if loc > (parts.len()-1) {
                println!("Location is outside boundaries for PATH. Run rpath list to see boundaries.");
                return
            }
            parts.remove(loc);
            let newpath = parts.connect(":");
            print!("{}", newpath)
        },
        _ => println!("Must have forgotten an option..."),
    }
}
