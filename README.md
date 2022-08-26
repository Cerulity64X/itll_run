# It'll Run
An interpreted, stack-based, byte-based scripting language that will not error.
Made for fun, just to see if I can make something that doesn't error.
Will require a hex editor or compiler (which I might make soon).

# No Errors
Indexing out of bounds will modulate or return 0 (based on which instruction you use) instead of an error.
A sudden EOF (for example, when operating on an i32 and there are only 2 bytes left) will return a 0.
There are 256 instructions, so no syntactical errors as all bytes have their own instructions.

There are, however, errors *before* interpretation, such as when no file was inputted. Since these are not interpretation errors, these do not count.

# Instructions
0: Stop the program.
1: Push an i32 onto the stack.
2: Pop the stack.
3: Push pop + pop.
4: Push pop - pop.
5: Push pop * pop.
6: Push pop / pop (integer division).
7: Compares pop to pop and pushes the result (less = -1, equal = 0, greater = 1).
8: Jumps to a position.
9: Jumps to a position if pop < 0.
10: Jumps to a position if pop = 0.
11: Jumps to a position if pop > 0.

# WIP
