let a = num1();
let b = num2();

fn gcd(x, y) {
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}
if a == 0 || b == 0 {
    0
} else {
    (a * b) / gcd(a, b)
}