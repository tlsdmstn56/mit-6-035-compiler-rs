# MIT 6-035 compiler

[MIT 6-035 Computer Language Engineering](https://ocw.mit.edu/courses/6-035-computer-language-engineering-spring-2010/)

## Components

1. Scanner and Parser (Front End)
    - Scanner: splits source file input to tokens
        - Tokens can be operator, keyword, literal, string, or identifier.
        - Non-tokens such as white spaces are discarded in this phase.
        - Malformed tokens are reported and aborts compilation process.
    - Parser: reads tokens and check if it conforms to the language spec.
        - matching braces
	- semicolons
	- Not verified: type, function/variable name
	- Outputs a kind of tree structure (not AST)

2. Semantic Checker (Front End)
    - checks various non-context-free constraints: e.g. type compatibility
    - builds symbol table that keeps user-defined types and location of each identifier
    - Outputs IR

3. Code Generation (Back-end)
    - generate _unoptimized_ x86-64 assembly 
    - Object code conforming to ABI (Application Binary Interface)

4. Data Flow Analysis (Back-end)
    - optimization pass


5. Optimizer (Back-end)
    - multiple data flow optimization pass

## CLI

```bash
./mit-6-035-compiler [option | filename ...]

# -o <outname>
# -target [scan|parse|inter|assembly]
# -opt [optimizations...] : prefix of - will exclude the optimization
# -debug : should not print anything if compilation is successful
```
    


