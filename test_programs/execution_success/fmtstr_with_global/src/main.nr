global FOO: Field = 1;

fn main() {
    println(f"foo = {FOO}");
}
unconstrained fn verify_assert(b: bool) { assert(false); }
