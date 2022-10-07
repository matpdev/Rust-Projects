use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
   println!("Adivinha o número");

   let secret_number = rand::thread_rng().gen_range(1..=100);

  println!("Por favor diga um número");
   let mut guess_number = String::new();
   io::stdin()
       .read_line(&mut guess_number)
       .expect("Deu um erro aqui mano");
       
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You Win!"),
    }

  println!("Número secreto é: {secret_number}");
   println!("Sua escolha: {guess_number}");
}

// Let Mut == Mutable ou Mutável
// Let {var} == Imutable ou imutável

