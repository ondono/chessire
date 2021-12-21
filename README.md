# chessire

Chessire is a library/chess engine.

## Organization

**engine**: Engine contains the code required for move generation, since
that's one of the core parts of a chess engine. Multiple implementations
are planned:

 - bitboard: A plain bitboard implementation inspired by BBC.
 - mailbox (not implemented): 

**game**: This module contains the game state and evaluation. It also
handles fancy printing and facilities to abstract the move generation
from the rest of the library.

**bin**: Some example binaries using the library itself
