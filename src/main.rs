mod tokenize;
mod parse;
mod generate;

use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufWriter};
use std::path::{Path, PathBuf};
use crate::generate::Generator;
use crate::parse::Parser;
use crate::parse::ParseTreeNode;
use crate::tokenize::{Token, Tokenizer};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: ./d [filename]");
        return;
    }

    let input_file_path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join(&args[1]);

    let file_contents: String = read_file(input_file_path);

    println!("{:?}", file_contents);

    let mut tokenizer = Tokenizer::new(file_contents.clone());
    let tokens: Vec<Token> = tokenizer.tokenize();
    
    for token in &tokens {
        println!("{:?}", token);
    }
    
    let mut parser = Parser::new(tokens, file_contents, args[1].clone());
    let tree: ParseTreeNode = parser.parse();

    parser.print_tree(&tree, 0);
    println!();

    let ast = parser.build_ast(&tree);
    parser.print_ast(&ast, 0);

    let output_file_path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/out.asm");

    let output_file = File::create(output_file_path).expect("Unable to create file.");
    let mut writer = BufWriter::new(&output_file);

    let mut generator = Generator::new();
    generator.generate_boilerplate(&mut writer);
    generator.generate_x64(&ast, &mut writer);
}

fn read_file(file_path: PathBuf) -> String {
    let contents: String =
        fs::read_to_string(file_path).expect("Unable to read file.");
    contents
}