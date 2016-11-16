extern crate rand;      //importar a biblioteca rand

use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Jantar {
    pronto: Mutex<bool>,             //mutex bollean que indica se o jantar esta finalizado
    garfos: Vec<Mutex<()>>,          //vetor de mutex para armazenar os garfos
}

impl Jantar {                       //em rust structs definem variaveis e impl definem funcoes para a struct, nao existe classe
    pub fn alterar_pronto(&self){         //implementa funcao publica que altera o estado do jantar
        let mut pronto = self.pronto.lock().unwrap();   //acessa o estado da variavel protegida por lock
        *pronto = !*pronto;         //altera a variavel
    }
}

impl Drop for Jantar {              //implementacao do desconstrutor da classe
    fn drop(&mut self){
        println!("Acabou o Jantar!");
    }
}

struct Filosofo {       //definicao de filosofo
    nome: String,       
    jantar: Arc<Jantar>,//atomically reference counted: referencia atomica para utilizar memoria compartilhada, basicamente torna acessar jantar thread-safe
    esquerda: usize,    //talher a esquerda
    direita: usize,     //talher a direita
}

impl Filosofo {
    pub fn novo(nome: &str, jantar: Arc<Jantar>, esquerda: usize, direita: usize) -> Filosofo {  //contrutor de filosofo
        Filosofo {
            nome: nome.to_owned(),
            jantar: jantar,
            esquerda: esquerda,
            direita: direita,
        }
    }

    pub fn acordar(&self) {
        while !*self.jantar.pronto.lock().unwrap() {    //tentar até o jantar estar pronto
        }
        println!("{} acordou", self.nome); 
        while *self.jantar.pronto.lock().unwrap() {     //se estiver pronto, tentar comer
            self.jantar();
            if !*self.jantar.pronto.lock().unwrap() {   //se não estiver pronto depois de comer break
                break;
            }
            self.filosofar();                           //filosofar após comer
        }
    }

    pub fn jantar(&self) {
        let _esquerda = self.jantar.garfos[self.esquerda].lock().unwrap();  //tenta pegar o garfo da esquerda
        let _direita = self.jantar.garfos[self.direita].lock().unwrap();    //tenta pegar o garfo da direita
        if !*self.jantar.pronto.lock().unwrap() {                           //se o jantar nao estiver pronto return
            return;
        }

        println!("{} começou a jantar", self.nome);                           //println do nome do filosofo
        let mut rng = rand::thread_rng();                                    //pega um numero randomico
        thread::sleep(Duration::from_millis(rng.gen_range(0, 1000)));      //dorme a thread por um tempo randomico
        println!("{} terminou de jantar", self.nome);
    }

    pub fn filosofar(&self) {
        println!("{} começou a filosofar", self.nome);
        let mut rng = rand::thread_rng();                                    //pega um numero randomico
        thread::sleep(Duration::from_millis(rng.gen_range(0, 1000)));     //dorme a thread por um tempo randomico
        println!("{} esta com fome novamente", self.nome);
    }
}

fn main() {
    let jantar = Arc::new(Jantar {  //arc para permitir apenas um jantar e que ele seja acessivel pelos filosofos
        pronto: Mutex::new(false),  //inicia pronto como falso atravez de um mutex
        garfos: vec![Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(())],   //inicializa os garfos
    });
    let filosofos = vec![        //inicia os filosofos
        Filosofo::novo("Sócrates", jantar.clone(), 0, 1),
        Filosofo::novo("Aristóteles", jantar.clone(), 1, 2),
        Filosofo::novo("Platão", jantar.clone(), 2, 3),
        Filosofo::novo("Descartes", jantar.clone(), 3, 4),
        Filosofo::novo("Nietzsche", jantar.clone(), 4, 0),
    ];

    let gerenciador: Vec<_> = filosofos.into_iter() //inicia uma interação no vetor dos filosofos
        .map(|p| {  //map para poder acessar os resultados na variavel p
            thread::spawn(move || { //inicia uma thread que acorda o filosofo
                p.acordar();
            })
        }).collect(); //collect para armazenar os resultados no gerenciador
    thread::sleep(Duration::new(1, 0));
    println!("O JANTAR COMEÇOU");
    jantar.alterar_pronto();

    for g in gerenciador {  //finaliza as threads
        g.join().unwrap();
    }
}