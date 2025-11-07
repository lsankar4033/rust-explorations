// Ownership & Borrowing Exercises
// Each exercise is marked as BROKEN - uncomment and fix it!
// Run with: cargo run -p ownership-exercises
// Check with: cargo check -p ownership-exercises

fn main() {
    println!("Ownership Exercises\n");

    exercise_1_move_semantics();
    exercise_2_clone_vs_copy();
    exercise_3_borrowing_basics();
    exercise_4_mutable_references();
    exercise_5_multiple_borrows();
    exercise_6_dangling_reference();
    exercise_7_string_slices();
    exercise_8_vector_iteration();
    exercise_9_struct_ownership();
    exercise_10_lifetime_challenge();
}

// ============================================================================
// EXERCISE 1: Move Semantics
// Goal: Understand that non-Copy types move by default
// ============================================================================
#[allow(unused)]
fn exercise_1_move_semantics() {
    let s1 = String::from("hello");
    let s2 = &s1;

    println!("s1 = {}, s2 = {}", s1, s2);
}

// ============================================================================
// EXERCISE 2: Clone vs Copy
// Goal: Understand which types are Copy and which need Clone
// ============================================================================
#[allow(unused)]
fn exercise_2_clone_vs_copy() {
    // Integers are Copy - this works fine
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // No problem!

    // But Strings are not Copy...
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}", s1);

    let v1 = vec![1, 2, 3];
    let v2 = v1.clone();
    println!("v1 = {:?}", v1);
}

// ============================================================================
// EXERCISE 3: Borrowing Basics
// Goal: Use references to avoid moves
// ============================================================================
#[allow(unused)]
fn exercise_3_borrowing_basics() {
    let s = String::from("hello");

    // BROKEN: This function takes ownership!
    let len = calculate_length(&s);
    println!("Length of '{}' is {}", s, len);
}

#[allow(unused)]
fn calculate_length(s: &str) -> usize {
    s.len()
}

// ============================================================================
// EXERCISE 4: Mutable References
// Goal: Learn the rules of mutable borrowing
// ============================================================================
#[allow(unused)]
fn exercise_4_mutable_references() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("Changed string: {}", s);

    // BROKEN: Uncomment the lines below
    let r1 = &mut s;
    println!("r1 = {}", r1);

    let r2 = &mut s;
    println!("r2 = {}", r2);
}

#[allow(unused)]
fn change(s: &mut String) {
    s.push_str(", world");
}

// ============================================================================
// EXERCISE 5: Multiple Borrows - The Classic Trap
// Goal: Can't mix mutable and immutable references
// ============================================================================
#[allow(unused)]
fn exercise_5_multiple_borrows() {
    let mut s = String::from("hello");

    let r1 = &s; // immutable borrow
    let r2 = &s; // another immutable borrow (OK!)
    println!("{} {}", r1, r2);
    let r3 = &mut s; // mutable borrow (NOT OK while r1, r2 exist!)
    println!("{}", r3)
}

// ============================================================================
// EXERCISE 6: Dangling Reference
// Goal: Rust prevents dangling pointers at compile time
// ============================================================================
#[allow(unused)]
fn exercise_6_dangling_reference() {
    // BROKEN: Uncomment and try to compile
    let reference_to_nothing = dangle();
}

// BROKEN: This function tries to return a reference to a local variable
// Commented out because it won't compile - that's the point!
// Uncomment and fix when you get to this exercise
#[allow(unused)]
fn dangle() -> String {
    String::from("hello")
}

// ============================================================================
// EXERCISE 7: String Slices
// Goal: Work with string slices (&str) vs String
// ============================================================================
#[allow(unused)]
fn exercise_7_string_slices() {
    let mut s = String::from("hello world");

    // BROKEN: Implement first_word to return a slice of the first word
    let word = first_word(&s);
    println!("First word: {}", word);

    // CHALLENGE: What happens if you try to modify s while word exists?
    let word = first_word(&s);
    println!("First word: {}", word);
    s.clear()
}

#[allow(unused)]
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(index) => &s[..index],
        None => s,
    }
}

// ============================================================================
// EXERCISE 8: Vector Iteration Issues
// Goal: Understand borrowing during iteration
// ============================================================================
#[allow(unused)]
fn exercise_8_vector_iteration() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in 0..v.len() {
        v.push(10);
        println!("{}", v[i]);
    }

    // FIX OPTIONS:
    // 1. Collect items first, then modify
    // 2. Use indices instead of references
    // 3. Use drain() or other consuming iterators
}

// ============================================================================
// EXERCISE 9: Struct Ownership
// Goal: Understand ownership in struct fields
// ============================================================================
#[allow(unused)]
fn exercise_9_struct_ownership() {
    struct User {
        name: String,
        email: String,
    }

    let user = User {
        name: String::from("alice"),
        email: String::from("alice@example.com"),
    };

    let name = user.name.clone(); // This moves name out of user
    println!("User email: {}", user.email); // OK
    println!("User name: {}", user.name); // NOT OK - name was moved!

    // FIX: Use references or clone
}

// ============================================================================
// EXERCISE 10: Lifetime Challenge
// Goal: Understand when you need lifetime annotations
// ============================================================================
#[allow(unused)]
fn exercise_10_lifetime_challenge() {
    let string1 = String::from("long string");
    let string2 = String::from("short");

    // BROKEN: Uncomment and implement longest
    let result = longest(&string1, &string2);
    println!("Longest string: {}", result);
}

// BROKEN: This needs lifetime annotations
// The compiler needs to know: does the returned reference come from x or y?
// Commented out - uncomment and add lifetimes when you get here
#[allow(unused)]
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

// ============================================================================
// TESTS - Run with: cargo test -p ownership-exercises
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length_fixed() {
        // Once you fix calculate_length to use references:
        let s = String::from("hello");
        let len = calculate_length(&s);
        assert_eq!(len, 5);
        assert_eq!(s, "hello"); // s still valid!
    }

    #[test]
    fn test_first_word() {
        // Uncomment once implemented:
        let s = String::from("hello world");
        assert_eq!(first_word(&s), "hello");

        let s2 = String::from("rust");
        assert_eq!(first_word(&s2), "rust");
    }

    #[test]
    fn test_longest() {
        // Uncomment once you add lifetimes:
        assert_eq!(longest("short", "longer"), "longer");
        assert_eq!(longest("same", "size"), "same");
    }
}
