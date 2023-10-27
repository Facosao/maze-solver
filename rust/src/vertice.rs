pub struct Vertice {
    pub id: i32,
    pub anterior: i32,
    pub explorado: bool,
    pub adjacencias: Vec<i32>,
    pub inicio: bool,
    pub fim: bool,
}

impl Vertice {
    pub fn novo(indice: i32, anterior: i32) -> Self {
        Vertice {
            id: indice,
            anterior: anterior,
            explorado: false,
            adjacencias: Vec::new(),
            inicio: false,
            fim: false
        }
    }
}