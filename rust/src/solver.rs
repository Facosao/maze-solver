use crate::api::API;
use crate::graph::Graph;
use crate::timer::Timer;

pub fn solver(address: Option<String>, maze: Option<String>) {
    let mut timer = Timer::novo();
    
    let mut api = API::novo(address, maze);
    let mut graph = Graph::novo();
    
    println!("1 - Fazendo chamada inicial (Labirinto: {})", api.maze);
    let indice_inicial = api.iniciar(&mut graph.vertices).unwrap();

    println!("2 - Explorando o labirinto (API) com o DFS");
    graph.dfs_recursivo(&mut api, indice_inicial, -1, true);    
    println!(""); // Nova linha após chamadas recursivas

    for value in graph.vertices.values() {
        println!("--- {}: {:?}", value.id, value.adjacencias);
    }

    //println!("3 - Resetando o estado dos nos");
    //graph.restaurar_nos();

    //println!("4 - Explorando o labirinto (RAM) com o BFS");
    //let indice_final = graph.bfs(indice_inicial);

    let indice_final = graph.indice_final.unwrap();

    println!("5 - Encontrando o menor caminho");
    let menor_caminho = graph.encontrar_caminho(indice_final);
    println!("--- {:?}", menor_caminho);

    println!("6 - Validando o menor caminho encontrado");
    api.validar_caminho(menor_caminho);

    timer.parar();

    println!("7 - Estatisticas finais");
    println!("--- API Calls: {}", api.n_calls);
    println!("--- Tempo total do programa  : {:.3}", timer.total());
    println!("--- Tempo total das API Calls: {:.3}", api.timer.total());
    let proporcao: f64 = (api.timer.total() * 100.0) / timer.total();
    println!("--- ({:.2}% do total do programa)", proporcao);
}