# LLVMKaleidoscope

LLVM's Kaleidoscope tutorial but in Rust.

Why do I make life harder for myself?

**[Bugs](/BUGS.md)**

Run `make help` for more crate-specific build options.

# Building on Windows

`LLVMKaleidoscope` uses crates
(such as [inkwell](https://crates.io/crates/inkwell))
for LLVM IR generation and compilation. These dependencies ultimately require
an installation of LLVM on your system. However, the installer for Windows
provided on the
[LLVM release page](https://github.com/llvm/llvm-project/releases)
is notably missing a couple important files and binaries, including
`llvm-config.exe` and pretty much all of the header files in the `include`
directory. Therefore, I had to build LLVM from source, which is not a very
fun thing to do when your laptop falls woefully short of LLVM's hardware
requirements. If you need a working copy of LLVM built using Visual Studio
2019, you can find a .7z file [here]().
