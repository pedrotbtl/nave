fn main() {
    let x = comptime {
        println("hello from compile-time!");
        1 + 2
    };
    println(x);

    comptime {
        let exp = [1, 2, 3];
        let quoted = quote {
            struct Foo { fn foo() { 
            // Some comment
            [ $exp ]
            } }
        };

        println(quoted);
    }
}
unconstrained fn verify_assert(b: bool) { assert(false); }
