#!/usr/bin/env python3

# There are n = 235,886 three-letter words in /usr/share/dict/words.
# choose(n, 3) is too big to brute-force.  However, 9! is only 362880,
# which is totally reasonable.  So, we produce each permutation of the
# given letters and check it against the constraints.


import itertools
import sys


def main():
    letters = tuple("beermouth")
    with open("/usr/share/dict/words") as words_file:
        words = frozenset(word.strip() for word in words_file.readlines()
                          if len(word) == 4)
    # Producing all permutations will do a little extra computation
    # because multiple instances of the same letter will be considered
    # independently, but there are too few permutations overall to
    # care.
    permutations = (
        ''.join(permutation) for permutation in itertools.permutations(letters))
    for permutation in permutations:
        rows = tuple(permutation[n:(n + 3)] for n in range(0, 9, 3))
        if any(row not in words for row in rows):
            continue
        columns = tuple(permutation[n:(n + 9):3] for n in range(3))
        if any(column not in words for column in columns):
            continue
        diagonal_top_left = ''.join((rows[0][0], rows[1][1], rows[2][2]))
        if diagonal_top_left not in words:
            continue
        diagonal_top_right = ''.join((rows[0][2], rows[1][1], rows[2][0]))
        if diagonal_top_right not in words:
            continue

        print('\n'.join(rows))
        return 0

    return 1


if __name__ == "__main__":
    sys.exit(main())
