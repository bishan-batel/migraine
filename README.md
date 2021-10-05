# Migraine

It's BrainF*ck (BF) but a bit more usable

Now with hot new features like:

- Functions
- Number and String Litearls
- The Stack (new architecture)

## Work in Progress

- [x] Preprocessor (for macros)
	- [x] Macros
	- [ ] Include statements
- [x] Lexer / Tokenizer
- [ ] Parser / Parsetree generation
- [ ] Intepreter
- [ ] Compiler
	- [ ] Linux x86_64 Architecture
	- [ ] Win x86_64 Architecture
	- [ ] Mac x86_64 Architecture

# Language Features
Here is a comparison of 'Hello World' between BF and Migraine (without any comments):


## Hello World in BF
```brainfuck
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```

## Hello World in Migraine

```brainfuck
@main {
  ^11
	"Hello World"
	[.>]
}
```

The first major difference between Migraine and Brainf*ck is 
