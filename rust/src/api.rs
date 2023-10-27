use core::panic;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::{self, blocking::Client, blocking::Response};
use crate::vertice::Vertice;

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
    pub _n_calls: i32,
    api: String,
    pub maze: String,
}

impl API {
    pub fn novo(api: Option<&str>, maze: Option<&str>) -> Self {
        let address: &str;
        let maze_id: &str;

        match api {
            Some(url) => address = url,
            None => address = "https://gtm.delary.dev",
        }

        match maze {
            Some(id) => maze_id = id,
            None => maze_id = "medium-maze",
        }

        let novo_client = reqwest::blocking::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        API{
            client: novo_client,
            _n_calls: 0,
            api: address.to_string(),
            maze: maze_id.to_string(),
        }
    }

    fn gravar_no(&self, vertices: &mut HashMap<i32, Vertice>, resp: Response, anterior: i32) -> Option<i32> {
        if resp.status().as_u16() != 200 {
            println!("Erro: {:?}", resp.status().as_u16());
            println!("{:?}", resp.text());
            panic!("Erro durante a gravacao do no!");
        } else {
            let pos: RMovimentar = resp.json().unwrap();

            if vertices.contains_key(&pos.pos_atual) {
                return None;
            } else {
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

    pub fn iniciar(&self, vertices: &mut HashMap<i32, Vertice>) -> Option<i32> {
        let dados = json!({
            "id": ID,
            "labirinto": self.maze
        });

        let response = self.client
            .post(format!("{}/iniciar", self.api))
            .json(&dados)
            .send()
            .unwrap();

        return self.gravar_no(vertices, response, -1);
    }

    pub fn movimentar(&self, vertices: &mut HashMap<i32, Vertice>, indice: i32, anterior: i32) -> Option<i32> {
        //let dados = json!({
        //    "id": ID,
        //    "labirinto:": self.maze,
        //    "nova_posicao": indice
        //});

        let response = self.client
            .post(format!("{}/movimentar", self.api))
            .json(&json!({
                "id": ID,
                "labirinto:": self.maze,
                "nova_posicao": indice
            }))
            .send()
            .unwrap();

        return self.gravar_no(vertices, response, anterior);
    }

    pub fn validar_caminho(&self, caminho: Vec<i32>) {
        let dados = json!({
            "id": ID,
            "labirinto": self.maze,
            "todos_movimentos": caminho
        });

        let response = self.client
            .post(format!("{}/validar_caminho", self.api))
            .json(&dados)
            .send()
            .unwrap();

        if response.status().as_u16() != 200 {
            println!("Erro: {:?}", response.status().as_u16());
            println!("{:?}", response.text());
            panic!("Erro durante a validacao do caminho!");
        } else {
            let validacao: RValidarCaminho = response.json().unwrap();
            println!("--- Qtd. movimentos: {}", validacao.quantidade_movimentos);
            println!("--- Caminho valido : {}", validacao.caminho_valido);
        }
    }
}

pub fn _rest_call() {
    //let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    //let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    //println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    //let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    //println!("deserialized = {:?}", deserialized);

    //let body = reqwest::blocking::get("https://gtm.delary.dev/labirintos").unwrap();
    //println!("body = {:?}", body);

    let client = reqwest::blocking::Client::new();
    let body = client
        .get("https://gtm.delary.dev/labirintos")
        .send()
        .unwrap();

    match body.text() {
        Ok(text) => println!("body = {:?}", text),
        Err(_) => unimplemented!(),
    }

    let map = json!({
        "id": "grupo_i",
        "labirinto": "sample-maze"
    });
    let res = client
        .post("https://gtm.delary.dev/iniciar")
        .json(&map)
        .send()
        .unwrap();

    //println!("pos = {:?}", res.text());

    let pos: RMovimentar = res.json().unwrap();
    println!("pos = {:?}", pos);
}
