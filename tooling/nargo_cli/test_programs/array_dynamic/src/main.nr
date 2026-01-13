fn main(x : [u8; 4], y : u8) {
    // Safety: for verification purposes
    // assert(x[0] == 0); 
    let z = (x[0] & 0) as Field;
    // assert(false);
    unsafe { verify_assert(z == 0); }
    unsafe { verify_assert(z == 1); }
    let z = ( (z+1) as u8 & 1) as Field;
    unsafe { verify_assert(z == 1); }
    unsafe { verify_assert(z == 2); }
    let z = ( (z+1) as u8 & 2) as Field;
    unsafe { verify_assert(z == 2); }
    unsafe { verify_assert(z == 0); }
}

unconstrained fn verify_assert(exp: bool){
    keep()
}

#[oracle(keep)]
unconstrained fn keep(){}

// fn main(
//     x: [u32; 5],
//     mut z: u32,
//     t: u32,
//     index: [u32; 5],
//     index2: [u32; 5],
//     offset: u32,
//     sublen: Field,
// ) {
//     let idx = (z - 5 * t - 5);
//     //dynamic array test
//     dyn_array(x, idx, idx - 3);
//     //regression for issue 1283
//     let mut s = 0;
//     let x3 = [246, 159, 32, 176, 8];
//     for i in 0..5 {
//         s += x3[index[i]];
//     }
//     assert(s != 0);

//     if 3 < (sublen as u32) {
//         assert(index[offset + 3] == index2[3]);
//     }
// }

// fn dyn_array(mut x: [u32; 5], y: u32, z: u32) {
//     assert(x[y] == 111);
//     assert(x[z] == 101);
//     x[z] = 0;
//     assert(x[y] == 111);
//     assert(x[1] == 0);
//     if y as u32 < 10 {
//         x[y] = x[y] - 2;
//     } else {
//         x[y] = 0;
//     }
//     unsafe { verify_assert(x[4] == 109); }
// }

// fn main(x : Field) {
//     assert(x * (x - 1) == 0);
//     unsafe { verify_assert((x == 0) | (x == 1)); }
// }

// fn main(x: Field) -> pub Field {
//     let mut invx = 0;
//     unsafe {
//         invx = invu(x);
//     }
//     assert(invx * x == 1);
//     invx
// }

// fn main(x: Field) -> pub Field {
//     let y = inv(x);
//     unsafe { verify_assert(x * y == 1); }
//     y
// }


unconstrained fn invu(x: Field) -> Field {
    inv(x)
} 

fn inv(x: Field) -> Field {
    let mut cur = x;
    let mut y = 1;
    // precomputed binary representation of p-2
    let rep: [u1; 253] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1];
    // exponentiation by squaring
    for i in 0..253 {
        if (rep[i] == 0) {
            cur = cur * cur;       
        } else {
            y = y * cur;
            cur = cur * cur;   
        }
    }
    y * cur
}

// unconstrained fn verify_assert(b: bool) {}
