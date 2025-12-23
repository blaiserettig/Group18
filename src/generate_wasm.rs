use crate::parse::{AbstractSyntaxTreeNode, AbstractSyntaxTreeSymbol, BinOpType, Expr, Type};
use std::collections::{HashMap, HashSet};
use std::io::Write;

pub struct WasmGenerator {
    scopes: Vec<HashMap<String, WasmVarEntry>>,
    functions: HashMap<String, Type>, // return types
    string_literals: Vec<String>, // ordered content
    _string_counter: usize,
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
            // Pre-populate with strings used by codegen to ensure they are accounted for in heap calculation
            string_literals: vec!["true".to_string(), "false".to_string(), "\n".to_string()],
            _string_counter: 0,
        }
    }

    pub fn generate_wat<W: Write>(&mut self, ast_root: &AbstractSyntaxTreeNode, writer: &mut W) {
        writeln!(writer, "(module").unwrap();
        
        // Imports
        writeln!(writer, "  (import \"env\" \"print_int\" (func $print_int (param i32)))").unwrap();
        writeln!(writer, "  (import \"env\" \"print_str\" (func $print_str (param i32)))").unwrap();
        writeln!(writer, "  (import \"env\" \"print_float\" (func $print_float (param f32)))").unwrap();
        writeln!(writer, "  (import \"env\" \"memory\" (memory 1))").unwrap();

        if let AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolEntry = &ast_root.symbol {
            // First pass to collect string literals
            self.collect_all_strings(ast_root);

            // Global heap pointer initialization
            let mut string_section_size = 0;
            for s in &self.string_literals {
                string_section_size += s.as_bytes().len() + 1;
            }
            // Align to 8 bytes just in case
            let heap_start = (string_section_size + 7) & !7;
            writeln!(writer, "  (global $heap_ptr (mut i32) (i32.const {}))", heap_start).unwrap();

            for child in &ast_root.children {
                self.generate_node(child, writer);
            }
        }

        // Data segment for strings
        let mut offset = 0;
        for content in &self.string_literals {
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
                for (_ptype, pname) in params {
                    seen_locals.insert(pname.clone());
                }

                for (lname, ltype, _lidx) in &locals {
                    if !seen_locals.contains(lname) {
                        writeln!(writer, "    (local ${} {})", lname, self.to_wasm_type(ltype)).unwrap();
                        seen_locals.insert(lname.clone());
                    }
                }
                
                // Scratch local for array construction and intermediate indexing
                writeln!(writer, "    (local $base_addr i32)").unwrap();
                writeln!(writer, "    (local $tmp_val i32)").unwrap();

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
                if name == "print" || name == "println" {
                    let is_println = name == "println";
                    for arg in args {
                        let ty = self.get_expr_type(arg);
                        self.generate_expr(arg, writer);
                        match ty {
                            Type::String => {
                                writeln!(writer, "    call $print_str").unwrap();
                            }
                            Type::F32S => {
                                writeln!(writer, "    call $print_float").unwrap();
                            }
                            Type::Bool => {
                                // Boolean printing: if 1 print "true", else "false"
                                // We need to handle this carefully.
                                // generate_expr put 0 or 1 on stack.
                                // We can use if/else or select.
                                // Simplest: if (val) { call print("true") } else { call print("false") }
                                
                                writeln!(writer, "    if").unwrap();
                                let true_off = self.get_string_offset("true");
                                writeln!(writer, "      i32.const {}", true_off).unwrap();
                                writeln!(writer, "      call $print_str").unwrap();
                                writeln!(writer, "    else").unwrap();
                                let false_off = self.get_string_offset("false");
                                writeln!(writer, "      i32.const {}", false_off).unwrap();
                                writeln!(writer, "      call $print_str").unwrap();
                                writeln!(writer, "    end").unwrap();
                            }
                            _ => {
                                // Int, Char (as int)
                                writeln!(writer, "    call $print_int").unwrap();
                            }
                        }
                    }
                    if is_println {
                        let nl_off = self.get_string_offset("\n");
                        writeln!(writer, "    i32.const {}", nl_off).unwrap();
                        writeln!(writer, "    call $print_str").unwrap();
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
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolArrayIndexAssignment { array, index, value } => {
                self.generate_expr(array, writer);
                self.generate_expr(index, writer);
                writeln!(writer, "    i32.const 4").unwrap();
                writeln!(writer, "    i32.mul").unwrap();
                writeln!(writer, "    i32.add").unwrap();
                self.generate_expr(value, writer);
                
                let val_type = self.get_expr_type(value);
                if val_type == Type::F32S {
                    writeln!(writer, "    f32.store").unwrap();
                } else {
                    writeln!(writer, "    i32.store").unwrap();
                }
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
                
                // Condition: i < end (Group18 'to' is exclusive like x64)
                writeln!(writer, "    local.get ${}", iterator_name).unwrap();
                self.generate_expr(iterator_end, writer);
                writeln!(writer, "    i32.lt_s").unwrap();
                
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
                
                let left_type = self.get_expr_type(left);
                let is_float = left_type == Type::F32S;
                
                match op {
                    BinOpType::Add => {
                        if is_float { writeln!(writer, "    f32.add").unwrap(); }
                        else { writeln!(writer, "    i32.add").unwrap(); }
                    },
                    BinOpType::Subtract => {
                        if is_float { writeln!(writer, "    f32.sub").unwrap(); }
                        else { writeln!(writer, "    i32.sub").unwrap(); }
                    },
                    BinOpType::Multiply => {
                        if is_float { writeln!(writer, "    f32.mul").unwrap(); }
                        else { writeln!(writer, "    i32.mul").unwrap(); }
                    },
                    BinOpType::Divide => {
                        if is_float { writeln!(writer, "    f32.div").unwrap(); }
                        else { writeln!(writer, "    i32.div_s").unwrap(); }
                    },
                    BinOpType::Equal => {
                        if is_float { writeln!(writer, "    f32.eq").unwrap(); }
                        else { writeln!(writer, "    i32.eq").unwrap(); }
                    },
                    BinOpType::NotEqual => {
                        if is_float { writeln!(writer, "    f32.ne").unwrap(); }
                        else { writeln!(writer, "    i32.ne").unwrap(); }
                    },
                    BinOpType::LessThan => {
                        if is_float { writeln!(writer, "    f32.lt").unwrap(); }
                        else { writeln!(writer, "    i32.lt_s").unwrap(); }
                    },
                    BinOpType::LessThanOrEqual => {
                        if is_float { writeln!(writer, "    f32.le").unwrap(); }
                        else { writeln!(writer, "    i32.le_s").unwrap(); }
                    },
                    BinOpType::GreaterThan => {
                        if is_float { writeln!(writer, "    f32.gt").unwrap(); }
                        else { writeln!(writer, "    i32.gt_s").unwrap(); }
                    },
                    BinOpType::GreaterThanOrEqual => {
                        if is_float { writeln!(writer, "    f32.ge").unwrap(); }
                        else { writeln!(writer, "    i32.ge_s").unwrap(); }
                    },
                }
            }
            Expr::FunctionCall { name, args } => {
                for arg in args {
                    self.generate_expr(arg, writer);
                }
                writeln!(writer, "    call ${}", name).unwrap();
            }
            Expr::Bool(b) => writeln!(writer, "    i32.const {}", if *b { 1 } else { 0 }).unwrap(),
            Expr::Char(c) => writeln!(writer, "    i32.const {}", *c as u32).unwrap(),
            Expr::UnaryOp { op: _, expr } => {
                let ty = self.get_expr_type(expr);
                if ty == Type::F32S {
                    self.generate_expr(expr, writer);
                    writeln!(writer, "    f32.neg").unwrap();
                } else {
                    writeln!(writer, "    i32.const 0").unwrap();
                    self.generate_expr(expr, writer);
                    writeln!(writer, "    i32.sub").unwrap();
                }
            }
            Expr::ArrayIndex { array, index } => {
                self.generate_expr(array, writer);
                self.generate_expr(index, writer);
                writeln!(writer, "    i32.const 4").unwrap();
                writeln!(writer, "    i32.mul").unwrap();
                writeln!(writer, "    i32.add").unwrap();
                
                let array_type = self.get_expr_type(array);
                let elem_type = if let Type::Array(inner) = array_type {
                    *inner
                } else {
                    Type::I32S
                };

                if elem_type == Type::F32S {
                    writeln!(writer, "    f32.load").unwrap();
                } else {
                    writeln!(writer, "    i32.load").unwrap();
                }
            }
            Expr::ArrayLiteral(elements) => {
                let size = elements.len() * 4;
                
                // Save current base_addr to WASM stack
                writeln!(writer, "    local.get $base_addr").unwrap();
 
                // 1. Capture base address
                writeln!(writer, "    global.get $heap_ptr").unwrap();
                writeln!(writer, "    local.set $base_addr").unwrap();
                
                // 2. Advance heap_ptr BEFORE filling elements to avoid clobbering nested arrays
                writeln!(writer, "    global.get $heap_ptr").unwrap();
                writeln!(writer, "    i32.const {}", size).unwrap();
                writeln!(writer, "    i32.add").unwrap();
                writeln!(writer, "    global.set $heap_ptr").unwrap();
                
                // 3. Store elements
                for (i, elem) in elements.iter().enumerate() {
                    writeln!(writer, "    local.get $base_addr").unwrap();
                    if i > 0 {
                        writeln!(writer, "    i32.const {}", i * 4).unwrap();
                        writeln!(writer, "    i32.add").unwrap();
                    }
                    self.generate_expr(elem, writer);
                    
                    let elem_type = self.get_expr_type(elem);
                    if elem_type == Type::F32S {
                        writeln!(writer, "    f32.store").unwrap();
                    } else {
                        writeln!(writer, "    i32.store").unwrap();
                    }
                }
                
                // 4. Restore the prev base_addr from the stack while keeping the current base_addr as the result
                writeln!(writer, "    local.get $base_addr").unwrap();
                writeln!(writer, "    local.set $tmp_val").unwrap();
                writeln!(writer, "    local.set $base_addr").unwrap();
                writeln!(writer, "    local.get $tmp_val").unwrap();
            }
            Expr::Float(f) => writeln!(writer, "    f32.const {}", f).unwrap(),
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
        let mut offset = 0;
        for content in &self.string_literals {
            if content == s {
                return offset;
            }
            offset += content.as_bytes().len() + 1;
        }
        // If not found, it must have been missed during pre-scan
        let off = offset;
        self.string_literals.push(s.to_string());
        off
    }

    fn collect_all_strings(&mut self, node: &AbstractSyntaxTreeNode) {
        match &node.symbol {
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFunctionDec { body, .. } => {
                for stmt in body {
                    self.collect_all_strings(stmt);
                }
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableDeclaration { value, .. } => {
                self.collect_strings_in_expr(value);
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableAssignment { value, .. } => {
                self.collect_strings_in_expr(value);
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolReturn(opt_expr) => {
                if let Some(expr) = opt_expr {
                    self.collect_strings_in_expr(expr);
                }
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFunctionCall { args, .. } => {
                for arg in args {
                    self.collect_strings_in_expr(arg);
                }
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolIf { condition, body, else_body } => {
                self.collect_strings_in_expr(condition);
                for stmt in body {
                    self.collect_all_strings(stmt);
                }
                if let Some(eb) = else_body {
                    self.collect_all_strings(eb);
                }
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolBlock { body } => {
                for stmt in body {
                    self.collect_all_strings(stmt);
                }
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFor { iterator_begin, iterator_end, body, .. } => {
                self.collect_strings_in_expr(iterator_begin);
                self.collect_strings_in_expr(iterator_end);
                for stmt in body {
                    self.collect_all_strings(stmt);
                }
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolArrayIndexAssignment { array, index, value } => {
                self.collect_strings_in_expr(array);
                self.collect_strings_in_expr(index);
                self.collect_strings_in_expr(value);
            }
            _ => {
                for child in &node.children {
                    self.collect_all_strings(child);
                }
            }
        }
    }

    fn collect_strings_in_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::String(s) => {
                if !self.string_literals.contains(s) {
                    self.string_literals.push(s.to_string());
                }
            }
            Expr::BinaryOp { left, right, .. } => {
                self.collect_strings_in_expr(left);
                self.collect_strings_in_expr(right);
            }
            Expr::UnaryOp { expr, .. } => {
                self.collect_strings_in_expr(expr);
            }
            Expr::FunctionCall { args, .. } => {
                for arg in args {
                    self.collect_strings_in_expr(arg);
                }
            }
            Expr::ArrayLiteral(elements) => {
                for elem in elements {
                    self.collect_strings_in_expr(elem);
                }
            }
            Expr::ArrayIndex { array, index } => {
                self.collect_strings_in_expr(array);
                self.collect_strings_in_expr(index);
            }
            _ => {}
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
            Expr::Float(_) => Type::F32S,
            Expr::Bool(_) => Type::Bool,
            Expr::Char(_) => Type::Char,
            Expr::String(_) => Type::String,
            Expr::Ident(name) => {
                for scope in self.scopes.iter().rev() {
                    if let Some(entry) = scope.get(name) {
                        return entry.type_.clone();
                    }
                }
                Type::I32S
            }
            Expr::UnaryOp { expr, .. } => self.get_expr_type(expr),
            Expr::BinaryOp { left, op, .. } => {
                match op {
                    BinOpType::Add | BinOpType::Subtract | BinOpType::Multiply | BinOpType::Divide => {
                        self.get_expr_type(left)
                    }
                    _ => Type::Bool, // Comparisons result in Bool
                }
            }
            Expr::ArrayIndex { array, .. } => {
                let array_type = self.get_expr_type(array);
                if let Type::Array(inner) = array_type {
                    *inner
                } else {
                    Type::I32S // Fallback, shouldn't happen in valid programs
                }
            }
            Expr::FunctionCall { name, .. } => {
                if let Some(ret_type) = self.functions.get(name) {
                    ret_type.clone()
                } else {
                    Type::I32S // Default or external functions
                }
            }
            _ => Type::I32S,
        }
    }
}
