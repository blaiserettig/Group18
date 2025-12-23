extern crate Group18;

use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufWriter};
use std::path::{Path, PathBuf};
use std::time::Instant;

use Group18::generate::Generator;
use Group18::parse::Parser;
use Group18::parse::ParseTreeNode;
use Group18::tokenize::{Token, Tokenizer};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: Noble <filename> [--tokens] [--cst] [--ast]");
        return;
    }

    let filename = &args[1];
    let show_tokens = args.contains(&"--tokens".to_string());
    let show_cst = args.contains(&"--cst".to_string());
    let show_ast = args.contains(&"--ast".to_string());

    let input_file_path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(filename);

    let start_time = Instant::now();

    println!("Reading file from: {:?}", input_file_path);
    let file_contents: String = read_file(input_file_path);

    let mut tokenizer = Tokenizer::new(file_contents.clone());
    let tokens: Vec<Token> = tokenizer.tokenize();

    if show_tokens {
        println!("--- TOKENS ---");
        for token in &tokens {
            println!("{:?}", token);
        }
    }

    let mut parser = Parser::new(tokens, file_contents, args[1].clone());
    let tree: ParseTreeNode = parser.parse();

    if show_cst {
        println!("--- CONTEXT FREE SYNTAX TREE ---");
        parser.print_tree(&tree, 0);
        println!();
    }

    let ast = parser.build_ast(&tree);

    if show_ast {
        println!("--- ABSTRACT SYNTAX TREE ---");
        parser.print_ast(&ast, 0);
    }

    let output_file_path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/out.asm");

    let output_file = File::create(output_file_path).expect("Unable to create file.");
    let mut writer = BufWriter::new(&output_file);

    let mut generator = Generator::new();
    generator.generate_boilerplate(&mut writer);
    generator.generate_x64(&ast, &mut writer);

    let duration = start_time.elapsed();
    println!("Compilation took: {:?}", duration);
}

fn read_file(file_path: PathBuf) -> String {
    let contents: String =
        fs::read_to_string(file_path).expect("Unable to read file.");
    contents
}