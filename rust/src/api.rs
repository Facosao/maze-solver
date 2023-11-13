use core::panic;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::{self, blocking::Client, blocking::Response};

use crate::vertice::Vertice;
use crate::timer::Timer;

const ID: &'static str = "grupo_i";

#[derive(Serialize, Deserialize, Debug)]
struct RMovimentar {
    pos_atual: i32,
    inicio: bool,
    r#final: bool,
    movimentos: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RValidarCaminho {
    caminho_valido: bool,
    quantidade_movimentos: i32,
}

pub struct API {
    client: Client,
    pub n_calls: i32,
    pub url: String,
    pub maze: String,
    pub timer: Timer
}

impl API {
    pub fn novo(api: Option<String>, maze: Option<String>) -> Self {
        let address: String;
        let maze_id: String;

        match api {
            Some(url) => address = url,
            None => address = "https://gtm.delary.dev".to_string(),
        }

        match maze {
            Some(id) => maze_id = id,
            None => maze_id = "maze-sample".to_string(),
        }

        let novo_client = reqwest::blocking::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        API{
            client: novo_client,
            n_calls: 0,
            url: address.to_string(),
            maze: maze_id.to_string(),
            timer: Timer::novo()
        }
    }

    fn gravar_no(&self, vertices: &mut HashMap<i32, Vertice>, resp: Response, anterior: i32) -> Option<i32> {
        if resp.status().as_u16() != 200 {
            println!("\nErro: {:?}", resp.status().as_u16());
            println!("{:?}", resp.text());
            panic!("Erro durante a gravacao do no!");
        } else {
            let pos: RMovimentar = resp.json().unwrap();

            match vertices.get_mut(&pos.pos_atual) {
                Some(vertice) => {
                    if vertice.fetch == false {
                        vertice.fetch = true;
                        vertice.inicio = pos.inicio;
                        vertice.fim = pos.r#final;

                        for item in pos.movimentos.iter() {
                            vertice.adjacencias.push(*item);
                        }
                    }

                    return None;
                }
                None => {
                    let mut novo_vertice = Vertice::novo(pos.pos_atual, anterior);
                    novo_vertice.inicio = pos.inicio;
                    novo_vertice.fim = pos.r#final;

                    for item in pos.movimentos.iter() {
                        novo_vertice.adjacencias.push(*item);
                    }

                    vertices.insert(novo_vertice.id, novo_vertice);

                    return Some(pos.pos_atual);
                }
            }
        }
    }

    pub fn iniciar(&mut self, vertices: &mut HashMap<i32, Vertice>) -> Option<i32> {
        let dados = json!({
            "id": ID,
            "labirinto": self.maze
        });

        self.timer.iniciar();

        let response = self.client
            .post(format!("{}/iniciar", self.url))
            .json(&dados)
            .send()
            .unwrap();

        self.timer.parar();
        self.n_calls += 1;

        return self.gravar_no(vertices, response, -1);
    }

    pub fn movimentar(&mut self, vertices: &mut HashMap<i32, Vertice>, indice: i32, anterior: i32) {
        let dados = json!({
            "id": ID,
            "labirinto": self.maze,
            "nova_posicao": indice
        });

        //println!("{}", serde_json::to_string_pretty(&dados).unwrap());
        self.timer.iniciar();

        let response = self.client
            .post(format!("{}/movimentar", self.url))
            .json(&dados)
            .send()
            .unwrap();

        self.timer.parar();
        self.n_calls += 1;

        self.gravar_no(vertices, response, anterior);
    }

    pub fn validar_caminho(&mut self, caminho: Vec<i32>) {
        let dados = json!({
            "id": ID,
            "labirinto": self.maze,
            "todos_movimentos": caminho
        });

        self.timer.iniciar();

        let response = self.client
            .post(format!("{}/validar_caminho", self.url))
            .json(&dados)
            .send()
            .unwrap();

        self.timer.parar();
        self.n_calls += 1;

        if response.status().as_u16() != 200 {
            println!("\nErro: {:?}", response.status().as_u16());
            println!("{:?}", response.text());
            panic!("Erro durante a validacao do caminho!");
        } else {
            let validacao: RValidarCaminho = response.json().unwrap();
            println!("--- Qtd. movimentos: {}", validacao.quantidade_movimentos);
            println!("--- Caminho valido : {}", validacao.caminho_valido);
        }
    }
}
