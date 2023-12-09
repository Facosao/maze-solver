use core::panic;

use crate::api::API;
use crate::graph::Graph;
use crate::timer::Timer;
use crate::strategy::Strategy;

pub fn solver(address: Option<String>, maze: Option<String>, strategy: Option<Strategy>, custom_end: Option<i32>) {
    let mut timer = Timer::new();
    let mut api = API::novo(address, maze);
    let mut graph = Graph::novo();
    
    println!("I - Initial API call (URL: {} | Maze: {} | Strategy: {:?})", api.url, api.maze, strategy);
    let first_pos = api.start(&mut graph.vertices, custom_end).unwrap();

    let path: Vec<i32> = match strategy {
        None => {
            panic!("No strategy selected!");
        }

        Some(strat) => match strat {
            Strategy::DFSFullBFS => {
                println!("2.1 Explorando todo o labirinto com o DFS");
                //graph.dfs_total(&mut api, indice_inicial, -1, true); 
                graph.dfs_otimizado(&mut api, first_pos, false);
                println!(""); // Nova linha após chamadas recursivas
    
                println!("2.2 Resetando o estado dos nos");
                graph.restaurar_nos();
    
                println!("2.3 Encontrando o menor caminho com o BFS");
                //graph.bfs_ram(indice_inicial)
                graph.bfs_restricted(first_pos, graph.indice_final.unwrap())
            }
    
            Strategy::DFSExitBFS => {
                println!("2.1 Explorando o labirinto ate o alvo com o DFS");
                //graph.dfs_alvo(&mut api, indice_inicial, -1, true);
                graph.dfs_otimizado(&mut api, first_pos, true);
                println!(""); // Nova linha após chamadas recursivas
                
                println!("2.2 Resetando o estado dos nos");
                graph.restaurar_nos();
    
                println!("2.3 Encontrando o menor caminho com o BFS");
                //graph.bfs_ram(indice_inicial)
                graph.bfs_restricted(first_pos, graph.indice_final.unwrap())
            }
    
            Strategy::DFS => {
                println!("2 - Explorando o labirinto ate o alvo com o DFS");
                //graph.dfs_alvo(&mut api, indice_inicial, -1, true);
                graph.dfs_otimizado(&mut api, first_pos, true);
                println!(""); // Nova linha após chamadas recursivas
                graph.encontrar_caminho(graph.indice_final.unwrap())
            }
    
            Strategy::BFS => {
                println!("2 - Explorando o labirinto ate o alvo com o BFS");
                let aux = graph.bfs_explorer(&mut api, first_pos);
                println!("");
                graph.encontrar_caminho(aux)
            }
        }
    };

    println!("3 - Gerando a lista com o caminho encontrado");
    println!("--- {:?}", path);

    println!("4 - Validating found path");
    api.validate_path(path);

    timer.stop();

    println!("5 - Final statistics");
    println!("--- API Calls: {}", api.n_calls);
    println!("--- Explored nodes: {}", graph.vertices.values().len());
    println!("--- Total running time  : {:.3}", timer.total());
    println!("--- Time spent on API Calls: {:.3}", api.timer.total());
    let ratio: f64 = (api.timer.total() * 100.0) / timer.total();
    println!("--- ({:.2}% of running time)", ratio);
}