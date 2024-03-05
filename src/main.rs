use std::io;
fn main() {
    //println!("Hello, world!");
    
    let pattern = std::env::args().nth(1).expect("no pattern given!");
    let path = std::env::args().nth(2).expect("no path given");
    let col = std::env::args().nth(3).expect("no column given");
    struct Cli{
        pattern: String;
        path: std::path::Pathbuf,
    };
    
    guess();
    
    println!("pattern: {:?}, path: {:?}, column {:?}", args.pattern, args.path, args.col );
}

fn guess(){
    println!("Guess a number: ");
    let mut guess = String::new();
    let y = 5; 
    let x = 10;
    println!("x = {x}, y + 2 = {}", y+2);
    io:stdn()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!(" Your Guess: {guess}");
}
//Note: Pathbuf is like a String but for file system paths that work cross-platform


