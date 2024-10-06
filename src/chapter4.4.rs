#[test]
fn test1() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum1(x, y);
    assert_eq!(s, 3);
    println!("Success!");
}
fn sum1(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test2() {
    print();
}
// Replace i32 with another type
fn print() -> () {
    println!("Success!")
}

#[test]
// Solve it in two ways
// DON'T let println! work
fn test3() {
    never_return();
    println!("Failed!");
}
fn never_return() -> () { //було так fn never_return() -> !
    // Implement this function, don't modify the fn signatures

}

#[test]
fn test4() {
    println!("Success!");
}
fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}
// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    todo!() //or panic!() or unimplemented!()
}

#[test]
fn test5() {
    // FILL in the blank
    let b = false;
    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for false, but we can panic");
        }
    };
    println!("Exercise Failed if printing out this line!");
}