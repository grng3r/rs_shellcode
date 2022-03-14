#

## Executing code from memory in Rust on Linux

1. Embedding shellcode in .text section
2. Using mmap crate and setting a memory mapped area as executable
3. Using Linux mprotect function

## exec

- executes shellcode.bin which every inidividual program should compile to
- it embeddes the shellcode in .text section

## mmap_exec

- sets a memory mapped area as executable

## shellcode.ld

- instructs linker what shape the binaries are going to have


## shell

- shellcode that executes a shell


## TODO 

- expore different arch
- ouyher shellcode
