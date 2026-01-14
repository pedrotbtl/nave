# Formally verifying Rock-Paper-Scissors (RPS) in NAVE

## Why RPS in Zero Knowledge?

In a zero-knowledge setting, we want to prove that:

“The game was played correctly according to the rules without revealing the players’ moves.”

Taking inspiration from the blog: https://dev.to/spalladino/a-beginners-intro-to-coding-zero-knowledge-proofs-c56,
[src/main](src/main.nr) contains a simplified version of the program.

RPS is a perfect minimal example because:

1. Inputs are private.
2. Rules are simple but non-trivial.

## Why This Is Formally Verifiable

In ZK terms, the RPS circuit is verifiable because:

1. Every rule can be modelled as a constraint.
2. Every branch is deterministic
3. No invalid states are reachable

## ZK Verification Model

We have 2 private witnesses:

1. Player X’s move -- denoted by `x`
2. Player Y’s move -- denoted by `y`

We can assert that:

1. Each move is valid.
2. The outcome follows RPS rules.

The final outcome is expressed as a public Field value.

## Modeling Moves in Noir

In Noir, we encode the move enum as field elements.

```
// Encoding:
// 0 = Rock
// 1 = Paper
// 2 = Scissors
```

We have explicitly constrained the input moves to be valid in Noir.

```
assert((x == 0) | (x == 1) | (x == 2));
assert((y == 0) | (y == 1) | (y == 2));
```

Though there is a relatively easy way to constrain this via modelling the parameters `x` and `y` as `u8` (highlighted below) but it generates `BLACKBOX::RANGE` opcode in ACIR which is hard to evaluate.

```
assert((x as u8) <= 2);
assert((y as u8) <= 2);
```

## Outcome Logic

We encode the RPS rules arithmetically, and calculate the score using the function below.

```
// Outcome or score encoding
// 0 = Player X wins
// 3 = Tie
// 6 = Player Y wins
```

We calculate the score from the moves using the following logic:

```
fn compute_diff(x: Field, y: Field) -> Field {
    let diffYX = (y + 3 - x);

    // diffYX == 0 -> tie
    // diffYX == 1 or 4 -> Y wins
    // diffYX == 2 or 5 -> X wins

    diffYX
}
```

## Formal Verification Conditions (Explicit) and Analysis

The following constraints formally specify and verify the correctness of the result value for a two-player game.

The circuit uses `verify_assert` statements to encode logical properties that must hold for all possible inputs. A verification result of `Verified` means that no counterexample exists, i.e., the negation of the asserted properties is unsatisfiable (UNSAT).

In this section, we only focus on the winning conditions for player X.

Check 1: Valid result values

```
unsafe { verify_assert((result == 0) | (result == 3) | (result == 6)); }
```
Formally, this ensures that the circuit cannot produce an invalid game result.

Check 2: Player X winning conditions

```
// If X wins, the result must be 0
unsafe { verify_assert(!(
    ((x == 0) & (y == 2)) |
    ((x == 1) & (y == 0)) |
    ((x == 2) & (y == 1))) |
    (result == 0)
); }

// If the result is 0, then X must have won
unsafe { verify_assert((
    ((x == 0) & (y == 2)) |
    ((x == 1) & (y == 0)) |
    ((x == 2) & (y == 1))) |
    !(result == 0)
); }

```

The first `verify_assert` statement encodes the implication:

```
X wins => result == 0
```

It states that whenever the inputs correspond to a winning position for player X, the output result must be 0.

The second one encodes the converse implication:

```
result == 0 => X wins
```

It ensures that the circuit cannot claim player X has won unless the inputs actually correspond to one of X’s winning configurations.

Similar verification conditions have been added to handle cases where Player Y can win and where the round can result in a tie.
All conditions return `Verified` as the result.

## Extensions

This verification design can be extended to:

1. Multi-round games
2. Score tracking

## How to run the verifier?

To run the verifier, from `rps` directory, use the following command:

`cargo run -- formal-verify`

We can also specify configuration options to it -- more details in `tooling/acir_checker/README.md`
