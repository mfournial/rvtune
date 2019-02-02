# Simple VTune control for Rust

The goal of this library is to have a minimalistic control wrapper for using vtune with rust code to be able to:
* Create domains
* Pause or resume control
* Create tasks
  * Name
  * Begin / End
  
## Usage

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
```
sudo mv libtest.so /usr/lib/
```
And link the code properly in rust (no `shared`, `dynamic`, etc.)
```rust
println!("cargo:rustc-link-lib=test");
```
