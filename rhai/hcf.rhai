let a = num1();
let b = num2();

while b != 0 {
    let temp = b;
    b = a % b;
    a = temp;
}
a