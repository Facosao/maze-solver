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
        print!("\r--- API Calls: {} | pos_atual: {}", api.n_calls, indice);
    }

    pub fn dfs(&mut self, api: &mut API, indice: i32) {
        let mut inicio = true;
        let mut anterior: i32 = -1;
        let mut pilha: Vec<i32> = Vec::new();

        self.vertices.get_mut(&indice).unwrap().explorado = true;
        pilha.push(indice);


        while pilha.len() > 0 {
            let atual = pilha.pop().unwrap();
            Self::dfs_status(api, atual);

            // movimentar para nó
            if inicio == false {
                api.movimentar(&mut self.vertices, atual, anterior);
            }

            let vec_adj = self.vertices.get(&atual).unwrap().adjacencias.clone();

            for aux in vec_adj.iter() {
                if self.vertices.contains_key(aux) == false {
                    self.vertices.insert(*aux, Vertice::novo(*aux, atual));
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

    pub fn dfs_recursivo(&mut self, api: &mut API, indice: i32, anterior: i32, inicio: bool) {
        Self::dfs_status(api, indice);

        if inicio == false {
            api.movimentar(&mut self.vertices, indice, anterior);
        }

        self.vertices.get_mut(&indice).unwrap().explorado = true;
        let vec_adj = self.vertices.get(&indice).unwrap().adjacencias.clone();

        for adj in vec_adj.iter() {
            match self.vertices.get_mut(adj) {
                None => {
                    self.dfs_recursivo(api, *adj, indice, false);
                }

                Some(no_adj) => {
                    if no_adj.explorado == false {
                        self.dfs_recursivo(api, *adj, indice, false);
                    }
                }
            }
        }

        let no = self.vertices.get(&indice).unwrap().clone();

        if no.inicio == false {
            api.movimentar(&mut self.vertices, no.anterior, indice);
        }

        Self::dfs_status(api, indice);        
    }

    pub fn restaurar_nos(&mut self) {
        for value in self.vertices.values_mut() {
            value.explorado = false;
        }
    }

    pub fn bfs(&mut self, raiz: i32) -> i32 {
        self.vertices.get_mut(&raiz).unwrap().explorado = true;

        let mut fila: Vec<i32> = Vec::new();
        fila.push(self.vertices.get(&raiz).unwrap().id);

        while fila.len() > 0 {
            let no = self.vertices
                .get(&fila.remove(0))
                .unwrap()
                .clone();

            if no.fim == true {
                return no.id; 
            }

            for adj in no.adjacencias {
                let no_adj = self.vertices.get_mut(&adj).unwrap();

                if no_adj.explorado == false {
                    no_adj.explorado = true;
                    no_adj.anterior = no.id;
                    fila.push(no_adj.id);
                }
            }
        }

        return -1; // Inalcançável
    }

    pub fn encontrar_caminho(&self, indice_final: i32) -> Vec<i32> {
        let mut caminho: Vec<i32> = Vec::new();
        let mut aux = self.vertices.get(&indice_final).unwrap();

        loop {
            if aux.inicio == false {
                caminho.push(aux.id);
                aux = self.vertices.get(&aux.anterior).unwrap();
            } else {
                caminho.push(aux.id);
                break;
            }
        }

        caminho.reverse();
        return caminho;
    }
}

