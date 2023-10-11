

fn help() {
    println!("\nUse: cralph [options] [input]\n");
    println!("Available [options] are:");
    println!("\t\t\t--file");
    println!("\t\t\t\tTo open a file provided as filepath via [input].");
    println!("\t\t\t--text");
    println!("\t\t\t\tTo open text provided via [input]");
    println!("\t\t\t--help");
    println!("\t\t\t\tTo open this\n");
    std::process::exit(0);
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let argc: usize = argv.iter().count();

    let _character_array: Vec<u8> = vec![1, 2, 3];

    dbg!(argc);
    if argv[1] == "--help" { help(); }
}
