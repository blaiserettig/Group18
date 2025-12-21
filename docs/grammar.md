# Noble Formal Grammar
***
```
"Entry Point"   → Stmt*
Stmt            → Exit | VariableDec | VariableAsm | For | If | FunctionDec | FunctionCall
VariableDec     → Type Ident "=" Expr ";"
VariableAsm     → Ident "=" Expr ";"
For             → "for" Ident "in" Int_Lit "to" Int_Lit Block
If              → "if" Expr Block Else
Else            → "else" If | "else" Block | ε
FunctionDec     → "fn" Ident "(" Expr* ")" "->" Type Block
FunctionCall    → Ident "(" Expr* ")" ";"
Block           → "{" Stmt* "}"
Type            → i32s | f32s | bool | char | void
Ident           → *user-defined non-keyword*
Exit            → "exit" Expr ";"
Expr            → Equality
Equality        → Comparison (("==" | "!=") Comparison)*
Comparison      → Add (("<" | "<=" | ">" | ">=") Add)*
Add             → Mul (("+" | "-") Mul)*
Mul             → Primary (("*" | "/") Primary)*
Primary         → Int_Lit | Float_Lit | Bool_Lit | Char_lit | Ident | "(" Expr ")"
Int_Lit         → *integer literal*
Int_Lit         → *floating point literal*
Int_Lit         → *boolean point literal*
Char_Lit        → *character literal*
```