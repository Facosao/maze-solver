use std::collections::HashMap;

use crate::vertice::Vertice;
use crate::api::API;

pub struct Graph {
    pub vertices: HashMap<i32, Vertice>
}

impl Graph {
    pub fn novo() -> Self {
        Graph { vertices: HashMap::new() }
    }

    fn dfs_status(api: &API, indice: i32) {
        print!("\r                                                       ");
        print!("\r--- API Calls: {} | pos_atual: {}", api._n_calls, indice);
    }

    pub fn dfs(&mut self, api: &API, indice: i32) {
        let mut inicio = true;
        let mut anterior: i32 = -1;
        let mut pilha: Vec<i32> = Vec::new();

        self.vertices.get_mut(&indice).unwrap().explorado = true;
        pilha.push(indice);


        while pilha.len() > 0 {
            let atual = pilha.pop().unwrap();
            //Self::dfs_status(api, atual);

            // movimentar para nó
            if inicio == false {
                api.movimentar(&mut self.vertices, atual, anterior);
            }

            let vec_adj = self.vertices.get(&atual).unwrap().adjacencias.clone();

            for aux in vec_adj.iter() {
                if self.vertices.contains_key(aux) == false {
                    self.vertices.insert(*aux, Vertice { 
                        id: *aux,
                        anterior: -1,
                        explorado: false,
                        adjacencias: Vec::new(),
                        inicio: false,
                        fim: false
                    });
                }
                
                let no_adj = self.vertices.get_mut(aux).unwrap();
                if no_adj.explorado == false {
                    no_adj.explorado = true;
                    no_adj.anterior = atual;
                    pilha.push(*aux);
                }
            }

            // voltar para vértice anterior
            if self.vertices.get(&atual).unwrap().inicio == false {
                api.movimentar(&mut self.vertices, anterior, indice);
            }

            anterior = atual;
            inicio = false;
        }
    }
}

