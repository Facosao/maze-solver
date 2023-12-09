use crate::api::API;
use crate::graph::Graph;
use crate::vertice::Vertice;

fn is_full_graph(graph: &Graph, stack: &Vec<i32>) -> bool {
    for value in stack {
        match graph.vertices.get(&value) {
            Some(vertex) => {
                if vertex.explorado == false {
                    return false;
                }
            }

            None => continue,
        }
    }

    return true;
}

pub fn dfs(graph: &mut Graph, api: &mut API, inicio: i32, early_return: bool) {
    let mut pilha: Vec<i32> = Vec::new();
    let mut anterior: i32 = -1;
    pilha.push(inicio);

    while !pilha.is_empty() {
        if is_full_graph(&graph, &pilha) {
            return;
        }

        let v = pilha.pop().unwrap();
        if v != inicio {
            //println!("M {}", v);
            Graph::graph_status(api, v);
            api.move_to(&mut graph.vertices, v, anterior);
            //pilha.push(anterior);
        }
        graph.vertices.get_mut(&v).unwrap().explorado = true;

        let mut vertice = graph.vertices[&v].clone();
        vertice.adjacencias.reverse();
        pilha.push(vertice.anterior);

        if vertice.fim {
            graph.indice_final = Some(v);
            if early_return {
                return;
            }
        }

        for adjacencia in vertice.adjacencias {
            if !graph.vertices.contains_key(&adjacencia) {
                // Insert incomplete vertex
                graph.vertices.insert(adjacencia, Vertice::novo(adjacencia, v));
            }

            let vtx_adj = graph.vertices.get_mut(&adjacencia).unwrap();
            if vtx_adj.explorado == false {
                vtx_adj.anterior = v;
                pilha.push(adjacencia);
            }
        }

        anterior = v;
    }
}