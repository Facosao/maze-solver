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

    fn deletar_nos(&mut self, indice_inicial: i32) {
        let vertice_inicial = self.vertices
            .get(&indice_inicial)
            .unwrap()
            .clone();
        
        let mut keys: Vec<i32> = Vec::new();
        for key in self.vertices.keys() {
            keys.push(*key);
        }
        
        for key in keys {
            self.vertices.remove(&key);
        }

        self.vertices.insert(indice_inicial, vertice_inicial);
    }

    // Usado apenas com todos os nós na memória
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

    fn dfs_status(api: &API, indice: i32) {
        print!("\r                                                      ");
        print!("\r--- API Calls: {} | pos_atual: {}", api.n_calls, indice);
    }

    // Explora todo o grafo e volta para o inicio
    pub fn dfs_total(&mut self, api: &mut API, indice: i32, anterior: i32, inicio: bool) {      
        Self::dfs_status(api, indice);

        if inicio == false {
            api.movimentar(&mut self.vertices, indice, anterior);
        }

        self.vertices.get_mut(&indice).unwrap().explorado = true;
        let no = self.vertices[&indice].clone();

        for adj in no.adjacencias.iter() {
            match self.vertices.get_mut(adj) {
                None => {
                    self.dfs_total(api, *adj, indice, false);
                }

                Some(no_adj) => {
                    if no_adj.explorado == false {
                        self.dfs_total(api, *adj, indice, false);
                    }
                }
            }
        }

        // Redundante?
        // let no = self.vertices.get(&indice).unwrap().clone();

        if no.inicio == false {
            api.movimentar(&mut self.vertices, no.anterior, indice);
        }

        Self::dfs_status(api, indice);        
    }

    // Explora o grafo até achar o alvo
    pub fn dfs_alvo(&mut self, api: &mut API, indice: i32, anterior: i32, inicio: bool) {
        if self.indice_final.is_some() {
            return;
        }
        
        Self::dfs_status(api, indice);

        if inicio == false {
            api.movimentar(&mut self.vertices, indice, anterior);
        }

        self.vertices.get_mut(&indice).unwrap().explorado = true;

        let no = self.vertices[&indice].clone();

        if no.fim == true {
            self.indice_final = Some(indice);
            return;
        }

        for adj in no.adjacencias.iter() {
            match self.vertices.get_mut(adj) {
                None => {
                    self.dfs_alvo(api, *adj, indice, false);
                }

                Some(no_adj) => {
                    if no_adj.explorado == false {
                        self.dfs_alvo(api, *adj, indice, false);
                    }
                }
            }
        }

        // Redundante?
        // let no = self.vertices.get(&indice).unwrap().clone();

        if (no.inicio == false) && (self.indice_final.is_none()) {
            api.movimentar(&mut self.vertices, no.anterior, indice);
        }

        Self::dfs_status(api, indice);        
    }

    // DFS com profundidade limitada; Explora até encontrar o alvo
    fn dfs_depth(&mut self, api: &mut API, indice: i32, anterior: i32, depth: i32, inicio: bool) {
        if self.indice_final.is_some() {
            return;
        }
        
        Self::dfs_status(api, indice);

        if inicio == false {
            //println!("M {}", indice);
            api.movimentar(&mut self.vertices, indice, anterior);
        } //else {
            //println!("START {}", indice);
        //}

        self.vertices.get_mut(&indice).unwrap().explorado = true;

        let no = self.vertices[&indice].clone();

        if no.fim == true {
            self.indice_final = Some(indice);
            return;
        }

        if depth > 0 {
            for adj in no.adjacencias.iter() {
                match self.vertices.get_mut(adj) {
                    None => {
                        self.dfs_depth(api, *adj, indice, depth - 1, false);
                    }
    
                    Some(no_adj) => {
                        if no_adj.explorado == false {
                            self.dfs_depth(api, *adj, indice, depth - 1, false);
                        }
                    }
                }
            }
        }

        // Redundante?
        // let no = self.vertices.get(&indice).unwrap().clone();

        if (no.inicio == false) && (self.indice_final.is_none()) {
            //println!("V {}", no.anterior);
            api.movimentar(&mut self.vertices, no.anterior, indice);
        }

        //Self::dfs_status(api, indice);        
    }

    // DFS com aprofundamento iterativo.
    pub fn iddfs(&mut self, api: &mut API, indice_inicial: i32) -> i32 {
        let mut i: i32 = 3;

        loop {
            self.dfs_depth(api, indice_inicial, -1, i, true);

            if self.indice_final.is_some() {
                return self.indice_final.unwrap();
            } else {
                self.deletar_nos(indice_inicial);
                i += 1;
            }
        }
    }

}

