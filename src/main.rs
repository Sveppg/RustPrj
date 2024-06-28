use std::io;

fn main(){
	println!("Guess a number...");
	println!("Type in a number: ");
	let mut guess = String::new();

	let x = 5;
	let y = 10;
	println!("x = {x} and y +2 ={}", y+2);
	io::stdin()
	.read_line(&mut guess)
	.expect("Failed to read line");
	println!("You guessed: {guess}");
}
