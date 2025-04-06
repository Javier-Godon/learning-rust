//arrays, tuples,slices, and strings (slice string)

pub fn compounds() {
    // Arrays
    // Arrays are fixed-size collections of elements of the same type
    // They are stored on the stack
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // array of integers
    println!("Array: {:?}", numbers);
    //Two types of formats:
    // 1. Debug format: is used for debugging purposes and is not meant for end-user display
    // 2. Display format: is used for end-user display and is meant to be more user-friendly
    let fruits: [&str; 3] = ["apple", "banana", "cherry"]; // array of strings
    println!("Fruits: {:?}", fruits);
    println!("Fruits first element: {:?}", fruits[0]);

    // Tuples
    // Tuples are fixed-size collections of elements of different types
    // They are stored on the stack
    let person: (&str, i32, bool) = ("Alice", 30, false); // tuple of string slice and integer
    println!("Tuple: {:?}", person);
    println!("Tuple first element: {:?}", person.0);

    let another_person: (String, i32, bool) = ("Bob".to_string(), 25, true); // tuple of string and integer
    println!("Another Tuple: {:?}", another_person);
    println!("Another Tuple first element: {:?}", another_person.0);

    let mix_tuple: (i32, f64, char, [i32; 5], (&str, f32)) =
        (42, 3.14, 'x', [1, 2, 3, 4, 5], ("John", 1.5)); // tuple of integer, float and char and array
    println!("Mix Tuple: {:?}", mix_tuple);
    println!("Mix Tuple first element: {:?}", mix_tuple.0);
    println!("Mix Tuple second element: {:?}", mix_tuple.1);
    println!("Mix Tuple third element: {:?}", mix_tuple.2);
    println!("Mix Tuple fourth element: {:?}", mix_tuple.3);
    println!("Mix Tuple fifth element: {:?}", mix_tuple.4);
    println!(
        "Mix Tuple fifth element first element: {:?}",
        mix_tuple.4 .0
    );
    println!(
        "Mix Tuple fifth element second element: {:?}",
        mix_tuple.4 .1
    );

    // Slices
    // Slices are dynamically-sized views into a contiguous sequence of elements
    // They are stored on the stack
    let slice: &[i32] = &numbers[1..4]; // slice of the array
    println!("Slice: {:?}", slice);
    // Slices are used to borrow a portion of an array or vector without taking ownership
    // They are useful for passing a portion of an array or vector to a function
    // without copying the entire array or vector

    let animal: &[&str] = &["cat", "dog", "fish"]; // slice of the array
    println!("Animal: {:?}", animal);

    //string slice
    // String slices are views into a string
    // They are used to borrow a portion of a string without taking ownership

    let book: &[String] = &[
        "book1".to_string(),
        "book2".to_string(),
        "book3".to_string(),
    ]; // slice of the array
    println!("Book: {:?}", book);

    let book_weird: &[&String] = &[
        &"book1".to_string(),
        &"book2".to_string(),
        &"book3".to_string(),
    ]; // slice of the array
    println!("Book: {:?}", book_weird);

    // Strings Vs String Slices (&str)
    // Strings are owned, growable, and mutable. They are stored on the heap
    // String slices are borrowed, fixed-size, and immutable. They are stored on the stack. $str is a reference to a string, you do not have to copy the string
    // It allows you to work with strings without taking ownership of them
    let string: String = "Hello, world!".to_string(); // owned string
    let string_slice: &str = &string[0..5]; // string slice
    println!("String: {:?}", string);
    println!("String slice: {:?}", string_slice);

    let mut stone_cold: String = String::from("Hell, "); // owned string, stored on the heap
    println!("Stone Cold says: {:?}", stone_cold);
    stone_cold.push_str("Yeah!"); // push string slice to the owned string
    println!("Stone Cold says: {:?}", stone_cold);

    let string: String = String::from("Hello, world!"); // owned string
    let string_slice: &str = &string;
    println! ("Slice Value: {}", string_slice);
    let string_slice: &str = &string[0..5]; // string slice
    println! ("Slice Value: {}", string_slice);


    
}
