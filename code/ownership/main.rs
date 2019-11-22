fn main() {
    // immutable (stored in the stack) - size known at compile time
    let name = "Julieth";
    println!("name: {}", name);

    // x is copied, so there are 2 values in the stack with 5 as value
    let x = 5;
    let y = x;
    println!("{} {}", x, y);

    // mutable (stored in the heap) - size only known at runtime
    // mutable metadata is stored in the stack
    // memory is released when heap vars goes out of scope
    // memory is returned by calling the drop method automatically
    let mut input = String::from("Hi, ");
    input.push_str(name);
    println!("input: {}", input);

    // stack data is copied, but both values point to the same place in the heap
    // here s2 will own the string content, ownership is moved
    // any assignment copies values in the stack, not the heap
    // to avoid a doble free, calling drop twice, s2 now owns data in the heap
    // string metadata lives in the stack (ptr, len, capacity)
    // data in the heap is moved from s1 to s2 (new owner)
    // rust will never automatically create “deep” copies of your data
    // rust copies metadata (shallow copy) and pushes values into the stack (this is faster)
    let s1 = String::from("hello");
    let s2 = s1;
    println!("str: {}", s2);

    // in order to create deep copies, use the clone method
    // expensive operation for large structures
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("{} {}", s3, s4);

    // stack vars are Copy types (data is copied in the stack)
    // assignment does not require allocation in the heap, so no drop needed
    let fa: f64 = 3.14;
    let fb: f64 = fa;
    let fb = fb + fb; // shadow
    println!("{} {}", fa, fb);

    // functions
    // when calling functions, passing variables will move or copy
    // if variables live in the stack, they can be reused in the scope
    // if variables live in the heep, ownership is transfer to the function
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    println!("{} again", x);

    // assigning a value to another variable moves it
    // when a variable that includes data on the heap goes out of scope
    // the value will be cleaned up by drop, unless
    // the data has been moved to be owned by another variable.
    let sx = gives_ownership();
    let sy = String::from("hello");
    let sz = takes_and_gives_back(sy);
    println!("{} {}", sx, sz);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_str = String::from("hi");
    some_str
}

fn takes_and_gives_back(a_str: String) -> String {
    a_str
}
