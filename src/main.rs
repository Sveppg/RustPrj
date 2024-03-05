fn main() {
    //println!("Hello, world!");
    
    let pattern = std::env::args().nth(1).expect("no pattern given!");
    let path = std::env::args().nth(2).expect("no path given");
    let col = std::env::args().nth(3).expect("no column given");

    let args = Cli {
        pattern: pattern,
        path: std::path::Pathbuf::from(path),
    };
    
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}

//Note: Pathbuf is like a String but for file system paths that work cross-platform


