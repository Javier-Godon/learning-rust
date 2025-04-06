pub fn functions() {
    first_function();
    tell_height(180);
    println!("{}", human_id("John", 36, 182.2));
    let total_invoice = {
        let price = 5;
        let quantity = 10;
        price * quantity
    };
    println!("The total price is: {}", total_invoice);
    let total_added = add(5, 10);
    println!("{} + {} = {}", 5, 10, total_added);

    let weight = 70.0;
    let height = 1.82;
    let total_bmi = calculate_bmi(weight, height);
    println!("Your BMI is: {}", total_bmi);
}

fn first_function() {
    println!("Hello from first function");
}

fn tell_height(height: u32) {
    println!("Your height is: {}", height);
}

fn human_id(name: &str, age: u32, height: f32) -> String {
    format!("Name: {}, Age: {}, Height: {}", name, age, height)
}

// Expressions and Statements *****************
// Expressions are code that evaluates to a value (they return a value), they do not have a semicolon at the end and can be used as part of a statement.
// Examples of expressions are: function calls, arithmetic operations, and control flow statements (if, match, etc.)
// Statements are code that performs an action, they do not return a value and are used to control the flow of the program, they have a semicolon at the end and connot be used as part of an expression
// Examples of statements are: variable declarations, function definitions, and control flow statements (if, match, etc.)

fn add(a: i32, b: i32) -> i32 {
    a + b // this is an expression, it returns a value
}

fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height) // this is an expression, it returns a value
}

//min 1:05:57
