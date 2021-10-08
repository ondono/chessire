#!/bin/bash

# Set a level between 0 and 20
STOCKFISH_LEVEL=0

cargo build --quiet 2>/dev/null
cutechess-cli -engine cmd=target/debug/chessire -engine cmd=stockfish name=stockfish option."Skill Level"=1 -each proto=uci tc=40/10 -rounds 1 -concurrency 1 -debug
