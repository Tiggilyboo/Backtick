# Backtick
## An Extended Brainfuck language transpiling to Brainfuck using bfc

### Extensions
    @{number|label}         : Put the pointer at the specified position (This means we need to know the current position...)
    >{number}               : Move right {number} positions
    ={number}               : Set the value of the current position to 0
    ~                       : Break out of the current loop or exit the program
    ^{label}                : Label the current position with an idenfitier, can be used later with @name f.e.
    !`{language}`           : Declare a function with enclosed backtick expression at the current position
    ^name!`{number|alpha}`  : Declare a function with byte valued contents at the current position (This overwrites your cells)
    =`{number|alpha}`       : Set the current position to the current + length memory to the contained backtick expression

### Transpilation
  Converts Braintick into BF, then compiled using [bfc](https://github.com/Wilfred/bfc)
  
### Why?
  I wanted to learn how to write a parser as well and learn more rustlang. I am planning on extending this for writing some
  instruction compression for procededurally generating programs - Where a program is modeled (Based on the instructions 
  created in Backtick), by using genetic algorithms, find one or more seeds which generate the instructions based off of XOR 
  shift or something... More on this to come should the project flesh out more in the future.
