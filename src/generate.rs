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

const ARRAY_HEAP_SIZE: usize = 1024 * 1024; // 1MB

pub struct Generator {
    scopes: Vec<HashMap<String, GeneratorVarEntry>>,
    functions: HashMap<String, Type>, // return types
    current_stack_offset: i32,
    global_vars: HashSet<String>,
    string_literals: HashMap<String, String>, // content, label
    string_counter: usize,
    label_counter: usize,
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
            label_counter: 0,
        }
    }

    pub fn generate_boilerplate<W: Write>(&mut self, writer: &mut W) {
        let stack_heap_size = 1024 * 1024; // 1MB
        write!(
            writer,
            "bits 64
default rel

segment .text
global mainCRTStartup
extern ExitProcess
extern puts
extern printf

mainCRTStartup:
    push rbp
    mov rbp, rsp
    
    ; Allocate 1MB on stack for array_heap
    ; Touch pages to ensure stack is committed (Stack Probe)
    mov rcx, {}
    mov rax, 4096
.probe_loop:
    sub rsp, rax
    test [rsp], rsp ; Touch the page
    sub rcx, rax
    cmp rcx, 0
    jg .probe_loop
    
    ; Save the start of our stack heap to array_ptr
    mov [array_ptr], rsp
",
            stack_heap_size
        )
        .expect("Unable to write to file.");

        // Add format strings to string_literals early
        self.string_literals.insert("%d\n".to_string(), "fmt_int".to_string());
        self.string_literals.insert("%s\n".to_string(), "fmt_str".to_string());
        self.string_literals.insert("%f\n".to_string(), "fmt_float".to_string());
        self.string_literals.insert("true\n".to_string(), "str_true".to_string());
        self.string_literals.insert("false\n".to_string(), "str_false".to_string());

        self.string_literals.insert("%d".to_string(), "fmt_int_raw".to_string());
        self.string_literals.insert("%s".to_string(), "fmt_str_raw".to_string());
        self.string_literals.insert("%f".to_string(), "fmt_float_raw".to_string());
        self.string_literals.insert("true".to_string(), "str_true_raw".to_string());
        self.string_literals.insert("false".to_string(), "str_false_raw".to_string());
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

                let _statements: Vec<&AbstractSyntaxTreeNode> = ast_root
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
                    writeln!(writer, "array_ptr resq 1").unwrap();
                } else {
                    writeln!(writer, "\nsegment .bss").unwrap();
                    writeln!(writer, "array_ptr resq 1").unwrap();
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

                // Account for spilled parameters (up to 4) which are stored at [rbp-8], [rbp-16], etc.
                let spill_count = std::cmp::min(params.len(), 4) as i32;
                total_locals_size += spill_count * 8;

                for stmt in body {
                    total_locals_size += self.calculate_stack_size(stmt);
                }

                // Ensure stack is aligned to 16 bytes.
                // total_locals_size should be multiple of 16 for calls.
                if total_locals_size % 16 != 0 {
                    total_locals_size += 16 - (total_locals_size % 16);
                }

                if total_locals_size > 0 {
                    writeln!(writer, "    sub rsp, {}", total_locals_size).unwrap();
                }

                // Windows x64 Calling Convention:
                // Params 1-4: RCX/XMM0, RDX/XMM1, R8/XMM2, R9/XMM3
                // Params 5+: stack starting at [RBP + 32 + 16] (32 shadow space + return addr + saved rbp)
                
                let mut stack_param_offset = 16 + 32; 
                for (i, (param_type, pname)) in params.iter().enumerate() {
                    let offset;
                    if i < 4 {
                        self.current_stack_offset -= 8;
                        offset = self.current_stack_offset;
                        match i {
                            0 => {
                                if matches!(param_type, Type::F32S) {
                                    writeln!(writer, "    movss dword [rbp{}], xmm0", offset).unwrap();
                                } else {
                                    writeln!(writer, "    mov qword [rbp{}], rcx", offset).unwrap();
                                }
                            }
                            1 => {
                                if matches!(param_type, Type::F32S) {
                                    writeln!(writer, "    movss dword [rbp{}], xmm1", offset).unwrap();
                                } else {
                                    writeln!(writer, "    mov qword [rbp{}], rdx", offset).unwrap();
                                }
                            }
                            2 => {
                                if matches!(param_type, Type::F32S) {
                                    writeln!(writer, "    movss dword [rbp{}], xmm2", offset).unwrap();
                                } else {
                                    writeln!(writer, "    mov qword [rbp{}], r8", offset).unwrap();
                                }
                            }
                            3 => {
                                if matches!(param_type, Type::F32S) {
                                    writeln!(writer, "    movss dword [rbp{}], xmm3", offset).unwrap();
                                } else {
                                    writeln!(writer, "    mov qword [rbp{}], r9", offset).unwrap();
                                }
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        offset = stack_param_offset as i32;
                        stack_param_offset += 8;
                    }

                    self.scopes.last_mut().unwrap().insert(
                        pname.clone(),
                        GeneratorVarEntry {
                            location: VariableLocation::Local(offset),
                            type_: param_type.clone(),
                        },
                    );
                }

                for stmt in body {
                    self.generate_x64(stmt, writer);
                }

                writeln!(writer, "    leave").unwrap();
                writeln!(writer, "    ret").unwrap();
                
                self.scopes.pop();
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFunctionCall { name, args } => {
                if name == "print" || name == "println" {
                    let is_println = name == "println";
                    for arg in args {
                        let ty = self.get_expr_type(arg);
                        match ty {
                            Type::String => {
                                self.generate_expr_into_register(arg, "rdx", writer); // rdx for printf argument
                                if is_println {
                                    // println for string: use puts which takes rcx
                                    writeln!(writer, "    mov rcx, rdx").unwrap(); 
                                    writeln!(writer, "    sub rsp, 32").unwrap();
                                    writeln!(writer, "    call puts").unwrap();
                                    writeln!(writer, "    add rsp, 32").unwrap();
                                } else {
                                    // print for string: use printf("%s")
                                    let fmt_label = self.string_literals.get("%s").unwrap().clone();
                                    writeln!(writer, "    lea rcx, [{}]", fmt_label).unwrap();
                                    writeln!(writer, "    sub rsp, 32").unwrap();
                                    writeln!(writer, "    call printf").unwrap();
                                    writeln!(writer, "    add rsp, 32").unwrap();
                                }
                            }
                            Type::Bool => {
                                self.generate_expr_into_register(arg, "rax", writer);
                                let true_key = if is_println { "true\n" } else { "true" };
                                let false_key = if is_println { "false\n" } else { "false" };

                                let true_label = self.string_literals.get(true_key).unwrap().clone();
                                let false_label = self.string_literals.get(false_key).unwrap().clone();
                                
                                let label_id = self.label_counter;
                                self.label_counter += 1;
                                let true_jump_label = format!("print_bool_true_{}", label_id);
                                let end_bool_label = format!("print_bool_end_{}", label_id);

                                writeln!(writer, "    cmp rax, 0").unwrap();
                                writeln!(writer, "    jne {}", true_jump_label).unwrap();
                                writeln!(writer, "    lea rcx, [{}]", false_label).unwrap();
                                writeln!(writer, "    jmp {}", end_bool_label).unwrap();
                                writeln!(writer, "{}:", true_jump_label).unwrap();
                                writeln!(writer, "    lea rcx, [{}]", true_label).unwrap();
                                writeln!(writer, "{}:", end_bool_label).unwrap();

                                writeln!(writer, "    sub rsp, 32").unwrap();
                                // bool strings behave like normal strings
                                if is_println {
                                    writeln!(writer, "    call puts").unwrap();
                                } else {
                                     // For 'print', we used printf for strings above, 
                                     // but here we have the address of "true" or "false" in rcx.
                                     // If "true" literal has no newline, puts adds one? No, puts always adds newline.
                                     // So if is_println=false, we MUST use printf("%s", rcx) OR define raw strings without newlines and pass them to printf?
                                     // Ah, puts takes a char* and prints it + newline.
                                     // printf("%s", str) prints str.
                                     
                                     // Here rcx holds the address of the string literal ("true" or "false" without newline)
                                     // We need to move rcx to rdx (arg 2) and load "%s" format into rcx (arg 1)
                                     writeln!(writer, "    mov rdx, rcx").unwrap();
                                     let fmt_label = self.string_literals.get("%s").unwrap().clone();
                                     writeln!(writer, "    lea rcx, [{}]", fmt_label).unwrap();
                                     writeln!(writer, "    call printf").unwrap();
                                }
                                writeln!(writer, "    add rsp, 32").unwrap();
                            }
                            Type::I32S | Type::Char => {
                                self.generate_expr_into_register(arg, "rdx", writer);
                                let fmt_key = if is_println { "%d\n" } else { "%d" };
                                let fmt_label = self.string_literals.get(fmt_key).unwrap().clone();
                                writeln!(writer, "    lea rcx, [{}]", fmt_label).unwrap();
                                writeln!(writer, "    sub rsp, 32").unwrap();
                                writeln!(writer, "    call printf").unwrap();
                                writeln!(writer, "    add rsp, 32").unwrap();
                            }
                            Type::F32S => {
                                self.generate_expr_into_register(arg, "xmm1", writer);
                                writeln!(writer, "    cvtss2sd xmm1, xmm1").unwrap(); // printf expects doubles for %f
                                writeln!(writer, "    movq rdx, xmm1").unwrap();
                                let fmt_key = if is_println { "%f\n" } else { "%f" };
                                let fmt_label = self.string_literals.get(fmt_key).unwrap().clone();
                                writeln!(writer, "    lea rcx, [{}]", fmt_label).unwrap();
                                writeln!(writer, "    sub rsp, 32").unwrap();
                                writeln!(writer, "    mov eax, 1").unwrap(); // 1 float param
                                writeln!(writer, "    call printf").unwrap();
                                writeln!(writer, "    add rsp, 32").unwrap();
                            }
                            _ => {
                                // Default fallback to puts (println behavior) or just print string?
                                // Let's treat like String
                                self.generate_expr_into_register(arg, "rdx", writer);
                                if is_println {
                                    writeln!(writer, "    mov rcx, rdx").unwrap(); 
                                    writeln!(writer, "    sub rsp, 32").unwrap();
                                    writeln!(writer, "    call puts").unwrap();
                                    writeln!(writer, "    add rsp, 32").unwrap();
                                } else {
                                    let fmt_label = self.string_literals.get("%s").unwrap().clone();
                                    writeln!(writer, "    lea rcx, [{}]", fmt_label).unwrap();
                                    writeln!(writer, "    sub rsp, 32").unwrap();
                                    writeln!(writer, "    call printf").unwrap();
                                    writeln!(writer, "    add rsp, 32").unwrap();
                                }
                            }
                        }
                    }
                } else {
                    self.generate_function_call(name, args, writer);
                }
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolReturn(opt_expr) => {
                if let Some(expr) = opt_expr {
                    let ty = self.get_expr_type(expr);
                    if matches!(ty, Type::F32S) {
                        self.generate_expr_into_register(expr, "xmm0", writer);
                    } else {
                        self.generate_expr_into_register(expr, "rax", writer);
                    }
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

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolArrayIndexAssignment {
                array,
                index,
                value,
            } => {
                // 1. Eval Value
                let value_ty = self.get_expr_type(value);
                if matches!(value_ty, Type::F32S) {
                    self.generate_expr_into_register(value, "xmm0", writer);
                    writeln!(writer, "    movd eax, xmm0").unwrap();
                } else {
                    self.generate_expr_into_register(value, "rax", writer);
                }
                
                writeln!(writer, "    push rax").unwrap();
                writeln!(writer, "    sub rsp, 8").unwrap(); // Align stack

                // 2. Eval Index
                self.generate_expr_into_register(index, "ebx", writer);
                writeln!(writer, "    movsxd rbx, ebx").unwrap();
                writeln!(writer, "    push rbx").unwrap();
                writeln!(writer, "    sub rsp, 8").unwrap(); // Align stack

                // 3. Eval Array Base
                match array {
                    Expr::Ident(name) => {
                        let loc = self.lookup_var(name).location;
                        match loc {
                            VariableLocation::Local(off) => {
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

                // 4. Restore Index
                writeln!(writer, "    add rsp, 8").unwrap();
                writeln!(writer, "    pop rbx").unwrap();
                
                // 5. Restore Value
                writeln!(writer, "    add rsp, 8").unwrap();
                writeln!(writer, "    pop rcx").unwrap();

                // 6. Store: [rax + rbx * 8] = rcx
                writeln!(writer, "    mov qword [rax + rbx * 8], rcx").unwrap();
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFor {
                iterator_name,
                iterator_begin,
                iterator_end,
                body,
            } => {
                let saved_stack_offset = self.current_stack_offset;

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

                let id = self.label_counter;
                self.label_counter += 1;

                let loop_label = format!("loop_begin_{}_{}", iterator_name, id);
                let end_label = format!("loop_end_{}_{}", iterator_name, id);

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
                writeln!(writer, "    jge {}", end_label).unwrap();

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

                self.current_stack_offset = saved_stack_offset;
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
        
        // Evaluate expression into rax/xmm0
        let ty = self.get_expr_type(value);
        if matches!(ty, Type::F32S) {
            self.generate_expr_into_register(value, "xmm0", writer);
            match location {
                VariableLocation::Global => writeln!(writer, "    movss dword [{}], xmm0", name).unwrap(),
                VariableLocation::Local(off) => writeln!(writer, "    movss dword [rbp{}], xmm0", if off < 0 { format!("{}", off) } else { format!("+{}", off) }).unwrap(),
            }
        } else {
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
            Expr::Ident(name) => {
                let entry = self.lookup_var(name);
                match entry.location {
                    VariableLocation::Global => {
                        if matches!(entry.type_, Type::F32S) {
                            if reg.starts_with('x') {
                                writeln!(writer, "    movss {}, dword [{}]", reg, name).unwrap();
                            } else {
                                let reg32 = if reg == "rax" { "eax" } else { "eax" };
                                writeln!(writer, "    mov {}, dword [{}]", reg32, name).unwrap();
                            }
                        } else {
                            if reg.starts_with('e') {
                                writeln!(writer, "    mov {}, dword [{}]", reg, name).unwrap()
                            } else {
                                writeln!(writer, "    mov {}, qword [{}]", reg, name).unwrap()
                            }
                        }
                    }
                    VariableLocation::Local(off) => {
                        let off_str = if off < 0 {
                            format!("{}", off)
                        } else {
                            format!("+{}", off)
                        };
                        
                        if matches!(entry.type_, Type::F32S) {
                            if reg.starts_with('x') {
                                writeln!(writer, "    movss {}, dword [rbp{}]", reg, off_str).unwrap()
                            } else {
                                // Load 32-bit float bits into integer register
                                let reg32 = if reg == "rax" { "eax" } else if reg == "rbx" { "ebx" } else if reg == "rcx" { "ecx" } else { "eax" };
                                writeln!(writer, "    mov {}, dword [rbp{}]", reg32, off_str).unwrap()
                            }
                        } else {
                            if reg.starts_with('x') {
                                writeln!(writer, "    movd {}, dword [rbp{}]", reg, off_str).unwrap()
                            } else if reg.starts_with('e') {
                                writeln!(writer, "    mov {}, dword [rbp{}]", reg, off_str).unwrap()
                            } else {
                                writeln!(writer, "    mov {}, qword [rbp{}]", reg, off_str).unwrap()
                            }
                        }
                    }
                }
            },
            Expr::Float(f) => {
                let bits = f.to_bits();
                if reg.starts_with('x') { // xmm register
                    writeln!(writer, "    mov eax, {}", bits).unwrap();
                    writeln!(writer, "    movd {}, eax", reg).unwrap();
                } else {
                    writeln!(writer, "    mov {}, {}", reg, bits).unwrap();
                }
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
            Expr::UnaryOp { op: _, expr } => {
                let ty = self.get_expr_type(expr);
                if matches!(ty, Type::F32S) {
                    self.generate_expr_into_register(expr, "xmm0", writer);
                    // Negate float: XOR with 0x80000000
                    writeln!(writer, "    mov eax, 0x80000000").unwrap();
                    writeln!(writer, "    movd xmm1, eax").unwrap();
                    writeln!(writer, "    xorps xmm0, xmm1").unwrap();
                    if reg != "xmm0" {
                        if reg.starts_with('x') {
                            writeln!(writer, "    movss {}, xmm0", reg).unwrap();
                        } else {
                            writeln!(writer, "    movd {}, xmm0", reg).unwrap();
                        }
                    }
                } else {
                    self.generate_expr_into_register(expr, reg, writer);
                    writeln!(writer, "    neg {}", reg).unwrap();
                }
            }
            Expr::BinaryOp { left, op, right } => {
                self.generate_binary_op(left, op, right, writer);
                if reg.starts_with('x') {
                    // Result is in xmm0
                    if reg != "xmm0" {
                        writeln!(writer, "    movss {}, xmm0", reg).unwrap();
                    }
                } else {
                    // Result is in rax/eax
                    if reg != "eax" && reg != "rax" {
                        writeln!(writer, "    mov {}, rax", reg).unwrap();
                    } else if reg == "rax" {
                        writeln!(writer, "    mov eax, eax").unwrap(); // Zero-extend eax into rax
                    }
                }
            }
            Expr::FunctionCall { name, args } => {
                self.generate_function_call(name, args, writer);
                let ret_ty = self.functions.get(name).cloned().unwrap_or(Type::I32S);
                if matches!(ret_ty, Type::F32S) {
                    // Result is in xmm0
                    if reg != "xmm0" {
                        if reg.starts_with('x') {
                            writeln!(writer, "    movss {}, xmm0", reg).unwrap();
                        } else {
                            writeln!(writer, "    movd {}, xmm0", reg).unwrap();
                        }
                    }
                } else {
                    // Result is in rax
                    if reg != "rax" {
                        if reg.starts_with('x') {
                            writeln!(writer, "    movd {}, rax", reg).unwrap();
                        } else if reg == "eax" {
                            writeln!(writer, "    mov eax, eax").unwrap();
                        } else {
                            writeln!(writer, "    mov {}, rax", reg).unwrap();
                        }
                    }
                }
            }
            Expr::ArrayLiteral(elements) => {
                let size = (elements.len() as i32) * 8;
                
                // 1. Get current array_ptr into rax
                writeln!(writer, "    mov rax, [array_ptr]").unwrap();
                writeln!(writer, "    push rax").unwrap();
                writeln!(writer, "    sub rsp, 8").unwrap(); // Align stack

                // 2. Increment array_ptr
                writeln!(writer, "    add rax, {}", size).unwrap();
                writeln!(writer, "    mov [array_ptr], rax").unwrap();

                // 3. Restore array base into rbx
                writeln!(writer, "    add rsp, 8").unwrap();
                writeln!(writer, "    pop rbx").unwrap();

                // 4. Initialize elements
                for (i, elem) in elements.iter().enumerate() {
                    let elem_offset = i as i32 * 8;
                    writeln!(writer, "    push rbx").unwrap();
                    writeln!(writer, "    sub rsp, 8").unwrap(); // Align stack
                    
                    let elem_ty = self.get_expr_type(elem);
                    if matches!(elem_ty, Type::F32S) {
                        self.generate_expr_into_register(elem, "xmm0", writer);
                        writeln!(writer, "    movd eax, xmm0").unwrap();
                    } else {
                        self.generate_expr_into_register(elem, "rax", writer);
                    }
                    
                    writeln!(writer, "    add rsp, 8").unwrap();
                    writeln!(writer, "    pop rbx").unwrap(); // Restore array base
                    writeln!(writer, "    mov qword [rbx + {}], rax", elem_offset).unwrap();
                }
                
                // Return start address in reg
                if reg != "rax" {
                    if reg.starts_with('x') {
                         writeln!(writer, "    movd {}, rbx", reg).unwrap();
                    } else {
                         writeln!(writer, "    mov {}, rbx", reg).unwrap();
                    }
                } else {
                    writeln!(writer, "    mov rax, rbx").unwrap();
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
                if reg.starts_with('x') {
                    writeln!(writer, "    movss {}, dword [rax + rbx * 8]", reg).unwrap();
                } else if reg.starts_with('e') {
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
        let ty = self.get_expr_type(left);
        if matches!(ty, Type::F32S) {
            // Evaluates left into xmm0
            self.generate_expr_into_register(left, "xmm0", writer);
            // Push xmm0 - Maintain 16-byte alignment
            writeln!(writer, "    sub rsp, 16").unwrap();
            writeln!(writer, "    movss dword [rsp], xmm0").unwrap();

            // Eval right into xmm1
            self.generate_expr_into_register(right, "xmm1", writer);

            // Restore left into xmm0
            writeln!(writer, "    movss xmm0, dword [rsp]").unwrap();
            writeln!(writer, "    add rsp, 16").unwrap();

            match op {
                BinOpType::Add => writeln!(writer, "    addss xmm0, xmm1").unwrap(),
                BinOpType::Subtract => writeln!(writer, "    subss xmm0, xmm1").unwrap(),
                BinOpType::Multiply => writeln!(writer, "    mulss xmm0, xmm1").unwrap(),
                BinOpType::Divide => writeln!(writer, "    divss xmm0, xmm1").unwrap(),
                BinOpType::LessThan => {
                    writeln!(writer, "    ucomiss xmm0, xmm1").unwrap();
                    writeln!(writer, "    setb al").unwrap();
                    writeln!(writer, "    movzx eax, al").unwrap();
                    return;
                }
                BinOpType::LessThanOrEqual => {
                    writeln!(writer, "    ucomiss xmm0, xmm1").unwrap();
                    writeln!(writer, "    setbe al").unwrap();
                    writeln!(writer, "    movzx eax, al").unwrap();
                    return;
                }
                BinOpType::GreaterThan => {
                    writeln!(writer, "    ucomiss xmm0, xmm1").unwrap();
                    writeln!(writer, "    seta al").unwrap();
                    writeln!(writer, "    movzx eax, al").unwrap();
                    return;
                }
                BinOpType::GreaterThanOrEqual => {
                    writeln!(writer, "    ucomiss xmm0, xmm1").unwrap();
                    writeln!(writer, "    setae al").unwrap();
                    writeln!(writer, "    movzx eax, al").unwrap();
                    return;
                }
                BinOpType::Equal => {
                    writeln!(writer, "    ucomiss xmm0, xmm1").unwrap();
                    writeln!(writer, "    sete al").unwrap();
                    writeln!(writer, "    movzx eax, al").unwrap();
                    return;
                }
                BinOpType::NotEqual => {
                    writeln!(writer, "    ucomiss xmm0, xmm1").unwrap();
                    writeln!(writer, "    setne al").unwrap(); // ZF=0
                    writeln!(writer, "    mov ah, al").unwrap();
                    writeln!(writer, "    setp al").unwrap();  // PF=1 (Unordered)
                    writeln!(writer, "    or al, ah").unwrap();
                    writeln!(writer, "    movzx eax, al").unwrap();
                    return;
                }
            }
            // Result is in xmm0
        } else {
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
    }

    fn generate_function_call<W: Write>(&mut self, name: &String, args: &Vec<Expr>, writer: &mut W) {
        // Windows calling convention: RCX, RDX, R8, R9
        // 1. Calculate space for args (min 32 bytes shadow space, aligned to 16)
        let mut arg_space = std::cmp::max(args.len() * 8, 32);
        if arg_space % 16 != 0 {
            arg_space += 8;
        }

        writeln!(writer, "    sub rsp, {}", arg_space).unwrap();

        // 2. Evaluate all args and store in shadow space
        for (i, arg) in args.iter().enumerate() {
            let ty = self.get_expr_type(arg);
            if matches!(ty, Type::F32S) {
                self.generate_expr_into_register(arg, "xmm0", writer);
                writeln!(writer, "    movss dword [rsp + {}], xmm0", i * 8).unwrap();
            } else {
                self.generate_expr_into_register(arg, "rax", writer);
                writeln!(writer, "    mov qword [rsp + {}], rax", i * 8).unwrap();
            }
        }

        // 3. Load first 4 into registers
        for i in 0..std::cmp::min(args.len(), 4) {
            let ty = self.get_expr_type(&args[i]);
            match i {
                0 => {
                    if matches!(ty, Type::F32S) {
                        writeln!(writer, "    movss xmm0, dword [rsp]").unwrap();
                    } else {
                        writeln!(writer, "    mov rcx, qword [rsp]").unwrap();
                    }
                }
                1 => {
                    if matches!(ty, Type::F32S) {
                        writeln!(writer, "    movss xmm1, dword [rsp + 8]").unwrap();
                    } else {
                        writeln!(writer, "    mov rdx, qword [rsp + 8]").unwrap();
                    }
                }
                2 => {
                    if matches!(ty, Type::F32S) {
                        writeln!(writer, "    movss xmm2, dword [rsp + 16]").unwrap();
                    } else {
                        writeln!(writer, "    mov r8, qword [rsp + 16]").unwrap();
                    }
                }
                3 => {
                    if matches!(ty, Type::F32S) {
                        writeln!(writer, "    movss xmm3, dword [rsp + 24]").unwrap();
                    } else {
                        writeln!(writer, "    mov r9, qword [rsp + 24]").unwrap();
                    }
                }
                _ => unreachable!(),
            }
        }
        
        let func_label = format!("func_{}", name);
        writeln!(writer, "    call {}", func_label).unwrap();
        writeln!(writer, "    add rsp, {}", arg_space).unwrap();
    }

    fn generate_if<W: Write>(
        &mut self,
        condition: &Expr,
        body: &Vec<AbstractSyntaxTreeNode>,
        else_body: &Option<Box<AbstractSyntaxTreeNode>>,
        writer: &mut W,
    ) {
        let id = self.label_counter;
        self.label_counter += 1;

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
            Expr::UnaryOp { expr, .. } => {
                size += self.calculate_expr_stack_size(expr);
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
            Expr::UnaryOp { expr, .. } => self.get_expr_type(expr),
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
