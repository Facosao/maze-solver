use std::collections::{HashMap, HashSet};

use crate::api::API;
use crate::graph::Graph;
use crate::vertice::Vertice;

// Explora o grafo com BFS
pub fn explorer(graph: &mut Graph, api: &mut API, indice_inicial: i32) -> i32 {
    let mut fila: Vec<i32> = Vec::new();
    let mut start = true;
    let mut current_node: i32 = indice_inicial;
    let mut anterior: i32;

    graph.vertices.get_mut(&indice_inicial).unwrap().explorado = true;
    fila.push(indice_inicial);

    while !fila.is_empty() {
        anterior = current_node;
        let v = fila.remove(0);

        if start == false {
            if current_node == v {    
                continue;
            } else {
                if !graph.vertices[&current_node].adjacencias.contains(&v) {
                    let mut path_to_node = pathfinder(&graph, current_node, v);
                    path_to_node.extend_from_slice(&fila);
                    fila = path_to_node;
                    continue;
                } else {
                    api.move_to(&mut graph.vertices, v, anterior);
                    current_node = v;
                }
            }
        }

        let vert = graph.vertices[&v].clone();

        if vert.fim == true {
            return v;
        }

        for vertice in vert.adjacencias {
            if !graph.vertices.contains_key(&vertice) {
                // Insert incomplete vertex
                graph.vertices.insert(vertice, Vertice::novo(vertice, vert.id));
            }

            let vert_adj = graph.vertices
                .get_mut(&vertice)
                .unwrap();

            if vert_adj.explorado == false {
                vert_adj.explorado = true;
                vert_adj.anterior = vert.id;
                fila.push(vertice);
            }
        }

        start = false;
    }
    
    return -1;
}

pub fn pathfinder(graph: &Graph, start: i32, end: i32) -> Vec<i32> {
    let mut previous: HashMap<i32,i32> = HashMap::new();
    let mut visited: HashSet<i32> = HashSet::new();
    let mut queue: Vec<i32> = Vec::new();

    visited.insert(start);
    previous.insert(start, -1);
    queue.push(start);

    while !queue.is_empty() {
        let v = queue.remove(0);

        let vert: Vertice;

        match graph.vertices.get(&v) {
            Some(vertex) => vert = vertex.clone(),
            None => continue,
        }

        for vertex in vert.adjacencias {
            if !visited.contains(&vertex) {
                visited.insert(vertex);
                previous.insert(vertex, v);
                queue.push(vertex);
            }

            if vertex == end {
                // Generate path to return
                let mut path: Vec<i32> = Vec::new();
                let mut aux = vertex;
                
                loop {
                    path.push(aux);
                    if previous[&aux] != -1 {
                        aux = previous[&aux];      
                    } else {
                        break;
                    }
                }

                path.reverse();
                return path;
            }
        }
    }

    return queue;
}