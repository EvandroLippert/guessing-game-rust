use std::io;
use rand::seq::SliceRandom;
use colored::Colorize;

fn main() {
    let vs = vec!["agua", "soda", "foca", "gelo"];
    let choice = vs.choose(&mut rand::thread_rng());
    let word = if let Some(x) = choice {
        x.to_string()
    } else {
        String::from("Teste")
    };
    
    let length = word.len();
    println!("Você tem 3 chances de acertar a palavra");
    for i in 0..3 {
        println!("{}ª tentativa", i+1);
        println!("Escreva uma palavra com 4 letras:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to readline");
        let mut answer = vec![];
        let mut counter = 0; 
        for n in 0..length {
            if guess.chars().nth(n).unwrap() == word.chars().nth(n).unwrap(){
                answer.push(String::from(guess.chars().nth(n).unwrap()).bright_green().bold());
                counter +=1;
            }
            else if word.contains(guess.chars().nth(n).unwrap()){
                answer.push(String::from(guess.chars().nth(n).unwrap()).bright_yellow().bold());
            }
            else
            {
                answer.push(String::from('_').red());
            }
            if counter == length{
                println!("Parabéns, você acertou, a palavra era {}", word);
                return 
            }
        }
        for character in answer.iter() {
            print!("{}", character);
        }
        println!("");
    }
    println!("Perdeu");
}
