# Word Game Solver

This is a CLI for solving a pretty simple word game. The rules of the game are
such:

* The player chooses a 5 letter word without any repeated letters.
* The solver is going to attempt to guess that word.
* Every time the solver guesses a word, type in the number of letters in common
between the guess and the selected word.

That's basically it.

## Notes

`hashbrown` might not actually be faster for this case since the `SwissMap`
design deliberately sacrificed performance of deletes in favor of performance of
lookups and inserts. The majority of time in the program is spent deleting
entries from the primary hash map, so it is quite possible that change made the
program slower. However, constructing a realistic benchmark for hash maps (e.g.
realistic entropy distributions, realistic orderings for operations) is more
challenging than I'm going to do this particular Sunday afternoon. Regardless,
I wanted to give it a try.

This currently builds on Rust `1.31.0`. No expectation of forward or backward
compatibility is presumed. Dependencies are pinned to their patch versions in
the hope this will actually build in the future.