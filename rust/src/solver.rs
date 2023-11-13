use crate::api::API;
use crate::graph::Graph;
use crate::timer::Timer;
use crate::strategy::Strategy;

pub fn solver(address: Option<String>, maze: Option<String>, strat: Strategy) {
    let mut timer = Timer::novo();
    let mut api = API::novo(address, maze);
    let mut graph = Graph::novo();
    
    println!("1 - Fazendo chamada inicial (URL: {} | Labirinto: {} | Modo: {:?})", api.url, api.maze, strat);
    let indice_inicial = api.iniciar(&mut graph.vertices).unwrap();

    let indice_final: i32 = match strat {
        Strategy::DFSBFSTotal => {
            println!("2.1 Explorando todo o labirinto com o DFS");
            graph.dfs_total(&mut api, indice_inicial, -1, true); 
            println!(""); // Nova linha após chamadas recursivas

            println!("2.2 Resetando o estado dos nos");
            graph.restaurar_nos();

            println!("2.3 Encontrando o menor caminho com o BFS total");
            graph.bfs_ram(indice_inicial)
        }

        Strategy::DFSBFSParcial => {
            println!("2.1 Explorando o labirinto ate o alvo com o DFS");
            graph.dfs_alvo(&mut api, indice_inicial, -1, true);
            println!(""); // Nova linha após chamadas recursivas
            
            println!("2.2 Resetando o estado dos nos");
            graph.restaurar_nos();

            println!("2.3 Encontrando o menor caminho com o BFS parcial");
            graph.bfs_ram(indice_inicial)
        }

        Strategy::DFS => {
            println!("2 - Explorando o labirinto ate o alvo com o DFS");
            graph.dfs_alvo(&mut api, indice_inicial, -1, true);
            println!(""); // Nova linha após chamadas recursivas
            graph.indice_final.unwrap()
        }

        Strategy::IDDFS => {
            println!("2 - Explorando o labirinto ate o alvo com o IDDFS");
            let aux = graph.iddfs(&mut api, indice_inicial);
            println!(""); // Nova linha após chamadas recursivas
            aux
        }

        Strategy::DBFS => {
            println!("2 - Explorando o labirinto ate o alvo com o BFS");
            let aux = graph.bfs_explorer(&mut api, indice_inicial);
            println!("");
            aux
        }
    };

    println!("--- Vertices explorados: {}", graph.vertices.values().len());

    println!("3 - Gerando a lista com o caminho encontrado");
    let menor_caminho = graph.encontrar_caminho(indice_final);
    println!("--- {:?}", menor_caminho);

    println!("4 - Validando o caminho encontrado");
    api.validar_caminho(menor_caminho);

    timer.parar();

    println!("5 - Estatisticas finais");
    println!("--- API Calls: {}", api.n_calls);
    println!("--- Tempo total do programa  : {:.3}", timer.total());
    println!("--- Tempo total das API Calls: {:.3}", api.timer.total());
    let proporcao: f64 = (api.timer.total() * 100.0) / timer.total();
    println!("--- ({:.2}% do total do programa)", proporcao);
}