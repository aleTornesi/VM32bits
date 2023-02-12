# VM32bits

This project is a Virtual Machine that emulates the MIPS I architecture

To start run `cargo run`

## Table of contents

- [Sources](#sources)
- [Instructions formats](#instruction-formats)
    - [R instructions](#r-format) 
    - [I instructions](#i-format) 
    - [J instructions](#j-format) 
- [Instructions](#instructions)
    - [R instructions](#r-instructions)
        - [Addi](#addi-add-immediate)
        - [Addui](#addui-add-unsigned-immediate])
        - [Subi](#subi-add-immediate)
        - [Subui](#subui-add-unsigned-immediate])
        - [Muli](#muli-add-immediate)
        - [Mului](#mului-add-unsigned-immediate])
        - [Divi](#divi-add-immediate)
        - [Divui](#divui-add-unsigned-immediate])
    - [I instructions](#i-instructions)
    - [J instructions](#j-instructions)


## Sources

The source while developing this project have been:
- The course in Computer's Architecture I and Computer's Architecture II that I attended at the University of Milan
- [This document from the Ca' Foscari University of Venice](https://www.dsi.unive.it/~gasparetto/materials/MIPS_Instruction_Set.pdf)
- [This more detailed document from Carnegie Mellon University](https://www.cs.cmu.edu/afs/cs/academic/class/15740-f97/public/doc/mips-isa.pdf)
- [Another very detailed document from Cornell University](https://www.cs.cornell.edu/courses/cs3410/2008fa/MIPS_Vol2.pdf)

## Instruction formats

It implements mainly 3 formats of instructions

### R format

Those instructions perform arithmetic operations on the registers

It has 6 fields:

- OPCode: 6 bits - Used in all formats, identifies the instruction, for all R instructions is always 0o00
- <i>r</i><sub>s</sub>: 5 bits - Identifies the register that contains the first operand
- <i>r</i><sub>t</sub>: 5 bits - Identifies the register that contains the second operand
- <i>r</i><sub>d</sub>: 5 bits - Identifies the register in which the result will be stored
- Shift amount: 5 bits - A immediate value that determines the shift to perform after the calculation
- Function: 6 bits - An identifier for the single R instruction

![R instructions format visual representation](mdImgs/r-instructions.png "R instructions format")

### I format

Those instructions perform different kinds of operations but all use a 16 bit immediate value

It has n fields:

- OPCode: 6 bits - Used in all formats, identifies the instruction
- <i>r</i><sub>s</sub>: 5 bits - Identifies a register
- <i>r</i><sub>t</sub>: 5 bits - Identifies a register
- Immediate: 16 bits - The immediate value that the instruction utilizes

The use of <i>r</i><sub>s</sub> and r<sub>t</sub> vary from instruction to instruction, so their use will be explained in detailed in each instruction section

![I instructions format visual representation](mdImgs/i-instructions.png "I instructions format")

### J format

Those instructions perform unconditional jumps

It has 2 fields:
- OPCode: 6 bits - Used in all formats, identifies the instruction
- Pseudo-address: 26 bits - The value from which the address of jump will be built

![J instructions format visual representation](mdImgs/j-instructions.png "J instructions format")

## Instructions

## R Instructions

<br>

## I instructions

### Addi (Add immediate)

This instruction performs a sum between an integer contained in the r<sub>s</sub> and the immediate, both encoded in 2's complement

**Op Code**: 0o03

### Addui (Add unsigned immediate)

This instruction performs a sum between an integer contained in the r<sub>s</sub> and the immediate, both interpreted as unsigned integers

**Op Code**: 0o04

### Subi (Subtract immediate)

This instruction performs a subtraction between an integer contained in the r<sub>s</sub> and the immediate, both encoded in 2's complement

**Op Code**: 0o05

### Subui (Subtract unsigned immediate)

This instruction performs a subtraction between an integer contained in the r<sub>s</sub> and the immediate, both interpreted as unsigned integers

**Op Code**: 0o06

### Muli (Multiply immediate)

This instruction performs a multiplication between an integer contained in the r<sub>s</sub> and the immediate, both encoded in 2's complement

**Op Code**: 0o07
### Mului (Multiply unsigned immediate)

This instruction performs a multiplication between an integer contained in the r<sub>s</sub> and the immediate, both encoded in 2's complement

****Op Code****: 0o010

### Divi

This instruction performs a integer division between an integer contained in the r<sub>s</sub> and the immediate, both encoded in 2's complement

**Op Code**: 0o011

### Divui

This instruction performs a integer division between an integer contained in the r<sub>s</sub> and the immediate, both encoded in 2's complement

**Op Code**: 0o012

<br>