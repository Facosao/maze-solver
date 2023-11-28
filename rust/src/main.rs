pub mod api;
pub mod vertice;
pub mod solver;
pub mod graph;
pub mod timer;
pub mod strategy;

use crate::strategy::Strategy;

fn print_help() {
    println!("Usage: ./solver [OPTION] [ARG] --maze [MAZE] [STRATEGY]\n");
    println!("Required arguments:");
    println!("[MAZE]       Maze name for the program to use");
    println!("");
    println!("[STRATEGY]   Selects which strategy the program will use.");
    println!("Available strategies:");
    println!("--dfs        Explore with DFS until exit is found. Return DFS path.");
    println!("--bfs        Explore with BFS until exit is found. Retrun BFS path.");
    println!("--dfsfullbfs Explore with DFS until all nodes are explored. Return path from BFS executed in RAM.");
    println!("--dfsexitbfs Explore with DFS until exit is found. Return path from BFS executed in RAM.");
    println!("");
    println!("Optional arguments:");
    println!("[OPTION]     [OPTION ARG]");
    println!("--url        Specify a custom URL for the API Calls.");
    println!("             Default URL: https://gtm.delary.dev");
    println!("");
    println!("--custom     Specify a custom end for the program. (integer arg)");
    println!("");
    println!("--help       Displays this help text. (No args)");
    println!("");

}

fn main() {
    println!("maze-solver (Rust)");

    let mut address: Option<String> = None;
    let mut maze: Option<String> = None;
    let mut strat: Option<Strategy> = None;
    let mut custom_end: Option<i32> = None;

    let args: Vec<_> = std::env::args().collect();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--url" => address = Some(args[i + 1].clone()),

            "--maze" => maze = Some(args[i + 1].clone()),

            "--custom" => custom_end = Some(args[i + 1].clone().parse().unwrap()),

            "--bfstotal" => strat = Some(Strategy::DFSBFSTotal),

            "--bfsparcial" => strat = Some(Strategy::DFSBFSParcial),
            
            "--dfs" => strat = Some(Strategy::DFS),

            "--iddfs" => strat = Some(Strategy::IDDFS),

            "--dbfs" => strat = Some(Strategy::DBFS),

            "--help" => {
                print_help();
                return;
            }

            &_ => {}
        }

        i += 1;
    }
    
    if (maze.is_none()) && (strat.is_none()) {
        print_help();
        return;
    }

    solver::solver(address, maze, strat, custom_end);
}