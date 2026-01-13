fn round(x: Field, y: Field) -> Field {
    assert((x == 0) | (x == 1) | (x == 2));
    assert((y == 0) | (y == 1) | (y == 2));

    let mut score = 0;

    if x == y {
        score = 3;
    }

    let diffYX = y + 3 - x;
    if (diffYX == 4) | (diffYX == 1) {
        score = 6;
    }
    score
}

fn main(x: Field, y: Field) -> pub Field {
    let result = round(x, y);

    unsafe { verify_assert((result == 0) | (result == 3) | (result == 6)); }

    // Verification of player X winning conditions
    // If X wins, the result must be 0
    unsafe { verify_assert(!(((x == 0) & (y == 2)) | ((x == 1) & (y == 0)) | ((x == 2) & (y == 1))) | (result == 0)); }
    // If the result is 0, then X must have won
    unsafe { verify_assert((((x == 0) & (y == 2)) | ((x == 1) & (y == 0)) | ((x == 2) & (y == 1))) | !(result == 0)); }

    // Verification of player Y winning conditions
    // If Y wins, the result must be 6
    unsafe { verify_assert(!(((x == 2) & (y == 0)) | ((x == 0) & (y == 1)) | ((x == 1) & (y == 2))) | (result == 6)); }
    // If the result is 6, then Y must have won
    unsafe { verify_assert((((x == 2) & (y == 0)) | ((x == 0) & (y == 1)) | ((x == 1) & (y == 2))) | !(result == 6)); }

    // Verification of tie conditions
    // If there is a tie, the result must be 3
    unsafe { verify_assert(!(((x == 0) & (y == 0)) | ((x == 1) & (y == 1)) | ((x == 2) & (y == 2))) | (result == 3)); }
    // If the result is 3, then it must be a tie
    unsafe { verify_assert((((x == 0) & (y == 0)) | ((x == 1) & (y == 1)) | ((x == 2) & (y == 2))) | !(result == 3)); }

    result
}

unconstrained fn verify_assert(b: bool) { assert(false); }