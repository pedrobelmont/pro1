use std::io;

use crate::utils::terminal::limpar_terminal;
use  crate::utils::operacoes::{soma, sub};
mod utils;

fn main() {
    let mut index_historico:i32 = 0;
    let mut ultimo_calc:i32;
    let mut historico:[String; 6]= Default::default();
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    
    let a:i32 = 8;
    let b:i32 = 5;
    limpar_terminal();
    println!("Você digitou: {}", input);
    let calc:String = String::from("calculadora veloz em rust");
    println!("{}",calc);
    ultimo_calc = soma(a, b,&mut historico,&mut index_historico);
    
    println!("valor atual {}", ultimo_calc);
    
    ultimo_calc = sub(ultimo_calc,5, &mut historico, &mut index_historico);
    
    println!("valor atual {}", ultimo_calc);
    println!("historico calculos {:?}", historico);

}

