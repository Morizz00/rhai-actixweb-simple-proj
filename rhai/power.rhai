let base_val = base();
let exp_val = exponent();

if exp_val == 0 {
    1
} else if exp_val < 0 {
    if base_val == 1 {
        1
    } else if base_val == -1 {
        if exp_val % 2 == 0 { 1 } else { -1 }
    } else {
        0
    }
} else {
    let result = 1;
    for i in 0..exp_val {
        result *= base_val;
    }
    result
}