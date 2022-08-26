# It'll Run
An interpreted, stack-based, byte-based scripting language that will not error.
Made for fun, just to see if I can make something that doesn't error.
Will require a hex editor or compiler (which I might make soon).

# No Errors
Indexing out of bounds will modulate or return 0 (based on which instruction you use) instead of an error.
A sudden EOF (for example, when operating on an i32 and there are only 2 bytes left) will return a 0.
There are 256 instructions, so no syntactical errors as all bytes have their own instructions.

# Instructions
0: Stop the program.

# WIP
