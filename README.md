"# Rust-Basic" 

1. install rustup

https://www.rust-lang.org/tools/install

2. rustup check
3. rustup update
4. rustc --version
5. cargo --version

6. Untuk membuat project, kita bisa gunakan perintah :
    cargo new nama-project

7. node : Di temukan Problem 

    issue "error: linking with link.exe failed: exit code: 1"


    * rustup toolchain install stable-x86_64-pc-windows-gnu

        then

    * rustup default stable-x86_64-pc-windows-gnu
        
        and

    * cargo build
  
  Compiling hello v0.1.0 (C:\Users\leke\dev\rust\hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.66s

8. cargo run

9. Untuk membuat distribution file dengan Cargo, kita bisa gunakan perintah :
    cargo build --release



