fn main() {
    let y = {
        let x = 3;
        x + 1 // Note that this line doesn’t have a semicolon at the end. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
    };
    println!("The value of y is: {y}");
}
