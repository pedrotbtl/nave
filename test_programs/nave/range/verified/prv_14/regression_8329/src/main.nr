fn main(x: u1, y: u1, z: u1) -> pub u1 {
    let p = y - z;
    if p != 0 { 
        let a = x / z;
        let b = a - z;
        if b != 0 {
            let _ = a / b;
        }
    }

    let u = x - z;
    let v = y - u;
    unsafe { 
        verify_assert(((u != 0) | (x == z)) 
            & ((v != 0) | (u == y))
            & ((p != 0) | (y == x))); 
    }
    v
}

unconstrained fn verify_assert(b: bool) {}