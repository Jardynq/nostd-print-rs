# nostd-print
Minimal #[no_std] implementation of `print!` `println!` `eprint!` and `eprintln!` on Windows and Unix systems. Includes `read_to_buf` for stdin with a known size buffer, or if an allocator is available,  `read_to_string` gated behind the alloc feature.
