use crate::parse::{AbstractSyntaxTreeNode, AbstractSyntaxTreeSymbol, BinOpType, Expr, Type};
use std::collections::{HashMap, HashSet};
use std::io::Write;

pub struct WasmGenerator {
    scopes: Vec<HashMap<String, WasmVarEntry>>,
    functions: HashMap<String, Type>, // return types
    string_literals: HashMap<String, String>, // content, label
    string_counter: usize,
}

struct WasmVarEntry {
    _index: u32,
    type_: Type,
}

impl WasmGenerator {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            string_literals: HashMap::new(),
            string_counter: 0,
        }
    }

    pub fn generate_wat<W: Write>(&mut self, ast_root: &AbstractSyntaxTreeNode, writer: &mut W) {
        writeln!(writer, "(module").unwrap();
        
        // Imports
        writeln!(writer, "  (import \"env\" \"print_int\" (func $print_int (param i32)))").unwrap();
        writeln!(writer, "  (import \"env\" \"print_str\" (func $print_str (param i32)))").unwrap();
        writeln!(writer, "  (import \"env\" \"memory\" (memory 1))").unwrap();

        if let AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolEntry = &ast_root.symbol {
            for child in &ast_root.children {
                self.generate_node(child, writer);
            }
        }

        // Data segment for strings
        let mut offset = 0;
        for (content, _label) in &self.string_literals {
            let bytes = content.as_bytes();
            write!(writer, "  (data (i32.const {}) \"", offset).unwrap();
            for &b in bytes {
                if b == b'\"' || b == b'\\' || b < 32 || b > 126 {
                    write!(writer, "\\{:02x}", b).unwrap();
                } else {
                    write!(writer, "{}", b as char).unwrap();
                }
            }
            writeln!(writer, "\\00\")").unwrap();
            offset += bytes.len() + 1;
        }

        writeln!(writer, ")").unwrap();
    }

    fn generate_node<W: Write>(&mut self, node: &AbstractSyntaxTreeNode, writer: &mut W) {
        match &node.symbol {
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFunctionDec {
                name,
                params,
                return_type,
                body,
            } => {
                self.functions.insert(name.clone(), return_type.clone());
                
                let mut param_str = String::new();
                self.scopes.push(HashMap::new());
                let mut local_idx = 0;

                for (ptype, pname) in params {
                    let wasm_type = self.to_wasm_type(ptype);
                    param_str.push_str(&format!(" (param ${} {})", pname, wasm_type));
                    self.scopes.last_mut().unwrap().insert(pname.clone(), WasmVarEntry { _index: local_idx, type_: ptype.clone() });
                    local_idx += 1;
                }

                let ret_str = if *return_type != Type::Void {
                    format!(" (result {})", self.to_wasm_type(return_type))
                } else {
                    "".to_string()
                };

                writeln!(writer, "  (func ${} (export \"{}\"){}{}", name, name, param_str, ret_str).unwrap();
                
                // Track locals for this function
                let mut locals = Vec::new();
                self.collect_locals(node, &mut locals, &mut local_idx);
                
                // Ensure unique local names in Wasm
                let mut seen_locals = HashSet::new();
                for (ptype, pname) in params {
                    seen_locals.insert(pname.clone());
                }

                for (lname, ltype, _lidx) in &locals {
                    if !seen_locals.contains(lname) {
                        writeln!(writer, "    (local ${} {})", lname, self.to_wasm_type(ltype)).unwrap();
                        seen_locals.insert(lname.clone());
                    }
                }

                for stmt in body {
                    self.generate_statement(stmt, writer);
                }

                // Satisfy Wasm validator for fallthrough in functions with result
                if *return_type != Type::Void {
                    if *return_type == Type::F32S {
                        writeln!(writer, "    f32.const 0.0").unwrap();
                    } else {
                        writeln!(writer, "    i32.const 0").unwrap();
                    }
                }
                
                writeln!(writer, "  )").unwrap();
                self.scopes.pop();
            }
            _ => { /* Globals or other top-level items logic here */ }
        }
    }

    fn generate_statement<W: Write>(&mut self, node: &AbstractSyntaxTreeNode, writer: &mut W) {
        match &node.symbol {
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableDeclaration { name, type_: _, value } => {
                let _entry = self.lookup_var(name);
                self.generate_expr(value, writer);
                writeln!(writer, "    local.set ${}", name).unwrap();
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableAssignment { name, value } => {
                self.generate_expr(value, writer);
                writeln!(writer, "    local.set ${}", name).unwrap();
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolReturn(opt_expr) => {
                if let Some(expr) = opt_expr {
                    self.generate_expr(expr, writer);
                }
                writeln!(writer, "    return").unwrap();
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFunctionCall { name, args } => {
                if name == "print" {
                    for arg in args {
                        let ty = self.get_expr_type(arg);
                        self.generate_expr(arg, writer);
                        if ty == Type::String {
                            writeln!(writer, "    call $print_str").unwrap();
                        } else {
                            writeln!(writer, "    call $print_int").unwrap();
                        }
                    }
                } else {
                    for arg in args {
                        self.generate_expr(arg, writer);
                    }
                    writeln!(writer, "    call ${}", name).unwrap();
                }
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolIf { condition, body, else_body } => {
            self.generate_expr(condition, writer);
            writeln!(writer, "    if").unwrap();
            for stmt in body {
                self.generate_statement(stmt, writer);
            }
            if let Some(eb) = else_body {
                writeln!(writer, "    else").unwrap();
                self.generate_statement(eb, writer);
            }
            writeln!(writer, "    end").unwrap();
        }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolBlock { body } => {
                for stmt in body {
                    self.generate_statement(stmt, writer);
                }
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFor { iterator_name, iterator_begin, iterator_end, body } => {
                // Simplified for loop: init, loop, check, body, inc, branch
                self.generate_expr(iterator_begin, writer);
                writeln!(writer, "    local.set ${}", iterator_name).unwrap();
                writeln!(writer, "    loop $loop_{}", iterator_name).unwrap();
                
                // Condition: i <= end
                writeln!(writer, "    local.get ${}", iterator_name).unwrap();
                self.generate_expr(iterator_end, writer);
                writeln!(writer, "    i32.le_s").unwrap();
                
                writeln!(writer, "    if").unwrap();
            for stmt in body {
                self.generate_statement(stmt, writer);
            }
            
            // Increment
            writeln!(writer, "      local.get ${}", iterator_name).unwrap();
            writeln!(writer, "      i32.const 1").unwrap();
            writeln!(writer, "      i32.add").unwrap();
            writeln!(writer, "      local.set ${}", iterator_name).unwrap();
            writeln!(writer, "      br $loop_{}", iterator_name).unwrap();
            writeln!(writer, "    end").unwrap();
            writeln!(writer, "    end").unwrap();
            }
            _ => {}
        }
    }

    fn generate_expr<W: Write>(&mut self, expr: &Expr, writer: &mut W) {
        match expr {
            Expr::Int(i) => writeln!(writer, "    i32.const {}", i).unwrap(),
            Expr::Ident(name) => writeln!(writer, "    local.get ${}", name).unwrap(),
            Expr::String(s) => {
                let offset = self.get_string_offset(s);
                writeln!(writer, "    i32.const {}", offset).unwrap();
            }
            Expr::BinaryOp { left, op, right } => {
                self.generate_expr(left, writer);
                self.generate_expr(right, writer);
                match op {
                    BinOpType::Add => writeln!(writer, "    i32.add").unwrap(),
                    BinOpType::Subtract => writeln!(writer, "    i32.sub").unwrap(),
                    BinOpType::Multiply => writeln!(writer, "    i32.mul").unwrap(),
                    BinOpType::Divide => writeln!(writer, "    i32.div_s").unwrap(),
                    BinOpType::Equal => writeln!(writer, "    i32.eq").unwrap(),
                    BinOpType::NotEqual => writeln!(writer, "    i32.ne").unwrap(),
                    BinOpType::LessThan => writeln!(writer, "    i32.lt_s").unwrap(),
                    BinOpType::LessThanOrEqual => writeln!(writer, "    i32.le_s").unwrap(),
                    BinOpType::GreaterThan => writeln!(writer, "    i32.gt_s").unwrap(),
                    BinOpType::GreaterThanOrEqual => writeln!(writer, "    i32.ge_s").unwrap(),
                }
            }
            Expr::FunctionCall { name, args } => {
                for arg in args {
                    self.generate_expr(arg, writer);
                }
                writeln!(writer, "    call ${}", name).unwrap();
            }
            _ => { /* More expression types */ }
        }
    }

    fn to_wasm_type(&self, t: &Type) -> &'static str {
        match t {
            Type::I32S | Type::Bool | Type::Char | Type::String => "i32",
            Type::F32S => "f32", // Group18 uses 32-bit floats
            _ => "i32", // Pointers are i32 in Wasm32
        }
    }

    fn lookup_var(&self, name: &str) -> &WasmVarEntry {
        for scope in self.scopes.iter().rev() {
            if let Some(entry) = scope.get(name) {
                return entry;
            }
        }
        panic!("WasmGenerator: Undefined variable {}", name);
    }

    fn get_string_offset(&mut self, s: &str) -> usize {
        if let Some(_l) = self.string_literals.get(s) {
            // We need to calculate cumulative offset. Let's do it simply during data segment generation.
            // For now, let's just use a map to labels and calculate later, 
            // BUT WAT is easier if we just know the offset now.
            let mut offset = 0;
            for (content, _label) in &self.string_literals {
                if content == s { return offset; }
                offset += content.as_bytes().len() + 1;
            }
            let label = format!("str_{}", self.string_counter);
            self.string_counter += 1;
            self.string_literals.insert(s.to_string(), label);
            offset
        } else {
            let mut offset = 0;
            for (content, _label) in &self.string_literals {
                offset += content.as_bytes().len() + 1;
            }
            let label = format!("str_{}", self.string_counter);
            self.string_counter += 1;
            self.string_literals.insert(s.to_string(), label);
            offset
        }
    }

    fn collect_locals(&mut self, node: &AbstractSyntaxTreeNode, locals: &mut Vec<(String, Type, u32)>, next_idx: &mut u32) {
        match &node.symbol {
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableDeclaration { name, type_, .. } => {
                locals.push((name.clone(), type_.clone(), *next_idx));
                self.scopes.last_mut().unwrap().insert(name.clone(), WasmVarEntry { _index: *next_idx, type_: type_.clone() });
                *next_idx += 1;
            }
            _ => {
                for child in &node.children {
                    self.collect_locals(child, locals, next_idx);
                }
                if let AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFunctionDec { body, .. } = &node.symbol {
                    for stmt in body {
                         self.collect_locals(stmt, locals, next_idx);
                    }
                }
                if let AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolIf { body, else_body, .. } = &node.symbol {
                    for stmt in body {
                        self.collect_locals(stmt, locals, next_idx);
                    }
                    if let Some(eb) = else_body {
                        self.collect_locals(eb, locals, next_idx);
                    }
                }
                if let AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFor { iterator_name, body, .. } = &node.symbol {
                     locals.push((iterator_name.clone(), Type::I32S, *next_idx));
                     self.scopes.last_mut().unwrap().insert(iterator_name.clone(), WasmVarEntry { _index: *next_idx, type_: Type::I32S });
                     *next_idx += 1;
                     for stmt in body {
                         self.collect_locals(stmt, locals, next_idx);
                     }
                }
            }
        }
    }

    fn get_expr_type(&self, expr: &Expr) -> Type {
        match expr {
            Expr::Int(_) => Type::I32S,
            Expr::String(_) => Type::String,
            Expr::Ident(name) => {
                for scope in self.scopes.iter().rev() {
                    if let Some(entry) = scope.get(name) {
                        return entry.type_.clone();
                    }
                }
                Type::I32S
            }
            _ => Type::I32S, // Simplified
        }
    }
}
