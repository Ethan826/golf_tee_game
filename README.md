# Golf Tee Game

Solver for

![golf tee game image](http://i.imgur.com/5eCYvIy.jpg)

## Usage

Clone the repo.

```bash
git clone https://github.com/Ethan826/golf_tee_game.git
```

Install Rust.

```bash
$> curl https://sh.rustup.rs -sSf | sh
```

Compile and run.

```bash
$> cd golf_tee_game
$> cargo run --release -- --help
```

There are two subcommands: `count` and `solutions`. The first (required)
argument is the space to leave open at the beginning of the game. The next
argument is the subcommand.

```bash
$> cargo run --release -- 3 count
Starting from position 3, there are 85258 solutions.
```

```bash
$> cargo run --release -- 7 solutions
Begin solution 1 of 1550
=============================

Jump tee at position 2 over position 4 to position 7
Jump tee at position 9 over position 5 to position 2
...
```

## Interesting things I’ve learned

There are the most solutions are for this kind of game (starting with an open
space at the middle of an outer row), 85,258:

        x
       x x
      . x x
     x x x x
    x x x x x

Given this numbering pattern, in descending order of number of
solutions (based on which spot we leave open at the beginning):

            00
          01  02
        03  04  05
      06  07  08  09
    10  11  12  13  14

- Middle of outside row (3, 5, 12): 85,258 solutions.
- Vertex of the board (0, 10, 14): 29,760 solutions.
- One in from the vertex (1, 2, 6, 9, 11, 13): 14,880 solutions.
- Interior of the board (4, 7, 8): 1,550 solutions.

If all you want is a single usable solution, here’s one chosen
arbitrarily. Start with spot 0 empty, then

1.  Jump tee at position 5 over position 2 to position 0
2.  Jump tee at position 3 over position 4 to position 5
3.  Jump tee at position 10 over position 6 to position 3
4.  Jump tee at position 9 over position 5 to position 2
5.  Jump tee at position 13 over position 8 to position 4
6.  Jump tee at position 1 over position 3 to position 6
7.  Jump tee at position 11 over position 12 to position 13
8.  Jump tee at position 0 over position 2 to position 5
9.  Jump tee at position 5 over position 4 to position 3
10. Jump tee at position 14 over position 13 to position 12
11. Jump tee at position 6 over position 3 to position 1
12. Jump tee at position 12 over position 7 to position 3
13. Jump tee at position 3 over position 1 to position 0

## Design

A `Game` contains a `GameState` (which wraps a `Vec` of booleans to
represent the state of each position on the board) as well as a
`LegalMoves` struct, which contains a `Vec` of the same size that
defines the rules of that game. Each of these is validated to assure
that it contains a triangular number of positions and are the state and legal
moves structures are the size as one another.

The positions on the board are the indices in each of those structures.
A board is numbered like the diagram above (although it can be any
triangular number in size).

A `GameMove` defines a single move, and is of the form

```rust
pub struct GameMove {
    pub starting_space: usize,
    pub leapt_space: usize,
    pub destination_space: usize,
}
```

`GameMove`s are used to describe the `LegalMove`s in a game, as an
argument to `Game::make_move()` to execute a move, and as the history of
moves made in a particular game.

The `Game::solve()` method consumes the `Game` and returns a
`Result<HashSet<Vec<GameMove>>, GameError>`, defining all unique
solutions to that game. It is a simple breadth-first search
implementation.

## TODO

If I continue to work on this, I will optimize by memoizing the
traversals performed so far, then detecting symmetries and returning the
memoized solution (which can be transformed according to the symmetry).
In other words, a game with all positions but `0` filled is the same as
a game with all positions but `10` filled, and so a solution for the
former will suffice for the latter once the game is rotated 90°. For
memoizing to be meaningful, I’ll probably have to switch to a
depth-first search.
