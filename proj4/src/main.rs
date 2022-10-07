use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
   loop {
      let secret_number = rand::thread_rng().gen_range(1..=100);
      
      let mut guess = String::new();
      
      println!("Please type a number");
      
      io::stdin().read_line(&mut guess).expect("Failed to read the line");
      
      let guess: u32 = match guess.trim().parse() {
         Ok(num) => num,
         Err(_) => continue,
      };
      
      match guess.cmp(&secret_number){
         Ordering::Less => println!("Too Small"),
         Ordering::Greater => println!("Too big"),
         Ordering::Equal => {
            println!("You win bro");
            break;
         },
      }
      println!("The number generated: {secret_number}");
      println!("Your guess: {guess}");
   }
}