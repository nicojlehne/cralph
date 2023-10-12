const ERR_GENERIC: isize = 1;
const ERR_NOFILE: isize = 2;
const ERR_NOLOGFILE: isize = 3;
const ERR_NOTEXT: isize = 4;
const ERR_NOARG: isize = 5;
const ERR_BADARG: isize = 6;
const ERR_NOWAY: isize = 255;         // This error is not supposed to be reached and is a placeholder :3
                                      // It's a placeholder for ERR_GENERIC, really. I use it when a possibility for an error is based on cosmic chance and not by user input.

static CHARACTER_ARRAY: &'static [char] = &[' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0',
                                            '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@',
                                            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                                            'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '[', '\\', ']', '^', '_', '`',
                                            '{', '|', '}', '~'];

fn help() -> () {
    println!("\nUse: cralph [options] [input] [optional: --log]\n");
    println!("Available [options] are:");
    println!("\t\t\t--file");
    println!("\t\t\t\tTo open a file provided as filepath via [input].");
    println!("\t\t\t--text");
    println!("\t\t\t\tTo open text provided via [input].");
    println!("\t\t\t--help");
    println!("\t\t\t\tTo open this.\n");
    std::process::exit(0);
}

fn error_handler(exit_code: isize) -> () {
    match exit_code {
        ERR_NOFILE => println!("File does not exist or is otherwise unavailable\nTry supplying a (valid) filename"),
        ERR_NOLOGFILE => println!("No (valid) location/name for output logfile provided"),
        ERR_NOTEXT => println!("No text was provided. Provide a text, preferably 'in quotes', with --text"),
        ERR_NOARG => println!("Not enough arguments given, use cralph --help or cralph -h to see syntax"),
        ERR_BADARG => println!("Bad argument given. Use cralph --help or cralph -h to see syntax"),
        ERR_NOWAY => println!("This error was not supposed to be reached."),
        ERR_GENERIC | _ => println!("Generic or unimplemented error"),
    }
    std::process::exit(i32::try_from(exit_code).ok().unwrap());
}

fn argument_handler(argc: usize, argv: Vec<String>) -> bool {
    if !(argc > 1) {error_handler(ERR_NOARG)};
    if argc > 3 {
        if argv[3] == "--log" || argv[3] == "-l" {
            if let Ok(input_file) = std::fs::File::open(&argv[4]) { return true; } else {error_handler(ERR_NOLOGFILE)}
            let mut logfile = std::fs::OpenOptions::new().create(true).write(true).append(true).open(&argv[4]).unwrap();
        }
    }
    return false;
}

fn file_handler(argv: Vec<String>, where_is_filename: usize) -> isize {
    if let Ok(input_file) = std::fs::File::open(&argv[where_is_filename]) {
        drop(input_file);
        return 0;
    }
    let mut input_file = std::fs::OpenOptions::new().create(true).write(true).append(true).open(&argv[where_is_filename]).unwrap();
    dbg!(input_file);
    error_handler(ERR_NOFILE);
    return ERR_NOFILE;                                      // This line is so incredibly useless (due to error_handler outright combusting) but cargo wouldn't compile without it
}

fn count_characters (mode: String, count: &mut Vec<usize>, argv: Vec<String>, count_array_size: isize, logfile_provided: bool) -> () {
    match mode.as_str() {
        "file" => if file_handler(argv, 2) == 0 {
            for _i in 0..count_array_size {
                ();
            }
        },
        "text" => {
            if argv.get(2) == None { error_handler(ERR_NOTEXT) };
            let mut x: usize = 0;                                   // This little shit line is needed cause i is not a usize, thank me never ùwú
            for _i in 0..count_array_size {
                for k in 0..argv[2].chars().count() {
                    let character = argv[2].chars().nth(k).unwrap();
                    if character == CHARACTER_ARRAY[x] || character == CHARACTER_ARRAY[x].to_ascii_uppercase() { count[x] += 1; }
                };
                x += 1;
            }
        },
        _ => error_handler(ERR_NOWAY),
    }
    sum_it_up (count, count_array_size, logfile_provided);
}

fn sum_it_up (count: &mut Vec<usize>, count_array_size: isize, logfile_provided: bool) -> () {
    let mut sum: usize = 0;
    let mut x: usize = 0;
    for _i in 0..count_array_size {
        sum += count[x];
        if !(count[x] == 0) {
            println!("{}: {}", CHARACTER_ARRAY[x], count[x]);
        }
        x += 1;
    }
    println!("Sum: {} characters\nSum (without spaces): {}", sum, sum-count[0]);
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let argc: usize = argv.iter().count();
    let first_argument: &String = &argv[1];
    let logfile_provided: bool = argument_handler(argc, argv.clone());

    let count_array_size = CHARACTER_ARRAY.iter().count();
    let mut count: Vec<usize> = vec![0; count_array_size];

    match first_argument.as_str() {
        "--help" | "-h" => help(),
        "--file" | "-f" => count_characters("file".to_string(), &mut count, argv, count_array_size.try_into().unwrap(), logfile_provided),    // Random bullshit, Go!
        "--text" | "-t" => count_characters("text".to_string(), &mut count, argv, count_array_size.try_into().unwrap(), logfile_provided),    // I don't know why this works, thanks compiler! :3
        _ => error_handler(ERR_BADARG),
    }
}
