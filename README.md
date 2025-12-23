
## Group18 Programming Language

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)


A minimal, educational programming language implemented in Rust that compiles to x86-64 assembly. Group18 (g18) demonstrates the complete pipeline of language implementation, from lexical analysis to code generation.

## Try It Out

Try it yourself via the online editor [here](https://blaiserettig.github.io/g18_demo.html).

## Features

- **Complete Compilation Pipeline**: Lexing → Parsing → AST Generation → x86-64 Code Generation
- **Type System**: Strongly typed with `i32s`, `f32s`, `bool`, `char`, `string`, and `void`
- **Functions**: User-defined functions with parameters and return values
- **Control Flow**: For loops and if/else conditionals
- **String Support**: String literals with escape sequences and print function
- **Cross-Platform Assembly Output**: Generates NASM-compatible x86-64 assembly
- **Comprehensive Error Handling**: Detailed error messages for failures

## Language Syntax

g18 follows a simple, C-like syntax:

```g18
fn is_less_than(char a, char b) -> bool {
    return a < b;
}

fn is_equal(char a, char b) -> bool {
    return a == b;
}

fn main() -> i32s {
    i32s x = 0;
    for i in 0 to 10 {              
        x = x + i;                  
    }

    {                               
        bool y = false;             
        f32s z = 3.14159;
    }

    i32s y = ((x + 10) * 5) / 2;    

    i32s[][] z = [[1, 2], [3, 4]];

    print("Hello, World!");

    char c = 'a';
    char d = 'b';

    if is_less_than(c, d) {                      
        return 2;
    } else if is_equal(c, d) {
        return 1;
    } else {                        
        return 0;
    }
}
```

### Grammar
```
"Entry Point"   → FunctionDec
Stmt            → Exit | VariableDec | VariableAsm | For | If | FunctionDec | FunctionCall | Return
VariableDec     → Type Ident "=" Expr ";"
VariableAsm     → (Ident | ArrayIndex) "=" Expr ";"
For             → "for" Ident "in" Int_Lit "to" Int_Lit Block
If              → "if" Expr Block Else
Else            → "else" If | "else" Block | ε
FunctionDec     → "fn" Ident "(" (Type Ident)* ")" "->" Type Block
FunctionCall    → Ident "(" Expr* ")" ";"
Block           → "{" Stmt* "}"
Type            → BaseType ("[" "]")*
BaseType        → i32s | f32s | bool | char | string | void
Ident           → *user-defined non-keyword*
Exit            → "exit" Expr ";"
Return          → "return" Expr ";"
Expr            → Equality
Equality        → Comparison (("==" | "!=") Comparison)*
Comparison      → Add (("<" | "<=" | ">" | ">=") Add)*
Add             → Mul (("+" | "-") Mul)*
Mul             → Unary (("*" | "/") Unary)*
Unary           → "-" Unary | Primary
Primary         → Int_Lit | Float_Lit | Bool_Lit | Char_lit | String_Lit | Ident | "(" Expr ")" | ArrayLiteral | ArrayIndex
ArrayLiteral    → "[" (Expr ("," Expr)*)? "]"
ArrayIndex      → Ident "[" Expr "]"
Int_Lit         → *integer literal*
Int_Lit         → *floating point literal*
Int_Lit         → *boolean point literal*
Char_Lit        → *character literal*
String_Lit      → *string literal*
```

## Architecture

### Compilation Pipeline

```
Source Code (.nbl)
       ↓
   Tokenizer (Lexer)
       ↓
   Parse Tree
       ↓
Abstract Syntax Tree
       ↓
   Code Generator
       ↓
x86-64 Assembly (.asm)
```

### Module Structure

- **`tokenize.rs`** - Lexical analysis and token generation
- **`parse.rs`** - Parsing, AST construction, and symbol table management  
- **`generate.rs`** - x86-64 assembly code generation
- **`main.rs`** - CLI interface and pipeline orchestration

## Implementation Details

### Lexer
- **Character-by-character lexing** with lookahead support

### Parser
- **Recursive descent parser** following the formal grammar
- **Two-phase approach**: Parse tree construction followed by AST generation
- **Symbol table**: Stack of HashMap-based variable tracking with type information
- **Error recovery**: Detailed error messages with token context

### Code Generator
- **x86-64 assembly generation** using NASM syntax
- **wasm generation** for web demo
- **Memory management**: Automatic `.bss` segment for variables and `.data` segment for string literals

## Getting Started

### Prerequisites
- Rust (latest stable version)
- NASM assembler (for assembling output)
- MSFT VS Linker (for creating executables)

### Installation

```bash
git clone https://github.com/blaiserettig/Group18
cd Group18
cargo build --release
```

### Usage

1. **Write a Noble program in the src/ directory** (`src/example.g18`):
```g18
fn is_identity(i32s[][] m) -> bool {
    for i in 0 to 3 {
        for j in 0 to 3 {
            if i == j {
                if m[i][j] != 1 { return false; }
            } else {
                if m[i][j] != 0 { return false; }
            }
        }
    }
    return true;
}

fn main() -> i32s {
    i32s[][] matrix = [
        [1, 0, 0],
        [0, 1, 0],
        [0, 0, 1]
    ];

    print("Matrix is identity: ");
    println(is_identity(matrix));

    return 0;
}
```

2. **Compile to assembly**:
```bash
./target/release/Group18 example.g18
```

3. **Assemble and link** (Windows):
```bash
nasm -f win64 src/out.asm -o out.obj
link out.obj "your_path_to_kernel32.lib" "your_path_to_ucrt.lib" "your_path_to_vcruntime.lib" "your_path_to_legacy_stdio_definitions.lib" "your_path_to_legacy_stdio_wide_specifiers.lib" subsystem:console /entry:mainCRTStartup /LARGEADDRESSAWARE:NO /STACK:2097152
```
4. **Run and verify** (Windows PowerShell):
```bash
./out
$LASTEXITCODE
```

## Example Compilation

**Input** (`input.g18`):
```g18
fn main() -> i32s {
    i32s[][] matrix = [
        [1, 0, 0],
        [0, 1, 0],
        [0, 0, 1]
    ];

    print(matrix[1][1] * 5);

    return 0;
}
```

**Generated Assembly** (`out.asm`):
```asm
bits 64
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
    mov rcx, 1048576
    mov rax, 4096
.probe_loop:
    sub rsp, rax
    test [rsp], rsp ; Touch the page
    sub rcx, rax
    cmp rcx, 0
    jg .probe_loop
    
    ; Save the start of our stack heap to array_ptr
    mov [array_ptr], rsp
    jmp main_entry
func_main:
    push rbp
    mov rbp, rsp
    sub rsp, 112
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, 1
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, 1
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, [array_ptr]
    push rax
    sub rsp, 8
    add rax, 24
    mov [array_ptr], rax
    add rsp, 8
    pop rbx
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 0], rax
    push rbx
    sub rsp, 8
    mov rax, 0
    add rsp, 8
    pop rbx
    mov qword [rbx + 8], rax
    push rbx
    sub rsp, 8
    mov rax, 1
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    add rsp, 8
    pop rbx
    mov qword [rbx + 16], rax
    mov rax, rbx
    mov qword [rbp-8], rax
    mov ebx, 1
    movsxd rbx, ebx
    push rbx
    mov ebx, 1
    movsxd rbx, ebx
    push rbx
    mov rax, qword [rbp-8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    pop rbx
    mov rax, qword [rax + rbx * 8]
    push rax
    mov rbx, 5
    pop rax
    imul eax, ebx
    mov rdx, rax
    lea rcx, [fmt_int_raw]
    sub rsp, 32
    call printf
    add rsp, 32
    mov rax, 0
    leave
    ret
    leave
    ret
main_entry:
    call func_main
    mov rcx, rax
    and rsp, -16
    sub rsp, 32
    call ExitProcess

segment .bss
array_ptr resq 1

segment .data
fmt_float db `%f\n`, 0
str_true db `true\n`, 0
str_false_raw db `false`, 0
fmt_char_raw db `%c`, 0
str_true_raw db `true`, 0
fmt_str_raw db `%s`, 0
fmt_float_raw db `%f`, 0
fmt_char db `%c\n`, 0
fmt_str db `%s\n`, 0
fmt_int db `%d\n`, 0
fmt_int_raw db `%d`, 0
str_false db `false\n`, 0
```

**Intermediate Steps** (Tokenization):
```tokens
Token { token_type: TokenTypeFn, value: None, line: 1, column: 1 }
Token { token_type: TokenTypeIdentifier, value: Some("main"), line: 1, column: 4 }
Token { token_type: TokenTypeLeftParen, value: None, line: 1, column: 8 }
Token { token_type: TokenTypeRightParen, value: None, line: 1, column: 9 }
Token { token_type: TokenTypeArrow, value: None, line: 1, column: 11 }
Token { token_type: TokenTypeTypeI32S, value: None, line: 1, column: 14 }
Token { token_type: TokenTypeLeftCurlyBrace, value: None, line: 1, column: 19 }
Token { token_type: TokenTypeTypeI32S, value: None, line: 2, column: 5 }
Token { token_type: TokenTypeLeftBracket, value: None, line: 2, column: 9 }
Token { token_type: TokenTypeRightBracket, value: None, line: 2, column: 10 }
Token { token_type: TokenTypeLeftBracket, value: None, line: 2, column: 11 }
Token { token_type: TokenTypeRightBracket, value: None, line: 2, column: 12 }
Token { token_type: TokenTypeIdentifier, value: Some("matrix"), line: 2, column: 14 }
Token { token_type: TokenTypeEquals, value: None, line: 2, column: 21 }
Token { token_type: TokenTypeLeftBracket, value: None, line: 2, column: 23 }
Token { token_type: TokenTypeLeftBracket, value: None, line: 3, column: 9 }
Token { token_type: TokenTypeIntegerLiteral, value: Some("1"), line: 3, column: 10 }
Token { token_type: TokenTypeComma, value: None, line: 3, column: 11 }
Token { token_type: TokenTypeIntegerLiteral, value: Some("0"), line: 3, column: 13 }
Token { token_type: TokenTypeComma, value: None, line: 3, column: 14 }
Token { token_type: TokenTypeIntegerLiteral, value: Some("0"), line: 3, column: 16 }
Token { token_type: TokenTypeRightBracket, value: None, line: 3, column: 17 }
Token { token_type: TokenTypeComma, value: None, line: 3, column: 18 }
Token { token_type: TokenTypeLeftBracket, value: None, line: 4, column: 9 }
Token { token_type: TokenTypeIntegerLiteral, value: Some("0"), line: 4, column: 10 }
Token { token_type: TokenTypeComma, value: None, line: 4, column: 11 }
Token { token_type: TokenTypeIntegerLiteral, value: Some("1"), line: 4, column: 13 }
Token { token_type: TokenTypeComma, value: None, line: 4, column: 14 }
Token { token_type: TokenTypeIntegerLiteral, value: Some("0"), line: 4, column: 16 }
Token { token_type: TokenTypeRightBracket, value: None, line: 4, column: 17 }
Token { token_type: TokenTypeComma, value: None, line: 4, column: 18 }
Token { token_type: TokenTypeLeftBracket, value: None, line: 5, column: 9 }
Token { token_type: TokenTypeIntegerLiteral, value: Some("0"), line: 5, column: 10 }
Token { token_type: TokenTypeComma, value: None, line: 5, column: 11 }
Token { token_type: TokenTypeIntegerLiteral, value: Some("0"), line: 5, column: 13 }
Token { token_type: TokenTypeComma, value: None, line: 5, column: 14 }
Token { token_type: TokenTypeIntegerLiteral, value: Some("1"), line: 5, column: 16 }
Token { token_type: TokenTypeRightBracket, value: None, line: 5, column: 17 }
Token { token_type: TokenTypeRightBracket, value: None, line: 6, column: 5 }
Token { token_type: TokenTypeSemicolon, value: None, line: 6, column: 6 }
Token { token_type: TokenTypeIdentifier, value: Some("print"), line: 8, column: 5 }
Token { token_type: TokenTypeLeftParen, value: None, line: 8, column: 10 }
Token { token_type: TokenTypeIdentifier, value: Some("matrix"), line: 8, column: 11 }
Token { token_type: TokenTypeLeftBracket, value: None, line: 8, column: 17 }
Token { token_type: TokenTypeIntegerLiteral, value: Some("1"), line: 8, column: 18 }
Token { token_type: TokenTypeRightBracket, value: None, line: 8, column: 19 }
Token { token_type: TokenTypeLeftBracket, value: None, line: 8, column: 20 }
Token { token_type: TokenTypeIntegerLiteral, value: Some("1"), line: 8, column: 21 }
Token { token_type: TokenTypeRightBracket, value: None, line: 8, column: 22 }
Token { token_type: TokenTypeMultiply, value: None, line: 8, column: 24 }
Token { token_type: TokenTypeIntegerLiteral, value: Some("5"), line: 8, column: 26 }
Token { token_type: TokenTypeRightParen, value: None, line: 8, column: 27 }
Token { token_type: TokenTypeSemicolon, value: None, line: 8, column: 28 }
Token { token_type: TokenTypeReturn, value: None, line: 10, column: 5 }
Token { token_type: TokenTypeIntegerLiteral, value: Some("0"), line: 10, column: 12 }
Token { token_type: TokenTypeSemicolon, value: None, line: 10, column: 13 }
Token { token_type: TokenTypeRightCurlyBrace, value: None, line: 11, column: 1 }value: None }
```

**Intermediate Steps** (Parsing):
```parse tree
ParseTreeSymbolNodeEntryPoint
None
    ParseTreeSymbolNodeStatement
    None
        ParseTreeSymbolNodeFunctionDec
        None
            ParseTreeSymbolTerminalFn
            None
            ParseTreeSymbolTerminalIdentifier
            Some("main")
            ParseTreeSymbolTerminalLeftParen
            None
            ParseTreeSymbolTerminalRightParen
            None
            ParseTreeSymbolTerminalArrow
            None
            ParseTreeSymbolNodeType
            None
                ParseTreeSymbolTerminalI32S
                None
            ParseTreeSymbolNodeBlock
            None
                ParseTreeSymbolTerminalLeftCurlyBrace
                None
                ParseTreeSymbolNodeStatement
                None
                    ParseTreeSymbolNodeVariableDeclaration
                    None
                        ParseTreeSymbolNodeType
                        None
                            ParseTreeSymbolNodeType
                            None
                                ParseTreeSymbolNodeType
                                None
                                    ParseTreeSymbolTerminalI32S
                                    None
                                ParseTreeSymbolTerminalLeftBracket
                                None
                                ParseTreeSymbolTerminalRightBracket
                                None
                            ParseTreeSymbolTerminalLeftBracket
                            None
                            ParseTreeSymbolTerminalRightBracket
                            None
                        ParseTreeSymbolNodeExpression
                        None
                            ParseTreeSymbolNodePrimary
                            None
                                ParseTreeSymbolTerminalIdentifier
                                Some("matrix")
                        ParseTreeSymbolTerminalEquals
                        None
                        ParseTreeSymbolNodeExpression
                        None
                            ParseTreeSymbolNodePrimary
                            None
                                ParseTreeSymbolNodeArrayLiteral
                                None
                                    ParseTreeSymbolTerminalLeftBracket
                                    None
                                    ParseTreeSymbolNodeExpression
                                    None
                                        ParseTreeSymbolNodePrimary
                                        None
                                            ParseTreeSymbolNodeArrayLiteral
                                            None
                                                ParseTreeSymbolTerminalLeftBracket
                                                None
                                                ParseTreeSymbolNodeExpression
                                                None
                                                    ParseTreeSymbolNodePrimary
                                                    None
                                                        ParseTreeSymbolTerminalIntegerLiteral
                                                        Some("1")
                                                ParseTreeSymbolTerminalComma
                                                None
                                                ParseTreeSymbolNodeExpression
                                                None
                                                    ParseTreeSymbolNodePrimary
                                                    None
                                                        ParseTreeSymbolTerminalIntegerLiteral
                                                        Some("0")
                                                ParseTreeSymbolTerminalComma
                                                None
                                                ParseTreeSymbolNodeExpression
                                                None
                                                    ParseTreeSymbolNodePrimary
                                                    None
                                                        ParseTreeSymbolTerminalIntegerLiteral
                                                        Some("0")
                                                ParseTreeSymbolTerminalRightBracket
                                                None
                                    ParseTreeSymbolTerminalComma
                                    None
                                    ParseTreeSymbolNodeExpression
                                    None
                                        ParseTreeSymbolNodePrimary
                                        None
                                            ParseTreeSymbolNodeArrayLiteral
                                            None
                                                ParseTreeSymbolTerminalLeftBracket
                                                None
                                                ParseTreeSymbolNodeExpression
                                                None
                                                    ParseTreeSymbolNodePrimary
                                                    None
                                                        ParseTreeSymbolTerminalIntegerLiteral
                                                        Some("0")
                                                ParseTreeSymbolTerminalComma
                                                None
                                                ParseTreeSymbolNodeExpression
                                                None
                                                    ParseTreeSymbolNodePrimary
                                                    None
                                                        ParseTreeSymbolTerminalIntegerLiteral
                                                        Some("1")
                                                ParseTreeSymbolTerminalComma
                                                None
                                                ParseTreeSymbolNodeExpression
                                                None
                                                    ParseTreeSymbolNodePrimary
                                                    None
                                                        ParseTreeSymbolTerminalIntegerLiteral
                                                        Some("0")
                                                ParseTreeSymbolTerminalRightBracket
                                                None
                                    ParseTreeSymbolTerminalComma
                                    None
                                    ParseTreeSymbolNodeExpression
                                    None
                                        ParseTreeSymbolNodePrimary
                                        None
                                            ParseTreeSymbolNodeArrayLiteral
                                            None
                                                ParseTreeSymbolTerminalLeftBracket
                                                None
                                                ParseTreeSymbolNodeExpression
                                                None
                                                    ParseTreeSymbolNodePrimary
                                                    None
                                                        ParseTreeSymbolTerminalIntegerLiteral
                                                        Some("0")
                                                ParseTreeSymbolTerminalComma
                                                None
                                                ParseTreeSymbolNodeExpression
                                                None
                                                    ParseTreeSymbolNodePrimary
                                                    None
                                                        ParseTreeSymbolTerminalIntegerLiteral
                                                        Some("0")
                                                ParseTreeSymbolTerminalComma
                                                None
                                                ParseTreeSymbolNodeExpression
                                                None
                                                    ParseTreeSymbolNodePrimary
                                                    None
                                                        ParseTreeSymbolTerminalIntegerLiteral
                                                        Some("1")
                                                ParseTreeSymbolTerminalRightBracket
                                                None
                                    ParseTreeSymbolTerminalRightBracket
                                    None
                        ParseTreeSymbolTerminalSemicolon
                        None
                ParseTreeSymbolNodeStatement
                None
                    ParseTreeSymbolNodeFunctionCall
                    None
                        ParseTreeSymbolTerminalIdentifier
                        Some("print")
                        ParseTreeSymbolTerminalLeftParen
                        None
                        ParseTreeSymbolNodeExpression
                        None
                            ParseTreeSymbolNodeMul
                            None
                                ParseTreeSymbolNodePrimary
                                None
                                    ParseTreeSymbolNodeArrayIndex
                                    None
                                        ParseTreeSymbolNodeArrayIndex
                                        None
                                            ParseTreeSymbolTerminalIdentifier
                                            Some("matrix")
                                            ParseTreeSymbolTerminalLeftBracket
                                            None
                                            ParseTreeSymbolNodeExpression
                                            None
                                                ParseTreeSymbolNodePrimary
                                                None
                                                    ParseTreeSymbolTerminalIntegerLiteral
                                                    Some("1")
                                            ParseTreeSymbolTerminalRightBracket
                                            None
                                        ParseTreeSymbolTerminalLeftBracket
                                        None
                                        ParseTreeSymbolNodeExpression
                                        None
                                            ParseTreeSymbolNodePrimary
                                            None
                                                ParseTreeSymbolTerminalIntegerLiteral
                                                Some("1")
                                        ParseTreeSymbolTerminalRightBracket
                                        None
                                ParseTreeSymbolTerminalStar
                                None
                                ParseTreeSymbolNodePrimary
                                None
                                    ParseTreeSymbolTerminalIntegerLiteral
                                    Some("5")
                        ParseTreeSymbolTerminalRightParen
                        None
                ParseTreeSymbolNodeStatement
                None
                    ParseTreeSymbolNodeReturn
                    None
                        ParseTreeSymbolTerminalReturn
                        None
                        ParseTreeSymbolNodeExpression
                        None
                            ParseTreeSymbolNodePrimary
                            None
                                ParseTreeSymbolTerminalIntegerLiteral
                                Some("0")
                        ParseTreeSymbolTerminalSemicolon
                        None
                ParseTreeSymbolTerminalRightCurlyBrace
                None
```

**Intermediate Steps** (Abstract Syntax Tree):
```ast
AbstractSyntaxTreeSymbolEntry
  AbstractSyntaxTreeSymbolFunctionDec { name: "main", params: [], return_type: I32S, body: [AbstractSyntaxTreeNode { symbol: AbstractSyntaxTreeSymbolVariableDeclaration { name: "matrix", type_: Array(Array(I32S)), value: ArrayLiteral([ArrayLiteral([Int(1), Int(0), Int(0)]), ArrayLiteral([Int(0), Int(1), Int(0)]), ArrayLiteral([Int(0), Int(0), Int(1)])]) }, children: [] }, AbstractSyntaxTreeNode { symbol: AbstractSyntaxTreeSymbolFunctionCall { name: "print", args: [BinaryOp { left: ArrayIndex { array: ArrayIndex { array: Ident("matrix"), index: Int(1) }, index: Int(1) }, op: Multiply, right: Int(5) }] }, children: [] }, AbstractSyntaxTreeNode { symbol: AbstractSyntaxTreeSymbolReturn(Some(Int(0))), children: [] }] }
```

## Technical Highlights

- **Memory-Safe Implementation**: Written in Rust with comprehensive error handling
- **Formal Grammar**: Well-defined BNF grammar specification
- **Parse Tree Visualization**: Debug output for understanding parsing process
- **AST Transformation**: Clean separation between concrete and abstract syntax
- **Symbol Table**: Proper variable scoping and type checking foundation
- **Modular Design**: Clean separation of concerns across compilation phases

## Roadmap

### Short Term
- [x] Assignment operator (`=`)
- [x] Symbol table refactor to allow scoping ({})
- [x] More primitive types (`f32`, `bool`, `char`)
- [x] Arithmetic expressions (`+`, `-`, `*`, `/`)
- [ ] Logical operations
- [x] Comparison operators (`==`, `!=`, `<`, `>`)

### Medium Term  
- [x] Arrays and basic data structures
- [x] String literals and manipulation
- [x] Print function for console output
- [x] Conditional statements (`if`/`else`)
- [x] Loops (`while`, `for`)

### Long Term
- [x] Functions and procedure calls
- [ ] Structs and user-defined types
- [ ] Standard library functions
- [ ] Optimization passes
- [ ] LLVM backend integration

## Educational Value

This project demonstrates:
- **Compiler theory fundamentals**
- **Rust systems programming**
- **Assembly language generation**
- **Formal language design**
- **Error handling strategies**
- **Modular software architecture**

## Contributing

Contributions are welcome! Areas of interest:
- New language features
- Optimization improvements  
- Better error messages
- Additional target architectures
- Documentation

## References

- [Crafting Interpreters](https://craftinginterpreters.com/)
- [Engineering a Compiler](https://www.elsevier.com/books/engineering-a-compiler/cooper/978-0-12-815412-0)
- [x86-64 Assembly Reference](https://www.nasm.us/xdoc/2.15.05/html/nasmdoc0.html)
- [Compiler Construction CSU Sacramento Lecture Series](https://www.youtube.com/@ghassanshobakicomputerscie9478/playlists)

## License

MIT License - see [LICENSE](LICENSE) for details.
