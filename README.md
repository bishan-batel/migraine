# Migraine

It's BrainF\*ck (BF) but a bit more usable

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

## Language Features

Here is a comparison of 'Hello World' between BF and Migraine (without any comments):

### Hello World in BF

```brainfuck
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```

### Hello World in Migraine

```migraine
@main {
  ^11
  "Hello World"
  [.>]
}
```

#### Functions

The first major difference between Migraine and Brainf\*ck is functions
which are defined as `@<name> { <code> }`, the terminology is 'functions' but
it may be more helpful to think about them as subroutines such as in assembly

```migraine
@printHelloWorld {
  ^3
  "Hi\0"
  [.>]&
}

@main {
  @printHelloWorld
}
```

#### Literals

By writing text such as `"Hello World"` in migraine, it treats it as a literal operator
where it will store the 32bit representation of each character into the pointer
of the current tape, similar to `memcpy` in C/C++
Example:\
`>>`\
[0, 0, <__0__>, 0 , 0]\
`"Hi!"`\
[0, 0, <__72__>, 105 , 33]\

#### The Stack Tape

In normal BF, the memory you manipulate is typically referred to as "the tape"
which is just a long line of cells with each containing a number\
In Migraine there is still a tape of memory, but migraine gives you the option
create more than just one inside a stack structure, where you can move between
them and perform operations between 2
tapes at a time.

An example of how you can manipulate the stack:
```migraine 
@main {
  ^20 // Creates a new tape in stack of 20 cells 
  "Hello World" // store 'Hello World' into new tape
  _ // Move to select the stack below (default root stack of size 0)
  ^ // Move to select the stack above (the one we created)
  & // Move to select the stack below and delete the stack we were previously on
  ^ // Move to stack above, but because we deleted the stack we would get an err
}
```
