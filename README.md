# The ic intensional compiler

## Process

The process of compilation is:

1. Parse the functional program
2. Transform the functional program to the equivalent intensional one
3. Compile the intensional program to C
4. Compile the C program to an executable and link it with the runtime library

## Roadmap

### Compiler

- [X] Parse functional programs as shown in `compiler/test.fl`
- [X] Transform the functional program to the equivalent intensional program
- [ ] Compile the intensional program to C
- [ ] Test that the source programs includes the nullary variable `result`
- [X] Handle function local arguments in the functional source program

### Runtime

- [X] World modeling and operations
- [X] Values
- [ ] Garbage Collection
- [ ] LAR
