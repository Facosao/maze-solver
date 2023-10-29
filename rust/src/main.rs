pub mod api;
pub mod vertice;
pub mod solver;
pub mod graph;
pub mod timer;

fn main() {
    // read argsv from command line argument
    solver::solver(None, None);
}