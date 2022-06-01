use std::io;

fn main(){
   println!("Guess the number!");
   println!("Please input your guess.");
   let mut guess = String::new();
   io::stdin
       .read_Line(buf: &mut guess)
       .expect(msg:"Failed to read line");
   println!("You guessed: {}", guess
}
