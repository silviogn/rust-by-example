// Rust provides type safety via static typing.
// Variable bindings can be type annotated when declared.
// However, in most cases, the compiler will be able to infer
// the type of the variable from the context, heavily reducing the annotation burden.

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning
    // Please note that warnings may not be shown in a browser

    //-------------------------
    // Mutability
    // Variable bindings are immutable by default,
    // but this can be overridden using the mut modifier.

    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error! Cannot assign a new value to an immutable variable
    //_immutable_binding += 1;

    //-------------------------
    // Shadowing

    {
        let shadowed_binding = 1;
        {
            println!("before being shadowed: {}", shadowed_binding);
            // This binding *shadows* the outer one
            let shadowed_binding = "abc";
            println!("shadowed in inner block: {}", shadowed_binding);
        }
        println!("outside inner block: {}", shadowed_binding);
        // This binding *shadows* the previous binding
        let shadowed_binding = 2;
        println!("shadowed in outer block: {}", shadowed_binding);
    }

    //-------------------------
    //Declare first
    // It's possible to declare variable bindings first,
    // and initialize them later. However, this form is seldom used,
    // as it may lead to the use of uninitialized variables.
    // Declare a variable binding
    let a_binding;
    {
        let x = 2;
        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);
    let another_binding;
    // Error! Use of uninitialized binding
    another_binding = 1;
    println!("another binding: {}", another_binding);
    // FIXME ^ Comment out this line
    println!("another binding: {}", another_binding);


    //-------------------------
    //Freezing
    // When data is bound by the same name immutably, it also freezes.
    // Frozen data can't be modified until the immutable binding goes out of scope:
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        //_mutable_integer = 50;
        // FIXME ^ Comment out this line
        // FIXME
        // TODO
        // `_mutable_integer` goes out of scope
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;

}


















