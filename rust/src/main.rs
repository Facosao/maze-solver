pub mod api;
pub mod vertice;
pub mod solver;
pub mod graph;

/*
struct No {
    adj: Vec<usize>,
    explorado: bool,
    anterior: usize,
}

impl No {
    fn novo() -> Self {
        No {
            adj: Vec::new(),
            anterior: 0, 
            explorado: false,
        }
    }
}

fn bfs(nos: &mut Vec<No>, raiz: usize, alvo: usize) -> usize {
    let mut fila: Vec<usize> = Vec::new();
    nos[raiz].explorado = true;
    fila.push(raiz);
    
    while fila.len() > 0 {
        let atual = fila.remove(0);
        if atual == alvo {
            return atual;
        }

        let vec_adj: Vec<usize> = nos[atual].adj.clone();        
        for aux in vec_adj.iter() {
            let no_adj = &mut nos[*aux];
            if no_adj.explorado == false {
                no_adj.explorado =  true;
                no_adj.anterior = atual;
                fila.push(*aux);
            }
        }
    }

    return 0;
}
*/

fn main() {
    // read argsv from command line argument
    solver::solver(None, None);
}