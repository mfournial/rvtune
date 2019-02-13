# Simple Intel SEAPI bindings for Rust

The goal of this library is to have a minimalistic control wrapper for using vtune with rust code to be able to:
* Create domains
* Pause or resume control
* Create tasks
  * Name
  * Begin / End
  
## Usage

### Shared library

*To remember but delete later: how to make a shared library.* See [gcc help](https://renenyffenegger.ch/notes/development/languages/C-C-plus-plus/GCC/create-libraries/index) for more infos.

Have **separate** header and `.c` files (here `test.h` and `test.c`). Compile the `.c` file with position independent addresses
```bash
gcc -c -fPIC test.c -o test.o
```
Create shared library
```bash
gcc -shared test.o -o libtest.so
```
Move the library to somewhere the linker can see it by default (or use `LD_LIBRARY_PATH`)
```bash
sudo mv libtest.so /usr/lib/
```
And link the code properly in rust (no `shared`, `dynamic`, etc.)
```rust
println!("cargo:rustc-link-lib=test");
```

### Static library

To create the static library:
```bash
gcc -c test.c -o test.o
ar rcs libtest.a test.o
```
As show in the code, can add flags in `build.rs`
```rust
println!("cargo:rustc-flags= -L intel_headers/test_headers");
println!("cargo:rustc-link-lib=static=test"); // with static
```

:arrow_right: Can include for now the compiled `libittnotify.a` file and later on we can imagine submoduling the official intel repo and building the library in the `build.rs` stage automatically.

## Limitations

This is the output of `nm libittnotify.so | grep "pause"`. Not very promising if we want to use the `pause` function (lowercase indicates hidden functions).
```
0000000000002870 t __itt_pause_init_3_0
0000000000000120 D __itt_pause_ptr__3_0
0000000000000006 T ittnotify_mp_itt_pause_
                 U __itt_pause_ptr__3_0
```

I built manually the official IntelSEAPI and after installing it I got the same problems, in detail:
```
nm /opt/intel/lib/libittnotify64.a | grep "pause"
00000000000036d0 t __itt_pause_init_3_0
0000000000000440 D __itt_pause_ptr__3_0
```
