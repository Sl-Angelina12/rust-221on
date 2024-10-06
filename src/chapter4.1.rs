#[test]
fn test1() {
    let x: i32 = 5;
    let mut _y = 5;
    _y = x;
    let _z: i32 = 10; // Type of z ?
    println!("Success!");
}

#[test]
fn test2() {
    let _v: u16 = 38_u8 as u16;
    println!("Success!");
}

#[test]
fn test3() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));
    println!("Success!");
}
// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    println!("Success!");
}

#[test]
fn test5() {
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
}

#[test]
fn test6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
    assert_eq!(v, 1597);
    println!("Success!");
}

// Fill the blank to make it work
#[test]
fn test7() {
    let x = 1_000.000_1; // f64
    let _y: f32 = 0.12; // f32
    let _z = 0.01_f64; // f64
    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}
fn type_of1<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test8() {
    assert_eq!(0.1_f32 + 0.2_f32, 0.3_f32); //or as f32
    println!("Success!");
}

#[test]
fn test9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }
    assert_eq!(sum, -5); //assert! (.. == ..)
    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}
use std::ops::{Range, RangeInclusive};
#[test]
fn test10() {
    assert_eq!(1..5, Range{ start: 1, end: 5 }); //була проблема з непотрібними дужками лол
    assert_eq!(1..=5, RangeInclusive::new(1, 5)); //
    println!("Success!");
}

#[test]
fn test11() {
    // Integer addition
    assert_eq!(1u32 + 2u32, 3u32);

    // Integer subtraction
    assert_eq!(1i32 - 2i32, -1i32);
    assert_eq!(1i8 - 2i8, -1i8); // з 1u8 в і8

    assert_eq!(3 * 50, 150);

    assert_eq!(9.6_f32 / 3.2_f32, 3.0_f32); // error ! make it work

    assert_eq!(24 % 5, 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert_eq!(!true, false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}