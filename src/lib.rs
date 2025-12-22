use wasm_bindgen::prelude::*;
use std::io::{BufWriter};

pub mod tokenize;
pub mod parse;
pub mod generate;
pub mod generate_wasm;

use crate::tokenize::Tokenizer;
use crate::parse::Parser;
use crate::generate::Generator;
use crate::generate_wasm::WasmGenerator;

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

#[wasm_bindgen]
pub fn compile_wat_g18(input: &str) -> String {
    let mut tokenizer = Tokenizer::new(input.to_string());
    let tokens = tokenizer.tokenize();
    
    let mut parser = Parser::new(tokens, input.to_string(), "web_demo.g18".to_string());
    let tree = parser.parse();
    let ast = parser.build_ast(&tree);
    
    let mut out_buffer = Vec::new();
    {
        let mut generator = WasmGenerator::new();
        generator.generate_wat(&ast, &mut out_buffer);
    }
    
    String::from_utf8(out_buffer).unwrap_or_else(|_| "Error: Invalid UTF-8 generated".to_string())
}

#[wasm_bindgen]
pub fn compile_wasm_g18(input: &str) -> Result<Vec<u8>, String> {
    let wat = compile_wat_g18(input);
    if wat.starts_with("Error") {
        return Err(wat);
    }
    wat::parse_str(&wat).map_err(|e| e.to_string())
}
