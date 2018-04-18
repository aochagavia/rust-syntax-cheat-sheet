// This is a comment. Line comments look like this...
// and extend multiple lines like this.

/// Documentation comments look like this and support markdown notation.
/// # Examples
///
/// ```
/// let five = 5
/// ```

///////////////
// 1. Basics //
///////////////

// Functions
// `i32` is the type for 32-bit signed integers
fn add_ints(x: i32, y: i32) -> i32 {
    // Implicit return (no semicolon)
    x + y
}

// `u32` is the type for 32-bit unsigned integers
fn add_uints(x: i32, y: i32) -> i32 {
    // Explicit return
    return x + y;
}

// Main function
fn main() {
    // Immutable bindings
    // <var>: <type> = <value>
    let x: i32 = 42;
    let y: f64 = 0.5; // Note: `f64` is the type for 64-bit floating point numbers

    // Type inference
    // Most of the time, the Rust compiler can infer what type a variable is, so
    // you don’t have to write an explicit type annotation.
    // Throughout this document, types are explicitly annotated in many places,
    // but only for demonstrative purposes. Type inference can handle this for
    // you most of the time.
    let implicit_x = 1;
    let implicit_f = 1.3;

    // Mutable variables
    let mut mutable = 1;
    mutable = 4;
    mutable += 2;

    // Strings //

    // String literals
    let x: &str = "hello world!";

    // Printing
    println!("{} {}", y, x); // 0.5 hello world

    // A `String` is a heap-allocated string
    let s: String = String::from("hello world");

    // A string slice is an immutable view into another string
    // This is basically a pointer and a length – it doesn’t
    // actually contain the contents of a string, just a pointer to
    // the begin of a string buffer and the length of the string
    let s_slice: &str = &s;

    println!("{} {}", s, s_slice); // hello world hello world

    // Vectors/arrays //

    // A fixed-size array
    let four_ints: [i32; 4] = [1, 2, 3, 4];

    // A dynamic array (vector)
    let mut vector: Vec<i32> = vec![1, 2, 3, 4];
    vector.push(5);

    // A slice – an immutable view into a vector or array
    // This is much like a string slice, but for vectors
    let slice: &[i32] = &vector;

    // Use `{:?}` to print something debug-style
    println!("{:?} {:?}", vector, slice); // [1, 2, 3, 4, 5] [1, 2, 3, 4, 5]

    // Tuples //

    // A tuple is a fixed-size set of values of possibly different types
    let x: (i32, &str, f64) = (1, "hello", 3.4);

    // Destructuring `let`
    let (a, b, c) = x;
    println!("{} {} {}", a, b, c); // 1 hello 3.4

    // Indexing
    println!("{}", x.1); // hello

    //////////////
    // 2. Types //
    //////////////

    // Struct
    struct Point {
        x: i32,
        y: i32,
    }

    // The only way to build a struct is by providing values for all its fields,
    // as shown below
    let origin: Point = Point { x: 0, y: 0 };

    // A struct with unnamed fields, called a ‘tuple struct’
    struct Point2(i32, i32);

    let origin2 = Point2(0, 0);

    // Basic C-like enum
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    let up = Direction::Up;

    // Enum with fields
    enum OptionalI32 {
        AnI32(i32),
        Nothing,
    }

    let two: OptionalI32 = OptionalI32::AnI32(2);
    let nothing = OptionalI32::Nothing;

    // Generics //

    struct Foo<T> { bar: T }

    fn identity<T>(x: T) -> T {
        x
    }

    // Methods //

    // Methods are defined in `impl` blocks, not in the
    // `struct` or `enum` declarations
    impl Point {
        // Methods take an explicit `self` parameter
        fn get_x(self) -> i32 {
            self.x
        }

        // Static methods have no `self` parameter
        fn new_point(a: i32, b: i32) -> Point {
            Point {
                x: a,
                y: b
            }
        }
    }

    let a_point = Point::new_point(1, 2);
    println!("{}", a_point.get_x()); // 1

    // Traits (known as interfaces or typeclasses in other languages) //

    trait Multiply {
        fn multiply(self, factor: i32) -> Self;
    }

    impl Multiply for Point {
        fn multiply(self, factor: i32) -> Point {
            Point {
                x: self.x * factor,
                y: self.y * factor
            }
        }
    }

    let another_point = Point { x: 1, y: 2 };
    println!("{:?}", another_point.multiply(2).get_x()); // 2

    /////////////////////////
    // 3. Pattern matching //
    /////////////////////////

    // Pattern matching is switch on steroids... Not only can you match on
    // values, but also on enum variants
    let foo = OptionalI32::AnI32(1);
    match foo {
        OptionalI32::AnI32(n) => println!("it’s an i32: {}", n),
        OptionalI32::Nothing  => println!("it’s nothing!"),
    }

    // Advanced pattern matching
    struct FooBar { x: i32, y: OptionalI32 }
    let bar = FooBar { x: 15, y: OptionalI32::AnI32(32) };

    match bar {
        FooBar { x: 0, y: OptionalI32::AnI32(0) } =>
            println!("The numbers are zero!"),
        FooBar { x: n, y: OptionalI32::AnI32(m) } if n == m =>
            println!("The numbers are the same"),
        FooBar { x: n, y: OptionalI32::AnI32(m) } =>
            println!("Different numbers: {} {}", n, m),
        FooBar { x: _, y: OptionalI32::Nothing } =>
            println!("The second number is Nothing!"),
    }

    /////////////////////
    // 4. Control flow //
    /////////////////////

    // `for` loops/iteration
    let array = [1, 2, 3];
    for i in array.iter() {
        println!("{}", i);
    }

    // Ranges
    for i in 0..10 {
        print!("{} ", i);
    }
    println!("");
    // prints `0 1 2 3 4 5 6 7 8 9 `

    // `if`
    if 1 == 1 {
        println!("Maths is working!");
    } else {
        println!("Oh no...");
    }

    // `if` is also an expression!
    let value = if true {
        "good"
    } else {
        "bad"
    };

    // `while` loop
    let mut flag = true;
    while flag {
        println!("Only one iteration");
        flag = !flag;
    }

    // Infinite loop
    loop {
        println!("Hello!");

        // Break or this will never end
        break;
    }

    /////////////////////////////////
    // 5. Memory safety & pointers //
    /////////////////////////////////

    // Smart pointer to a value on the heap. Deallocated when it falls out of scope.
    let mut mine: Box<i32> = Box::new(3);
    *mine = 5; // dereference
    println!("{}", mine); // 5 (note: doesn't print the pointer address)

    // Shared reference – a pointer to some data that disallows mutation
    // Multiple shared references may exist at the same time
    let mut var = 4;
    let ref_var: &i32 = &var;
    let another_ref_var = &var;
    // *ref_var = 5; // this would not compile, because mutation is forbidden

    println!("{}", var); // 4
    println!("{}", *ref_var); // 4

    // Unique reference - a pointer to some data that allows mutation
    // Only one unique reference may exist to the same data
    let mut var2 = 4;
    let ref_var2: &mut i32 = &mut var2;
    // let another_ref_var2 = &mut var2; // this would not compile, because the reference must be unique

    *ref_var2 += 2;

    println!("{}", *ref_var2); // 6
}

// Adapted from https://learnxinyminutes.com/docs/rust/
