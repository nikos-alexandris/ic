# The ic intensional compiler

## Process

The process of compilation is:

1. Parse the functional program
2. Transform the functional program to a high level intermediate representation
3. Compile the HIR to the intensional program
4. Compile the intensional program to C
5. Compile the C program to an executable and link it with the runtime library

## Roadmap

### Compiler

- [X] Parse functional programs as shown in `prog.fl`
- [X] Peform error checking on the functional program (undefined variables, wrong argument arities, definition of nullary variable `result`, etc.)
- [X] Handle function local arguments in the functional source program
- [X] Transform the functional program to a high level intermediate representation
- [X] Transform the HIR to the equivalent intensional program
- [X] Compile the intensional program to C
- [X] Don't require the user to compile and link the generated code; do it automatically

### Runtime

- [X] World modeling and operations
- [X] Values
- [ ] Garbage Collection
- [ ] LAR

## Dependencies

- The Rust toolchain (<https://www.rust-lang.org/tools/install>)

- GCC (<https://gcc.gnu.org/install/>)

- CMake (<https://cmake.org/install/>)

- Make (<https://www.gnu.org/software/make/>)

## Running a program

### Step 1: Building the compiler

```bash
cd compiler
cargo build --release
```

Leaves the executable `ic` in `compiler/target/release/ic`

### Step 2: Building the runtime library

```bash
cd runtime
mkdir lib
cd lib
cmake -G"Unix Makefiles" -DCMAKE_BUILD_TYPE=Release ..
make -j $(nproc)
```

Leaves the static library `libic.a` in `runtime/lib/libic.a`

### Step 3: Setting up the environment variable

```bash
export IC_HOME=/path/to/ic
```

### Step 4: Compiling a program

If the program is called `prog.fl`:

```bash
cd /path/to/program
/path/to/compiler/target/release/ic prog.fl
```

Creates a `_build` subdirectory in the current directory with the generated executable `out` (and the generated C source code `out.c`)

### Step 4: Running the program

```bash
_build/out
```
