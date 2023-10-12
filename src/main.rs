// I just don't like having to use multiple lines for if statements that could be done without multiple lines/curly braces uwu
macro_rules! ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {$v} else {$v1}
    };
}

const ERR_GENERIC: i32 = 1;
const ERR_NOFILE: i32 = 2;
const ERR_NOARG: i32 = 3;
const ERR_NOLOGFILE: i32 = 4;

static CHARACTER_ARRAY: &'static [char] = &[' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0',
                                            '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@',
                                            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                                            'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '[', '\\', ']', '^', '_', '`',
                                            '{', '|', '}', '~'];

static _SUM: u32 = 0;

static mut LOGFILEPROVIDED: bool = false;

fn help() -> () {
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

fn error_handler(exit_code: i32) -> () {
    ternary! (exit_code == ERR_NOFILE, println!("File does not exist or is otherwise unavailable\nTry supplying a (valid) filename"),
    ternary! (exit_code == ERR_NOLOGFILE, println!("No (valid) location/name for output logfile provided"),
    ternary! (exit_code == ERR_NOARG, println!("Not enough arguments given, use cralph --help or cralph -h to see syntax"), ())));
    std::process::exit(exit_code);
}

fn argument_handler(argc: usize, argv: Vec<String>) -> () {
    if !(argc > 1) {error_handler(ERR_NOARG)};
    if argc > 3 {
        if argv[3] == "--log" || argv[3] == "-l" {
            if let Ok(input_file) = std::fs::File::open(&argv[4]) { unsafe {LOGFILEPROVIDED = true}; } else {error_handler(ERR_NOLOGFILE)}
        }
    }
}

fn file_handler(argv: Vec<String>) -> i32 {
    if let Ok(input_file) = std::fs::File::open(&argv[4]) {
        return 0;
    }
    error_handler(ERR_NOFILE);
    return ERR_NOFILE;                                      // This line is so incredibly useless (due to error_handler outright combusting) but cargo wouldn't compile without it
}

fn count_characters (mode: String, argv: Vec<String>) -> () {

}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let argc: usize = argv.iter().count();

    let count_array_size = CHARACTER_ARRAY.iter().count();
    let /*mut*/ count: Vec<u32> = vec![0; count_array_size];

    argument_handler(argc, argv.clone());

    ternary!(argv[1] == "--help", help(), ());
    for i in 0..count_array_size {
        ternary!(!count[i] == 0, println!("Character '{}': {}", CHARACTER_ARRAY[i], count[i]), ());
    }

}
