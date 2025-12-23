Group18 is a programming language and compiler. It is designed to be a simple, yet powerful language that can be used to create a wide range of applications.

src/ contains the lexer, parser, and compiler.
test/ contains the test files.
pkg/ contains the wasm generated files for a web portfolio demo of the compiler.

generate_wasm.rs generates wasm instead of x64 asm to showcase on my portfolio in a web demo. generate.rs is to generate x64 asm, and to my knowledge contains no bugs. generate_wasm.rs is a work in progress and contains several bugs.