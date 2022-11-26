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
- [ ] Peform error checking on the functional program (undefined variables, wrong argument arities, definition of nullary variable `result`, etc.)
- [X] Handle function local arguments in the functional source program
- [X] Transform the functional program to the equivalent intensional program
- [ ] Compile the intensional program to C

### Runtime

- [X] World modeling and operations
- [X] Values
- [ ] Garbage Collection
- [ ] LAR

## Dependencies

- The Rust toolchain (https://www.rust-lang.org/tools/install)

- GCC (https://gcc.gnu.org/install/)

## Running a program

### Step 1: Building the compiler

```bash
cd compiler
cargo build --release
```

Leaves the executable `ic` in `compiler/target/release/ic`

### Step 2: Building the runtime library

TODO

### Step 3: Compiling a program

```bash
ic prog.fl
```

### Step 4: Running the program

TODO
