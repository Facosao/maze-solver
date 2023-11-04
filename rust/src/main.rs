pub mod api;
pub mod vertice;
pub mod solver;
pub mod graph;
pub mod timer;

fn main() {
    let mut address: Option<String> = None;
    let mut maze: Option<String> = None;

    println!("maze-solver (Rust)");

    let mut iter = std::env::args();

    match iter.next() {
        Some(arg) => {
            match arg.as_str() {
                "--url" => {
                    address = Some(iter.next().unwrap());
                }

                "--maze" => {
                    maze = Some(iter.next().unwrap());
                }

                "--help" => {
                    println!("Usage: main.py [OPTION] [ARG]\n");
                    println!(" [OPTION]  [OPTION ARG]");
                    println!(" --url     Specify a custom URL for the API Calls.");
                    println!("           Default URL: https://gtm.delary.dev");
                    println!("");
                    println!(" --maze    Specify a custom maze for the program.");
                    println!("           Default maze: maze-sample");
                    println!("");
                    println!(" --help    Displays this help text. (No args)");
                    println!("");
                    return;
                }

                &_ => {}
            }
        }
        
        None => {}
    }

    solver::solver(address, maze);
}