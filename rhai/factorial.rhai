let n = num();
if n < 0 {
    0  
} else if n == 0 || n == 1 {
    1
} else {
    let result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}