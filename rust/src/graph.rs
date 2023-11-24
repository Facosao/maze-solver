use std::collections::{HashMap, HashSet};

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

    // Sem chamadas de API
    pub fn bfs_ram(&mut self, raiz: i32) -> i32 {
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
                let no_adj: &mut Vertice;

                match self.vertices.get_mut(&adj) {
                    Some(vert) => no_adj = vert,
                    None => continue,
                }

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
        self.vertices.get_mut(&indice).unwrap().depth = depth;
        self.vertices.get_mut(&indice).unwrap().anterior = anterior;

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
                        if (no_adj.explorado == false) || (no_adj.depth <= depth) {
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

        Self::dfs_status(api, indice);        
    }

    // DFS com aprofundamento iterativo.
    pub fn iddfs(&mut self, api: &mut API, indice_inicial: i32) -> i32 {
        let mut i: i32 = 0;

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

    // Explora o grafo com BFS
    // TODO: Rewrite this to use two states
    pub fn bfs_explorer(&mut self, api: &mut API, indice_inicial: i32) -> i32 {
        let mut fila: Vec<i32> = Vec::new();
        let mut start = true;
        let mut current_node: i32 = indice_inicial;
        let mut anterior: i32;

        self.vertices.get_mut(&indice_inicial).unwrap().explorado = true;
        fila.push(indice_inicial);

        while !fila.is_empty() {
            anterior = current_node;
            let v = fila.remove(0);

            if (current_node == v) || (start == true) {    
                continue;
            } else {
                if !self.vertices[&current_node].adjacencias.contains(&v) {
                    let mut path_to_node = self.bfs_restricted(current_node, v);
                    path_to_node.extend_from_slice(&fila);
                    fila = path_to_node;
                    continue;
                } else {
                    api.movimentar(&mut self.vertices, v, anterior);
                    current_node = v;
                }
            }

            let vert = self.vertices[&v].clone();

            if vert.fim == true {
                return v;
            }

            for vertice in vert.adjacencias {
                if !self.vertices.contains_key(&vertice) {
                    // Insert incomplete vertex
                    self.vertices.insert(vertice, Vertice::novo(vertice, vert.id));
                }

                let vert_adj = self.vertices
                    .get_mut(&vertice)
                    .unwrap();

                if vert_adj.explorado == false {
                    vert_adj.explorado = true;
                    vert_adj.anterior = vert.id;
                    //previous.insert(vertice, vert.id);
                    fila.push(vertice);
                }
            }

            start = false;
        }
        
        return -1;
    }

    // Used with a broken graph to find a path
    fn bfs_restricted(&mut self, start: i32, end: i32) -> Vec<i32> {
        //println!("HashMap: {:?}", self.vertices.keys());
        //println!("S: {} E: {}", start, end);
        let mut previous: HashMap<i32,i32> = HashMap::new();
        let mut visited: HashSet<i32> = HashSet::new();
        let mut queue: Vec<i32> = Vec::new();

        visited.insert(start);
        previous.insert(start, -1);
        queue.push(start);

        while !queue.is_empty() {
            let v = queue.remove(0);
            //print!("v = {}", v);

            // Possibly skip through unavailable vertices?

            let vert: Vertice;

            match self.vertices.get(&v) {
                Some(vertex) => vert = vertex.clone(),
                None => continue,
            }

            //println!(", adj = {:?}", vert.adjacencias);

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
                        //println!("aux = {}", aux);
                        path.push(aux);
                        if previous[&aux] != -1 {
                            aux = previous[&aux];      
                        } else {
                            break;
                        }
                    }

                    path.reverse();
                    //println!("S: {} E: {} L: {:?}", start, end, path);
                    return path;
                }
            }
        }

        return queue;
    }

}

