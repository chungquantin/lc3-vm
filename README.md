# Little Computer 3 (LC3) Virtual Machine (by @lowlevelers)
## What is Little Computer 3?
Little Computer 3, or LC-3, is a type of computer educational programming language, an assembly language, which is a type of low-level programming language.

It features a relatively simple instruction set but can be used to write moderately complex assembly programs, and is a viable target for a C compiler. The language is less complex than x86 assembly but has many features similar to those in more complex languages.
## Experiment
This project is a Little Computer 3 virtual machine written in Rust programming language. It follows the online guide from [Write your Own Machine](https://www.jmeiners.com/lc3-vm/) by Justin Meiners and Ryan Pendleton. The virtual machine components are organized as below:

| Filename    | Description |
| -------- | ------- |
| main.rs  | Program entry point for image loader and simple CLI implementation  |
| cpu.rs | Implementation of a virtual CPU or virtual machine     |
| instruction.rs    | Declaration of the enumeration of instructions    |
| trap.rs    | Declaration of the enumeration of trap routine    |
| register.rs    | Registers and conditional flags    |
| constant.rs    | Constants    |


## Reference 
- [LC3 instruction set architecture (ISA)](https://www.jmeiners.com/lc3-vm/supplies/lc3-isa.pdf)
- [lc3-vm in C++](https://github.com/justinmeiners/lc3-vm/)
- [LC-3-Rust: LC3 virtual machine written in Rust](https://github.dev/digorithm/LC-3-Rust/)
- [Lecture from University of Texas](https://www.cs.utexas.edu/users/fussell/courses/cs310h/lectures/Lecture_10-310h.pdf)
