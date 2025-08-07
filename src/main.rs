use std::io;
use rand::Rng;

fn main()
{
   println!("Guess the number");

   let secret_number = rand::thread_rng()
                                  .gen_range(1..=100);
    
   println!("Secret number is {secret_number}");  
    
   loop
   {
      println!("please input your guess");

      let mut guess = String::new();

      io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read lines");


      let guess: u32 = guess
                    .trim()
                    .parse()
                    .expect("Please try a number");   


      match guess.cmp(&secret_number) 
     {
      std::cmp::Ordering::Less => println!("Too Small !"),
      std::cmp::Ordering::Greater => println!("Too Big!"),
      std::cmp::Ordering::Equal => 
          {
            println!("You win!");
            break;
          }
     }

      println!("your guess {guess} ");

    
   } 
}