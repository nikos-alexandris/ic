# The ic intensional compiler

## ⚠️⚠️⚠️ This is the 'types' branch (note that the build steps are slightly different) ⚠️⚠️⚠️

### Typing progress

- [X] Complete rewrite of the compiler because it is 💩 in master
- [X] Static typing and typechecking
- [X] Product types
- [ ] Sum types
- [ ] Typeclasses
- [ ] Runtime optimizations based on the type system

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
- [X] Transform the functional program to a typed intermediate representation
- [X] Transform the typed program to the equivalent intensional program
- [X] Compile the intensional program to C
- [X] Compile and link the generated C program with the runtime library

### Runtime

- [X] LARs (Only heap allocated, will change when strictness analysis is added)
- [X] Garbage Collection (Currently mark and sweep - pretty slow should be improved)

## Dependencies

- The Rust toolchain (<https://www.rust-lang.org/tools/install>)

- GCC (<https://gcc.gnu.org/install/>)

- CMake (<https://cmake.org/install/>)

- Make (<https://www.gnu.org/software/make/>)

## Running a program

### Step 1: Cloning the repository

```bash
git clone https://github.com/nikos-alexandris/ic
```

### Step 2: Setting up the environment variable

This is needed for the compiler to know where the runtime library is located. This should be set to the root of the repository you just cloned.

```bash
cd ic
export IC_HOME=$(pwd)
```

### Step 3: Building the compiler

```bash
cd $IC_HOME/compiler
cargo build --release
```

Leaves the executable `icc` in `$IC_HOME/compiler/target/release/icc`

### Step 4: Building the runtime library

```bash
cd $IC_HOME/runtime
mkdir lib
cd lib
cmake -G"Unix Makefiles" -DCMAKE_BUILD_TYPE=Release ..
make -j $(nproc)
```

Leaves the static library `libicr.a` in `$IC_HOME/runtime/lib/libicr.a`

### Step 5: Compiling the program

If the program is called `prog.fl`:

```bash
cd /path/to/program
$IC_HOME/compiler/target/release/icc prog.fl
```

Creates a `_build` subdirectory in the current directory with the generated executable `out` (and the generated C source code `out.c`)

### Step 6: Running the program

```bash
_build/out
```

There are program examples in the `examples/fl` directory.
