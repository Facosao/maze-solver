use std::collections::HashMap;

use crate::vertice::Vertice;
use crate::api::API;

pub struct Graph {
    pub vertices: HashMap<i32, Vertice>,
    pub indice_final: Option<i32>
}

impl Graph {
    pub fn novo() -> Self {
        Graph {
            vertices: HashMap::new(),
            indice_final: None
        }
    }

    pub fn restaurar_nos(&mut self) {
        for value in self.vertices.values_mut() {
            value.explorado = false;
        }
    }

    pub fn encontrar_caminho(&self, indice_final: i32) -> Vec<i32> {
        let mut caminho: Vec<i32> = Vec::new();
        let mut aux = &self.vertices[&indice_final];

        loop {
            if aux.inicio == false {
                caminho.push(aux.id);
                aux = &self.vertices[&aux.anterior];
            } else {
                caminho.push(aux.id);
                break;
            }
        }

        caminho.reverse();
        return caminho;
    }

    pub fn graph_status(api: &API, indice: i32) {
        print!("\r                                                      ");
        print!("\r--- API Calls: {} | pos_atual: {}", api.n_calls, indice);
    }
}
