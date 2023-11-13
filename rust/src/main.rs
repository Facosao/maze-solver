pub mod api;
pub mod vertice;
pub mod solver;
pub mod graph;
pub mod timer;
pub mod strategy;

use crate::strategy::Strategy;

fn main() {
    let mut address: Option<String> = None;
    let mut maze: Option<String> = None;
    let mut strat: Strategy = Strategy::DFSBFSTotal;

    let args: Vec<_> = std::env::args().collect();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--url" => address = Some(args[i + 1].clone()),

            "--maze" => maze = Some(args[i + 1].clone()),

            "--bfstotal" => strat = Strategy::DFSBFSTotal,

            "--bfsparcial" => strat = Strategy::DFSBFSParcial,
            
            "--dfs" => strat = Strategy::DFS,

            "--iddfs" => strat = Strategy::IDDFS,

            "--dbfs" => strat = Strategy::DBFS,

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

        i += 1;
    }

    solver::solver(address, maze, strat);
}