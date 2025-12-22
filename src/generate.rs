use crate::parse::{AbstractSyntaxTreeNode, AbstractSyntaxTreeSymbol, BinOpType, Expr, Type};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

#[derive(Clone)]
enum VariableLocation {
    Global,
    Local(i32), // offset from rbp
}

pub struct Generator {
    scopes: Vec<HashMap<String, VariableLocation>>,
    current_stack_offset: i32,
    global_vars: HashSet<String>,
    string_literals: HashMap<String, String>, // content, label
    string_counter: usize,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // Global scope
            current_stack_offset: 0,
            global_vars: HashSet::new(),
            string_literals: HashMap::new(),
            string_counter: 0,
        }
    }

    pub fn generate_boilerplate(&mut self, writer: &mut BufWriter<&File>) {
        write!(
            writer,
            "{}",
            "bits 64\ndefault rel\n\nsegment .text\nglobal mainCRTStartup\nextern ExitProcess\nextern puts\n\nmainCRTStartup:\n"
        )
        .expect("Unable to write to file.");
    }

    pub fn generate_x64(
        &mut self,
        ast_root: &AbstractSyntaxTreeNode,
        writer: &mut BufWriter<&File>,
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
                        writeln!(writer, "{} db `{}`, 0", label, content).unwrap();
                    }
                }
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFunctionDec {
                name,
                params,
                return_type: _,
                body,
            } => {
                let func_label = format!("func_{}", name);
                writeln!(writer, "{}:", func_label).unwrap();
                
                writeln!(writer, "    push rbp").unwrap();
                writeln!(writer, "    mov rbp, rsp").unwrap();

                self.scopes.push(HashMap::new());
                self.current_stack_offset = 0;
    
                let mut param_offset = 16;
                // params vector is left to right
                // if pushed R->L, first param is closest to RBP
                for (_type, pname) in params {
                    self.scopes.last_mut().unwrap().insert(pname.clone(), VariableLocation::Local(param_offset));
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
                    // calls puts
                    for arg in args.iter().rev() {
                        self.generate_expr_into_register(arg, "rcx", writer);
                        writeln!(writer, "    and rsp, -16").unwrap();
                        writeln!(writer, "    sub rsp, 32").unwrap();
                        writeln!(writer, "    call puts").unwrap();
                        writeln!(writer, "    add rsp, 32").unwrap();
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
                    self.scopes[0].insert(name.clone(), VariableLocation::Global);
                    self.match_variable_helper(name, value, writer);
                } else { // local
                    let size = 8; // Always allocate 8 bytes (pointers or 32-bit values)
                    self.current_stack_offset -= size;
                    let offset = self.current_stack_offset;
                    self.scopes.last_mut().unwrap().insert(name.clone(), VariableLocation::Local(offset));
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
                     self.scopes[0].insert(iterator_name.clone(), VariableLocation::Global);
                } else {
                     self.current_stack_offset -= 8;
                     self.scopes.last_mut().unwrap().insert(iterator_name.clone(), VariableLocation::Local(self.current_stack_offset));
                }

                let loop_label = format!("loop_begin_{}", iterator_name);
                let end_label = format!("loop_end_{}", iterator_name);

                self.generate_expr_into_register(iterator_begin, "eax", writer);
                writeln!(writer, "    mov dword [{}], eax", iterator_name).unwrap();

                writeln!(writer, "{}:", loop_label).unwrap();

                writeln!(writer, "    mov eax, dword [{}]", iterator_name).unwrap();
                self.generate_expr_into_register(iterator_end, "ebx", writer);
                writeln!(writer, "    cmp eax, ebx").unwrap();
                writeln!(writer, "    jg {}", end_label).unwrap();

                for stmt in body {
                    self.generate_x64(stmt, writer);
                }

                writeln!(writer, "    mov eax, dword [{}]", iterator_name).unwrap();
                writeln!(writer, "    inc eax").unwrap();
                writeln!(writer, "    mov dword [{}], eax", iterator_name).unwrap();

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

    fn match_variable_helper(
        &mut self,
        name: &String,
        value: &Expr,
        writer: &mut BufWriter<&File>,
    ) {
        let location = self.lookup_var(name);
        
        // Evaluate expression into rax
        self.generate_expr_into_register(value, "rax", writer);
        
        match location {
            VariableLocation::Global => {
                writeln!(writer, "    mov qword [{}], rax", name).unwrap();
            }
            VariableLocation::Local(off) => {
                writeln!(writer, "    mov qword [rbp{}], rax", if off < 0 { format!("{}", off) } else { format!("+{}", off) }).unwrap();
            }
        }
    }
    
    fn lookup_var(&self, name: &str) -> VariableLocation {
        for scope in self.scopes.iter().rev() {
            if let Some(loc) = scope.get(name) {
                return loc.clone();
            }
        }
        VariableLocation::Global // fallback
    }

    fn generate_expr_into_register(
        &mut self,
        expr: &Expr,
        reg: &str,
        writer: &mut BufWriter<&File>,
    ) {
        match expr {
            Expr::Int(i) => {
                writeln!(writer, "    mov {}, {}", reg, i).unwrap();
            }
            Expr::Ident(name) => {
                match self.lookup_var(name) {
                    VariableLocation::Global => writeln!(writer, "    mov {}, qword [{}]", reg, name).unwrap(),
                    VariableLocation::Local(off) => writeln!(writer, "    mov {}, qword [rbp{}]", reg, if off < 0 { format!("{}", off) } else { format!("+{}", off) }).unwrap(),
                }
            }
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
                writeln!(writer, "    mov {}, eax", reg).unwrap();
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
                    writeln!(writer, "    mov {}, rax", reg).unwrap();
                }
            }
            Expr::ArrayLiteral(elements) => {
                let size = (elements.len() as i32) * 8;
                self.current_stack_offset -= size;
                let addr_offset = self.current_stack_offset;
                
                // Initialize elements
                for (i, elem) in elements.iter().enumerate() {
                    let elem_offset = i as i32 * 8;
                    self.generate_expr_into_register(elem, "eax", writer);
                    let final_off = addr_offset + elem_offset;
                    writeln!(writer, "    mov dword [rbp{}], eax", if final_off < 0 { format!("{}", final_off) } else { format!("+{}", final_off) }).unwrap();
                }
                
                // Return start address in reg
                writeln!(writer, "    lea rax, [rbp{}]", if addr_offset < 0 { format!("{}", addr_offset) } else { format!("+{}", addr_offset) }).unwrap();
                if reg != "rax" {
                    writeln!(writer, "    mov {}, rax", reg).unwrap();
                }
            }
            Expr::ArrayIndex { array, index } => {
                // 1. Eval index into ebx
                self.generate_expr_into_register(index, "ebx", writer);
                writeln!(writer, "    movsxd rbx, ebx").unwrap(); // sign extend index

                // 2. Eval base array into rax (address)
                match &**array {
                    Expr::Ident(name) => {
                        let loc = self.lookup_var(name);
                        match loc {
                            VariableLocation::Local(off) => {
                                // Load the pointer into rax
                                writeln!(writer, "    mov rax, qword [rbp{}]", if off < 0 { format!("{}", off) } else { format!("+{}", off) }).unwrap();
                            }
                            VariableLocation::Global => {
                                writeln!(writer, "    mov rax, qword [{}]", name).unwrap();
                            }
                        }
                    }
                    _ => {
                        // Recursive case: e.g. b[0] is the base of b[0][1]
                        self.generate_expr_into_register(array, "rax", writer);
                    }
                }
                
                // 3. Access element: [rax + rbx * 8]
                writeln!(writer, "    mov {}, qword [rax + rbx * 8]", reg).unwrap();
            }
        }
    }

    fn generate_binary_op(
        &mut self,
        left: &Expr,
        op: &BinOpType,
        right: &Expr,
        writer: &mut BufWriter<&File>,
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

    fn generate_if(
        &mut self,
        condition: &Expr,
        body: &Vec<AbstractSyntaxTreeNode>,
        else_body: &Option<Box<AbstractSyntaxTreeNode>>,
        writer: &mut BufWriter<&File>,
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
}
