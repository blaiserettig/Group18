
## Group18 Programming Language

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)


A minimal, educational programming language implemented in Rust that compiles to x86-64 assembly. Group18 (g18) demonstrates the complete pipeline of language implementation, from lexical analysis to code generation.

## Features

- **Complete Compilation Pipeline**: Lexing → Parsing → AST Generation → x86-64 Code Generation
- **Type System**: Strongly typed with `i32s`, `f32s`, `bool`, `char`, `string`, and `void`
- **Variable Declaration and Assignment**: Store and retrieve values
- **Control Flow**: For loops and if/else conditionals
- **Functions**: User-defined functions with parameters and return values
- **String Support**: String literals with escape sequences and print function
- **Cross-Platform Assembly Output**: Generates NASM-compatible x86-64 assembly
- **Comprehensive Error Handling**: Detailed error messages for failures
- **Symbol Table Management**: Tracks variable declarations and types across multiple scopes

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
Mul             → Primary (("*" | "/") Primary)*
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

### Tokenizer
- **Character-by-character lexing** with lookahead support
- **Keyword recognition**
- **Error handling**: Graceful failure on unrecognized characters

### Parser
- **Recursive descent parser** following the formal grammar
- **Two-phase approach**: Parse tree construction followed by AST generation
- **Symbol table**: Stack of HashMap-based variable tracking with type information
- **Error recovery**: Detailed error messages with token context

### Code Generator
- **x86-64 assembly generation** using NASM syntax
- **Memory management**: Automatic `.bss` segment for variables and `.data` segment for string literals
- **Register allocation**: Strategic use of EAX register for operations
- **Boilerplate generation**: Windows-compatible entry point setup

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
i32s x = 0;
for i in 0 to 10 {
    x = i;
}
i32s y = x;
exit y;
```

2. **Compile to assembly**:
```bash
./target/release/Group18 example.g18
```

3. **Assemble and link** (Windows):
```bash
nasm -f win64 src/out.asm -o out.obj
link out.obj "your_path_to_kernel32.lib" "your_path_to_ucrt.lib" "your_path_to_vcruntime.lib" subsystem:console /entry:mainCRTStartup /LARGEADDRESSAWARE:NO
```
4. **Run and verify** (Windows PowerShell):
```bash
./out
$LASTEXITCODE
```

## Example Compilation

**Input** (`input.g18`):
```g18
i32s x = 0;
for i in 0 to 10 {
    x = i;
}
i32s y = x;
exit y;
```

**Generated Assembly** (`out.asm`):
```asm
bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov eax, dword [i]
    mov dword [x], eax
    mov eax, 0
    mov dword [i], eax
loop_begin_i:
    mov eax, dword [i]
    mov ebx, 10
    cmp eax, ebx
    jg loop_end_i
    mov eax, dword [i]
    mov dword [x], eax
    mov eax, dword [i]
    inc eax
    mov dword [i], eax
    jmp loop_begin_i
loop_end_i:
    mov eax, dword [x]
    mov dword [y], eax
    mov eax, dword [y]
    ret

segment .bss
x resd 1
y resd 1
i resd 1

```

**Intermediate Steps** (Tokenization):
```tokens
Token { token_type: TokenTypeEntryPoint, value: None }
Token { token_type: TokenTypeTypeI32S, value: None }
Token { token_type: TokenTypeIdentifier, value: Some("x") }
Token { token_type: TokenTypeEquals, value: None }
Token { token_type: TokenTypeIntegerLiteral, value: Some("0") }
Token { token_type: TokenTypeSemicolon, value: None }
Token { token_type: TokenTypeFor, value: None }
Token { token_type: TokenTypeIdentifier, value: Some("i") }
Token { token_type: TokenTypeForIn, value: None }
Token { token_type: TokenTypeIntegerLiteral, value: Some("0") }
Token { token_type: TokenTypeForTo, value: None }
Token { token_type: TokenTypeIntegerLiteral, value: Some("10") }
Token { token_type: TokenTypeLeftCurlyBrace, value: None }
Token { token_type: TokenTypeIdentifier, value: Some("x") }
Token { token_type: TokenTypeEquals, value: None }
Token { token_type: TokenTypeIdentifier, value: Some("i") }
Token { token_type: TokenTypeSemicolon, value: None }
Token { token_type: TokenTypeRightCurlyBrace, value: None }
Token { token_type: TokenTypeTypeI32S, value: None }
Token { token_type: TokenTypeIdentifier, value: Some("y") }
Token { token_type: TokenTypeEquals, value: None }
Token { token_type: TokenTypeIdentifier, value: Some("x") }
Token { token_type: TokenTypeSemicolon, value: None }
Token { token_type: TokenTypeExit, value: None }
Token { token_type: TokenTypeIdentifier, value: Some("y") }
Token { token_type: TokenTypeSemicolon, value: None }

```

**Intermediate Steps** (Parsing):
```parse tree
ParseTreeSymbolNodeEntryPoint
None
    ParseTreeSymbolNodeStatement
    None
        ParseTreeSymbolNodeVariableDeclaration
        None
            ParseTreeSymbolNodeType
            None
                ParseTreeSymbolTerminalI32S
                None
            ParseTreeSymbolNodeExpression
            None
                ParseTreeSymbolTerminalIdentifier
                Some("x")
            ParseTreeSymbolTerminalEquals
            None
            ParseTreeSymbolNodeExpression
            None
                ParseTreeSymbolTerminalIntegerLiteral
                Some("0")
            ParseTreeSymbolTerminalSemicolon
            None
    ParseTreeSymbolNodeStatement
    None
        ParseTreeSymbolNodeFor
        None
            ParseTreeSymbolTerminalFor
            None
            ParseTreeSymbolNodeExpression
            None
                ParseTreeSymbolTerminalIdentifier
                Some("i")
            ParseTreeSymbolTerminalForIn
            None
            ParseTreeSymbolNodeExpression
            None
                ParseTreeSymbolTerminalIntegerLiteral
                Some("0")
            ParseTreeSymbolTerminalForTo
            None
            ParseTreeSymbolNodeExpression
            None
                ParseTreeSymbolTerminalIntegerLiteral
                Some("10")
            ParseTreeSymbolTerminalLeftCurlyBrace
            None
            ParseTreeSymbolNodeStatement
            None
                ParseTreeSymbolNodeVariableAssignment
                None
                    ParseTreeSymbolTerminalIdentifier
                    Some("x")
                    ParseTreeSymbolTerminalEquals
                    None
                    ParseTreeSymbolNodeExpression
                    None
                        ParseTreeSymbolTerminalIdentifier
                        Some("i")
                    ParseTreeSymbolTerminalSemicolon
                    None
            ParseTreeSymbolTerminalRightCurlyBrace
            None
    ParseTreeSymbolNodeStatement
    None
        ParseTreeSymbolNodeVariableDeclaration
        None
            ParseTreeSymbolNodeType
            None
                ParseTreeSymbolTerminalI32S
                None
            ParseTreeSymbolNodeExpression
            None
                ParseTreeSymbolTerminalIdentifier
                Some("y")
            ParseTreeSymbolTerminalEquals
            None
            ParseTreeSymbolNodeExpression
            None
                ParseTreeSymbolTerminalIdentifier
                Some("x")
            ParseTreeSymbolTerminalSemicolon
            None
    ParseTreeSymbolNodeStatement
    None
        ParseTreeSymbolNodeExit
        None
            ParseTreeSymbolTerminalExit
            None
            ParseTreeSymbolNodeExpression
            None
                ParseTreeSymbolTerminalIdentifier
                Some("y")
            ParseTreeSymbolTerminalSemicolon
            None

```

**Intermediate Steps** (Abstract Syntax Tree):
```ast
AbstractSyntaxTreeSymbolEntry
  AbstractSyntaxTreeSymbolVariableDeclaration { name: "x", type_: I32S, value: Ident("i") }
  AbstractSyntaxTreeSymbolFor { iterator_name: "i", iterator_begin: Int(0), iterator_end: Int(10), body: [AbstractSyntaxTreeNode { symbol: AbstractSyntaxTreeSymbolVariableAssignment { name: "x", value: Ident("i") }, children: [] }] }
  AbstractSyntaxTreeSymbolVariableDeclaration { name: "y", type_: I32S, value: Ident("x") }
  AbstractSyntaxTreeSymbolExit(Ident("y"))
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
