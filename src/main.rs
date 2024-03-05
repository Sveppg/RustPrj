fn main() {
    //println!("Hello, world!");
    
    let pattern = std::env::args().nth(1).expect("no pattern given!");
    let path = std::env::args().nth(2).expect("no path given");
    let col = std::env::args().nth(3).expect("no column given");
    struct Cli{
        pattern: String;
        path: std::path::Pathbuf,
    };
    
    println!("pattern: {:?}, path: {:?}, column {:?}", args.pattern, args.path, args.col );
}

//Note: Pathbuf is like a String but for file system paths that work cross-platform


