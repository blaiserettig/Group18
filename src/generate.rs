use crate::parse::{AbstractSyntaxTreeNode, AbstractSyntaxTreeSymbol, BinOpType, Expr, Type};
use std::collections::{HashMap, HashSet};
use std::io::Write;

#[derive(Clone, Copy, PartialEq)]
enum VariableLocation {
    Global,
    Local(i32), // offset from rbp
}

#[derive(Clone)]
struct GeneratorVarEntry {
    location: VariableLocation,
    type_: Type,
}

pub struct Generator {
    scopes: Vec<HashMap<String, GeneratorVarEntry>>,
    functions: HashMap<String, Type>, // return types
    current_stack_offset: i32,
    global_vars: HashSet<String>,
    string_literals: HashMap<String, String>, // content, label
    string_counter: usize,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // Global scope
            functions: HashMap::new(),
            current_stack_offset: 0,
            global_vars: HashSet::new(),
            string_literals: HashMap::new(),
            string_counter: 0,
        }
    }

    pub fn generate_boilerplate<W: Write>(&mut self, writer: &mut W) {
        write!(
            writer,
            "{}",
            "bits 64\ndefault rel\n\nsegment .text\nglobal mainCRTStartup\nextern ExitProcess\nextern puts\nextern printf\n\nmainCRTStartup:\n"
        )
        .expect("Unable to write to file.");

        // Add format strings to string_literals early
        self.string_literals.insert("%d\n".to_string(), "fmt_int".to_string());
        self.string_literals.insert("%s\n".to_string(), "fmt_str".to_string());
    }

    pub fn generate_x64<W: Write>(
        &mut self,
        ast_root: &AbstractSyntaxTreeNode,
        writer: &mut W,
    ) {
        match &ast_root.symbol {
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolEntry => {
                // separate FunctionDecs from other statements
                let functions: Vec<&AbstractSyntaxTreeNode> = ast_root
                    .children
                    .iter()
                    .filter(|child| matches!(child.symbol, AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFunctionDec { .. }))
                    .collect();

                let statements: Vec<&AbstractSyntaxTreeNode> = ast_root
                    .children
                    .iter()
                    .filter(|child| !matches!(child.symbol, AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFunctionDec { .. }))
                    .collect();
                
                writeln!(writer, "    jmp main_entry").unwrap();
                
                for func in &functions {
                     self.generate_x64(func, writer);
                }

                writeln!(writer, "main_entry:").unwrap();

                // call mandatory main fn
                writeln!(writer, "    call func_main").unwrap();
                
                // return value of main is in eax... use it for exit process
                writeln!(writer, "    mov rcx, rax").unwrap();
                writeln!(writer, "    and rsp, -16").unwrap();
                writeln!(writer, "    sub rsp, 32").unwrap();
                writeln!(writer, "    call ExitProcess").unwrap();

                if !self.global_vars.is_empty() {
                    writeln!(writer, "\nsegment .bss").unwrap();
                    for var in &self.global_vars {
                        writeln!(writer, "{} resd 1", var).unwrap();
                    }
                }

                if !self.string_literals.is_empty() {
                    writeln!(writer, "\nsegment .data").unwrap();
                    for (content, label) in &self.string_literals {
                        // NASM backticks support C-style escapes.
                        let escaped = content.replace("\\", "\\\\").replace("\n", "\\n").replace("\r", "\\r").replace("\t", "\\t").replace("`", "\\`");
                        writeln!(writer, "{} db `{}`, 0", label, escaped).unwrap();
                    }
                }
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFunctionDec {
                name,
                params,
                return_type,
                body,
            } => {
                self.functions.insert(name.clone(), return_type.clone());
                let func_label = format!("func_{}", name);
                writeln!(writer, "{}:", func_label).unwrap();
                
                writeln!(writer, "    push rbp").unwrap();
                writeln!(writer, "    mov rbp, rsp").unwrap();

                self.scopes.push(HashMap::new());
                self.current_stack_offset = 0;
                let mut total_locals_size = 0;
                for stmt in body {
                    total_locals_size += self.calculate_stack_size(stmt);
                }

                if total_locals_size > 0 {
                    writeln!(writer, "    sub rsp, {}", total_locals_size).unwrap();
                }

                let mut param_offset = 16;
                // params vector is left to right
                // if pushed R->L, first param is closest to RBP
                for (param_type, pname) in params {
                    self.scopes.last_mut().unwrap().insert(
                        pname.clone(),
                        GeneratorVarEntry {
                            location: VariableLocation::Local(param_offset),
                            type_: param_type.clone(),
                        },
                    );
                    param_offset += 8; // Assuming 64-bit/8-byte slots on stack
                }

                for stmt in body {
                    self.generate_x64(stmt, writer);
                }

                writeln!(writer, "    leave").unwrap();
                writeln!(writer, "    ret").unwrap();
                
                self.scopes.pop();
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFunctionCall { name, args } => {
                if name == "print" {
                    for arg in args.iter().rev() {
                        let ty = self.get_expr_type(arg);
                        match ty {
                            Type::String => {
                                self.generate_expr_into_register(arg, "rcx", writer);
                                writeln!(writer, "    and rsp, -16").unwrap();
                                writeln!(writer, "    sub rsp, 32").unwrap();
                                writeln!(writer, "    call puts").unwrap();
                                writeln!(writer, "    add rsp, 32").unwrap();
                            }
                            Type::I32S | Type::Bool | Type::Char => {
                                // Use printf with fmt_int
                                self.generate_expr_into_register(arg, "rdx", writer);
                                let fmt_label = if let Some(l) = self.string_literals.get("%d\n") { l.clone() } else { "fmt_int".to_string() };
                                writeln!(writer, "    lea rcx, [{}]", fmt_label).unwrap();
                                writeln!(writer, "    and rsp, -16").unwrap();
                                writeln!(writer, "    sub rsp, 32").unwrap();
                                writeln!(writer, "    call printf").unwrap();
                                writeln!(writer, "    add rsp, 32").unwrap();
                            }
                            _ => {
                                // Fallback to puts for pointers
                                self.generate_expr_into_register(arg, "rcx", writer);
                                writeln!(writer, "    and rsp, -16").unwrap();
                                writeln!(writer, "    sub rsp, 32").unwrap();
                                writeln!(writer, "    call puts").unwrap();
                                writeln!(writer, "    add rsp, 32").unwrap();
                            }
                        }
                    }
                } else {
                    // reg function call
                    for arg in args.iter().rev() {
                        self.generate_expr_into_register(arg, "eax", writer);
                        writeln!(writer, "    push rax").unwrap();
                    }
                    
                    let func_label = format!("func_{}", name);
                    writeln!(writer, "    call {}", func_label).unwrap();
                    
                    if !args.is_empty() {
                        writeln!(writer, "    add rsp, {}", args.len() * 8).unwrap();
                    }
                }
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolReturn(opt_expr) => {
                if let Some(expr) = opt_expr {
                    self.generate_expr_into_register(expr, "eax", writer);
                }
                writeln!(writer, "    leave").unwrap();
                writeln!(writer, "    ret").unwrap();
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolExit(expr) => {
                self.generate_expr_into_register(expr, "eax", writer);
                writeln!(writer, "    mov rcx, rax").unwrap();
                // x64 ABI..... align stack to 16 bytes and allocate 32 bytes shadow space
                writeln!(writer, "    and rsp, -16").unwrap();
                writeln!(writer, "    sub rsp, 32").unwrap();
                writeln!(writer, "    call ExitProcess").unwrap();
            }


            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableDeclaration {
                name,
                type_: _type_,
                value,
            } => {
                if self.scopes.len() == 1 { // global
                    self.global_vars.insert(name.clone());
                    self.scopes[0].insert(name.clone(), GeneratorVarEntry { location: VariableLocation::Global, type_: _type_.clone() });
                    self.match_variable_helper(name, value, writer);
                } else { // local
                    let size = 8; // Always allocate 8 bytes (pointers or 32-bit values)
                    self.current_stack_offset -= size;
                    let offset = self.current_stack_offset;
                    self.scopes.last_mut().unwrap().insert(name.clone(), GeneratorVarEntry { location: VariableLocation::Local(offset), type_: _type_.clone() });
                    self.match_variable_helper(name, value, writer);
                }
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableAssignment {
                name,
                value,
            } => {
                self.match_variable_helper(name, value, writer);
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFor {
                iterator_name,
                iterator_begin,
                iterator_end,
                body,
            } => {
                if self.scopes.len() == 1 {
                    self.global_vars.insert(iterator_name.clone());
                    self.scopes[0].insert(iterator_name.clone(), GeneratorVarEntry { location: VariableLocation::Global, type_: Type::I32S });
                } else {
                    self.current_stack_offset -= 8;
                    self.scopes
                        .last_mut()
                        .unwrap()
                        .insert(iterator_name.clone(), GeneratorVarEntry { location: VariableLocation::Local(self.current_stack_offset), type_: Type::I32S });
                }

                let loop_label = format!("loop_begin_{}", iterator_name);
                let end_label = format!("loop_end_{}", iterator_name);

                self.generate_expr_into_register(iterator_begin, "eax", writer);
                
                let iter_loc = self.lookup_var(iterator_name).location;
                match iter_loc {
                    VariableLocation::Global => writeln!(writer, "    mov dword [{}], eax", iterator_name).unwrap(),
                    VariableLocation::Local(off) => writeln!(writer, "    mov dword [rbp{}], eax", if off < 0 { format!("{}", off) } else { format!("+{}", off) }).unwrap(),
                }

                writeln!(writer, "{}:", loop_label).unwrap();

                match iter_loc {
                    VariableLocation::Global => writeln!(writer, "    mov eax, dword [{}]", iterator_name).unwrap(),
                    VariableLocation::Local(off) => writeln!(writer, "    mov eax, dword [rbp{}]", if off < 0 { format!("{}", off) } else { format!("+{}", off) }).unwrap(),
                }
                
                self.generate_expr_into_register(iterator_end, "ebx", writer);
                writeln!(writer, "    cmp eax, ebx").unwrap();
                writeln!(writer, "    jg {}", end_label).unwrap();

                for stmt in body {
                    self.generate_x64(stmt, writer);
                }

                match iter_loc {
                    VariableLocation::Global => writeln!(writer, "    mov eax, dword [{}]", iterator_name).unwrap(),
                    VariableLocation::Local(off) => writeln!(writer, "    mov eax, dword [rbp{}]", if off < 0 { format!("{}", off) } else { format!("+{}", off) }).unwrap(),
                }
                writeln!(writer, "    inc eax").unwrap();
                match iter_loc {
                    VariableLocation::Global => writeln!(writer, "    mov dword [{}], eax", iterator_name).unwrap(),
                    VariableLocation::Local(off) => writeln!(writer, "    mov dword [rbp{}], eax", if off < 0 { format!("{}", off) } else { format!("+{}", off) }).unwrap(),
                }

                writeln!(writer, "    jmp {}", loop_label).unwrap();

                writeln!(writer, "{}:", end_label).unwrap();
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolIf {
                condition,
                body,
                else_body,
            } => {
                self.generate_if(condition, body, else_body, writer);
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolBlock { body } => {
                for stmt in body {
                    self.generate_x64(stmt, writer);
                }
            }
        }
    }

    fn match_variable_helper<W: Write>(
        &mut self,
        name: &String,
        value: &Expr,
        writer: &mut W,
    ) {
        let location = self.lookup_var(name).location;
        
        // Evaluate expression into rax
        self.generate_expr_into_register(value, "rax", writer);
        
        match location {
            VariableLocation::Global => {
                writeln!(writer, "    mov qword [{}], rax", name).unwrap();
            }
            VariableLocation::Local(off) => {
                writeln!(writer, "    mov qword [rbp{0}], rax", if off < 0 { format!("{}", off) } else { format!("+{}", off) }).unwrap();
            }
        }
    }
    
    fn lookup_var(&self, name: &str) -> GeneratorVarEntry {
        for scope in self.scopes.iter().rev() {
            if let Some(entry) = scope.get(name) {
                return entry.clone();
            }
        }
        GeneratorVarEntry { location: VariableLocation::Global, type_: Type::I32S } // fallback
    }

    fn generate_expr_into_register<W: Write>(
        &mut self,
        expr: &Expr,
        reg: &str,
        writer: &mut W,
    ) {
        match expr {
            Expr::Int(i) => {
                writeln!(writer, "    mov {}, {}", reg, i).unwrap();
            }
            Expr::Ident(name) => match self.lookup_var(name).location {
                VariableLocation::Global => {
                    if reg.starts_with('e') {
                        writeln!(writer, "    mov {}, dword [{}]", reg, name).unwrap()
                    } else {
                        writeln!(writer, "    mov {}, qword [{}]", reg, name).unwrap()
                    }
                }
                VariableLocation::Local(off) => {
                    let off_str = if off < 0 {
                        format!("{}", off)
                    } else {
                        format!("+{}", off)
                    };
                    if reg.starts_with('e') {
                        writeln!(writer, "    mov {}, dword [rbp{}]", reg, off_str).unwrap()
                    } else {
                        writeln!(writer, "    mov {}, qword [rbp{}]", reg, off_str).unwrap()
                    }
                }
            },
            Expr::Float(f) => {
                let bits = f.to_bits();
                writeln!(writer, "    mov {}, {}", reg, bits).unwrap();
            }
            Expr::Bool(b) => {
                let val = if *b { 1 } else { 0 };
                writeln!(writer, "    mov {}, {}", reg, val).unwrap();
            }
            Expr::Char(c) => {
                writeln!(writer, "    mov {}, {}", reg, *c as u32).unwrap();
            }
            Expr::String(s) => {
                let label = if let Some(existing_label) = self.string_literals.get(s) {
                    existing_label.clone()
                } else {
                    let new_label = format!("str_{}", self.string_counter);
                    self.string_counter += 1;
                    self.string_literals.insert(s.clone(), new_label.clone());
                    new_label
                };
                writeln!(writer, "    lea {}, [{}]", reg, label).unwrap();
            }
            Expr::BinaryOp { left, op, right } => {
                self.generate_binary_op(left, op, right, writer);
                if reg != "eax" && reg != "rax" {
                    writeln!(writer, "    mov {}, eax", reg).unwrap();
                } else if reg == "rax" {
                    writeln!(writer, "    mov eax, eax").unwrap(); // Zero-extend eax into rax
                }
            }
            Expr::FunctionCall { name, args } => {
                for arg in args.iter().rev() {
                    self.generate_expr_into_register(arg, "eax", writer);
                    writeln!(writer, "    push rax").unwrap();
                }
                let label = format!("func_{}", name);
                writeln!(writer, "    call {}", label).unwrap();
                if !args.is_empty() {
                    writeln!(writer, "    add rsp, {}", args.len() * 8).unwrap();
                }
                if reg != "rax" {
                    if reg == "eax" {
                        writeln!(writer, "    mov eax, eax").unwrap();
                    } else {
                        writeln!(writer, "    mov {}, rax", reg).unwrap();
                    }
                }
            }
            Expr::ArrayLiteral(elements) => {
                let size = (elements.len() as i32) * 8;
                self.current_stack_offset -= size;
                let addr_offset = self.current_stack_offset;
                
                // Initialize elements
                for (i, elem) in elements.iter().enumerate() {
                    let elem_offset = i as i32 * 8;
                    self.generate_expr_into_register(elem, "rax", writer);
                    let final_off = addr_offset + elem_offset;
                    writeln!(writer, "    mov qword [rbp{}], rax", if final_off < 0 { format!("{}", final_off) } else { format!("+{}", final_off) }).unwrap();
                }
                
                // Return start address in reg
                writeln!(writer, "    lea rax, [rbp{}]", if addr_offset < 0 { format!("{}", addr_offset) } else { format!("+{}", addr_offset) }).unwrap();
                if reg != "rax" {
                    if reg == "eax" {
                        writeln!(writer, "    mov eax, eax").unwrap();
                    } else {
                        writeln!(writer, "    mov {}, rax", reg).unwrap();
                    }
                }
            }
            Expr::ArrayIndex { array, index } => {
                // 1. Eval index into ebx
                self.generate_expr_into_register(index, "ebx", writer);
                writeln!(writer, "    movsxd rbx, ebx").unwrap(); // sign extend index

                // Save rbx during base address evaluation (potentially nested)
                writeln!(writer, "    push rbx").unwrap();

                // 2. Eval base array into rax (address)
                match &**array {
                    Expr::Ident(name) => {
                        let loc = self.lookup_var(name).location;
                        match loc {
                            VariableLocation::Local(off) => {
                                // Load the pointer into rax
                                writeln!(writer, "    mov rax, qword [rbp{0}]", if off < 0 { format!("{}", off) } else { format!("+{}", off) }).unwrap();
                            }
                            VariableLocation::Global => {
                                writeln!(writer, "    mov rax, qword [{}]", name).unwrap();
                            }
                        }
                    }
                    _ => {
                        self.generate_expr_into_register(array, "rax", writer);
                    }
                }

                // Restore rbx
                writeln!(writer, "    pop rbx").unwrap();
                
                // 3. Access element: [rax + rbx * 8]
                if reg.starts_with('e') {
                    writeln!(writer, "    mov {}, dword [rax + rbx * 8]", reg).unwrap();
                } else {
                    writeln!(writer, "    mov {}, qword [rax + rbx * 8]", reg).unwrap();
                }
            }
        }
    }

    fn generate_binary_op<W: Write>(
        &mut self,
        left: &Expr,
        op: &BinOpType,
        right: &Expr,
        writer: &mut W,
    ) {
        // Eval left into rax
        self.generate_expr_into_register(left, "rax", writer);

        // Push eax (save left value)
        writeln!(writer, "    push rax").unwrap();

        // Eval right into rbx
        self.generate_expr_into_register(right, "rbx", writer);

        // Restore left into eax
        writeln!(writer, "    pop rax").unwrap();

        match op {
            BinOpType::Add => {
                writeln!(writer, "    add eax, ebx").unwrap();
            }
            BinOpType::Subtract => {
                writeln!(writer, "    sub eax, ebx").unwrap();
            }
            BinOpType::Multiply => {
                writeln!(writer, "    imul eax, ebx").unwrap();
            }
            BinOpType::Divide => {
                writeln!(writer, "    cdq").unwrap(); // sign-extend eax into edx:eax
                writeln!(writer, "    idiv ebx").unwrap(); // eax = eax / ebx
            }

            // set eax to 1 or 0 on comparisons
            BinOpType::LessThan => {
                writeln!(writer, "    cmp eax, ebx").unwrap();
                writeln!(writer, "    setl al").unwrap();
                writeln!(writer, "    movzx eax, al").unwrap();
            }
            BinOpType::LessThanOrEqual => {
                writeln!(writer, "    cmp eax, ebx").unwrap();
                writeln!(writer, "    setle al").unwrap();
                writeln!(writer, "    movzx eax, al").unwrap();
            }
            BinOpType::GreaterThan => {
                writeln!(writer, "    cmp eax, ebx").unwrap();
                writeln!(writer, "    setg al").unwrap();
                writeln!(writer, "    movzx eax, al").unwrap();
            }
            BinOpType::GreaterThanOrEqual => {
                writeln!(writer, "    cmp eax, ebx").unwrap();
                writeln!(writer, "    setge al").unwrap();
                writeln!(writer, "    movzx eax, al").unwrap();
            }
            BinOpType::Equal => {
                writeln!(writer, "    cmp eax, ebx").unwrap();
                writeln!(writer, "    sete al").unwrap();
                writeln!(writer, "    movzx eax, al").unwrap();
            }
            BinOpType::NotEqual => {
                writeln!(writer, "    cmp eax, ebx").unwrap();
                writeln!(writer, "    setne al").unwrap();
                writeln!(writer, "    movzx eax, al").unwrap();
            }
        }
    }

    fn generate_if<W: Write>(
        &mut self,
        condition: &Expr,
        body: &Vec<AbstractSyntaxTreeNode>,
        else_body: &Option<Box<AbstractSyntaxTreeNode>>,
        writer: &mut W,
    ) {
        static mut LABEL_COUNT: usize = 0;
        let id = unsafe {
            let current = LABEL_COUNT;
            LABEL_COUNT += 1;
            current
        };

        let else_label = format!("else_{}", id);
        let end_label = format!("endif_{}", id);

        self.generate_expr_into_register(condition, "eax", writer);

        // Compare eax with 0 (false)
        writeln!(writer, "    cmp eax, 0").unwrap();

        // Jump if false → else or end if no else
        if else_body.is_some() {
            writeln!(writer, "    je {}", else_label).unwrap();
        } else {
            writeln!(writer, "    je {}", end_label).unwrap();
        }

        // IF BODY
        for stmt in body {
            self.generate_x64(stmt, writer);
        }

        // End of IF always jumps to end_label if else exists
        if else_body.is_some() {
            writeln!(writer, "    jmp {}", end_label).unwrap();
        }

        // ELSE or ELSE IF
        if let Some(else_ast) = else_body {
            writeln!(writer, "{}:", else_label).unwrap();
            self.generate_x64(else_ast, writer);
        }

        writeln!(writer, "{}:", end_label).unwrap();
    }

    fn calculate_stack_size(&self, node: &AbstractSyntaxTreeNode) -> i32 {
        let mut size = 0;
        match &node.symbol {
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableDeclaration { value, .. } => {
                size += 8;
                size += self.calculate_expr_stack_size(value);
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableAssignment { value, .. } => {
                size += self.calculate_expr_stack_size(value);
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolReturn(opt_expr) => {
                if let Some(expr) = opt_expr {
                    size += self.calculate_expr_stack_size(expr);
                }
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolExit(expr) => {
                size += self.calculate_expr_stack_size(expr);
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFor {
                iterator_begin,
                iterator_end,
                body,
                ..
            } => {
                size += 8; // iterator
                size += self.calculate_expr_stack_size(iterator_begin);
                size += self.calculate_expr_stack_size(iterator_end);
                for stmt in body {
                    size += self.calculate_stack_size(stmt);
                }
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolIf {
                condition,
                body,
                else_body,
                ..
            } => {
                size += self.calculate_expr_stack_size(condition);
                for stmt in body {
                    size += self.calculate_stack_size(stmt);
                }
                if let Some(else_node) = else_body {
                    size += self.calculate_stack_size(else_node);
                }
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolBlock { body } => {
                for stmt in body {
                    size += self.calculate_stack_size(stmt);
                }
            }
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFunctionCall { args, .. } => {
                for arg in args {
                    size += self.calculate_expr_stack_size(arg);
                }
            }
            _ => {
                for child in &node.children {
                    size += self.calculate_stack_size(child);
                }
            }
        }
        size
    }

    fn calculate_expr_stack_size(&self, expr: &Expr) -> i32 {
        let mut size = 0;
        match expr {
            Expr::ArrayLiteral(elements) => {
                size += (elements.len() as i32) * 8;
                for elem in elements {
                    size += self.calculate_expr_stack_size(elem);
                }
            }
            Expr::BinaryOp { left, right, .. } => {
                size += self.calculate_expr_stack_size(left);
                size += self.calculate_expr_stack_size(right);
            }
            Expr::FunctionCall { args, .. } => {
                for arg in args {
                    size += self.calculate_expr_stack_size(arg);
                }
            }
            Expr::ArrayIndex { array, index } => {
                size += self.calculate_expr_stack_size(array);
                size += self.calculate_expr_stack_size(index);
            }
            _ => {}
        }
        size
    }

    fn get_expr_type(&self, expr: &Expr) -> Type {
        match expr {
            Expr::Int(_) => Type::I32S,
            Expr::Float(_) => Type::F32S,
            Expr::Bool(_) => Type::Bool,
            Expr::Char(_) => Type::Char,
            Expr::String(_) => Type::String,
            Expr::Ident(name) => self.lookup_var(name).type_,
            Expr::BinaryOp { left, op, .. } => {
                let left_type = self.get_expr_type(left);
                match op {
                    BinOpType::Equal
                    | BinOpType::NotEqual
                    | BinOpType::LessThan
                    | BinOpType::LessThanOrEqual
                    | BinOpType::GreaterThan
                    | BinOpType::GreaterThanOrEqual => Type::Bool,
                    _ => left_type,
                }
            }
            Expr::FunctionCall { name, .. } => self
                .functions
                .get(name)
                .cloned()
                .unwrap_or(Type::I32S),
            Expr::ArrayLiteral(elements) => {
                if elements.is_empty() {
                    Type::Array(Box::new(Type::I32S))
                } else {
                    let elem_type = self.get_expr_type(&elements[0]);
                    Type::Array(Box::new(elem_type))
                }
            }
            Expr::ArrayIndex { array, .. } => {
                let array_type = self.get_expr_type(array);
                match array_type {
                    Type::Array(inner) => *inner,
                    _ => Type::I32S,
                }
            }
        }
    }
}
