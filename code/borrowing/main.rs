// Rules
// At any given time, you can have either one mutable reference
// or any number of immutable references
// References must always be valid

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    
    let mut s2 = String::from("Jon");
    change(&mut s2);
    println!("New str: '{}'", s2);

    let mut s = String::from("hello");

    // a reference’s scope starts from where it is introduced
    // continues through the last time that reference is used
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    // scope can be defined with {} or ends when vars are last used
    // rust is really smart at deallocating data and aliasing/mutation

    /*
    // error:
    // only 1 mutable reference to a particular piece of data in a particular scope
    // avoid data races
    // code with data races will not compile
    let r1 = &mut s;
    let r2 = &mut s;
    */

    // scopes end when vars are last used, no just with {}  
    let r3 = &mut s; // no problem
    println!("{}", r3);

    // slices
    // contiguous sequence of elements in a collection
    // rather than the whole collection
    let s = String::from("hello world");
    let world = &s[6..11];
    println!("{}", world);

    let slice = first_word(&s);
    println!("{}", slice);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_str: &mut String) {
    some_str.push_str("!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
