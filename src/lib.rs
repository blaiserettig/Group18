use wasm_bindgen::prelude::*;
use std::io::{BufWriter};

pub mod tokenize;
pub mod parse;
pub mod generate;

use crate::tokenize::Tokenizer;
use crate::parse::Parser;
use crate::generate::Generator;

#[wasm_bindgen]
pub fn compile_g18(input: &str) -> String {
    let mut tokenizer = Tokenizer::new(input.to_string());
    let tokens = tokenizer.tokenize();
    
    let mut parser = Parser::new(tokens, input.to_string(), "web_demo.g18".to_string());
    let tree = parser.parse();
    let ast = parser.build_ast(&tree);
    
    let mut out_buffer = Vec::new();
    {
        let mut writer = BufWriter::new(&mut out_buffer);
        let mut generator = Generator::new();
        generator.generate_boilerplate(&mut writer);
        generator.generate_x64(&ast, &mut writer);
        // Explicitly flush to ensure all data is in out_buffer
        std::io::Write::flush(&mut writer).unwrap();
    }
    
    String::from_utf8(out_buffer).unwrap_or_else(|_| "Error: Invalid UTF-8 generated".to_string())
}
