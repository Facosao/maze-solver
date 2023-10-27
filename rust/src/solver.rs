use crate::api::API;
use crate::graph::Graph;

pub fn solver(address: Option<&str>, maze: Option<&str>) {
    // start timer
    let api = API::novo(address, maze);
    let mut graph = Graph::novo();
    
    println!("maze-solver (Rust)");
    println!("1 - Fazendo chamada inicial (Labirinto: {})", api.maze);
    let indice_inicial = api.iniciar(&mut graph.vertices).unwrap();

    println!("2 - Explorando o labirinto (API) com o DFS");
    graph.dfs(&api, indice_inicial);    
    println!(""); // Nova linha após chamadas recursivas

    for value in graph.vertices.values() {
        println!("--- {}: {:?}", value.id, value.adjacencias);
    }

    return; // temp

    println!("3 - Resetando o estado dos nos");
    // resetar nos

    println!("4 - Explorando o labirinto (RAM) com o BFS");
    //let indice_final = bfs();

    println!("5 - Encontrando o menor caminho");
    //let menor_caminho = encontrar_caminho()
    //println!("--- {:?}", menor_caminho);

    println!("6 - Validando o menor caminho encontrado");
    //api.validar_caminho(caminho);

    // stop timer

    println!("7 - Estatisticas finais");
    //println!("--- API Calls: {}", );
    //println!("--- Tempo total do programa  : {:.3}", );
    //println!("--- Tempo total das API Calls: {:.3}", );
    //let proporcao: f64 = (api.timer.total * 100) / local.timer.total;
    //println!("--- ({:.2}% do total do programa)", proporcao);
}