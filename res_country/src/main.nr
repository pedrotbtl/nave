fn main(country: u8) {
    let restricted_countries = [1, 33, 44, 49];
    for i in 0..restricted_countries.len() - 1 {
        assert(country != restricted_countries[i]);
    }
    assert(country != restricted_countries[restricted_countries.len() - 1]);
    // unsafe { verify_assert(country != restricted_countries[restricted_countries.len() - 1]); }
}

// unconstrained fn verify_assert(b: bool) { assert(false); }
