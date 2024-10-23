#[test]
fn test1() {
    let v = { //or v: i32
        let mut x = 1;
        x += 2 //;
        //x
    };
    assert_eq!(v, ()); //assert_eq!(v, 3);
    println!("Success!");
}

#[test]
fn test2() {
    let v = {
        let x = 3;
        x
    };
    assert_eq!(v, 3);
    println!("Success!");
}

#[test]
fn test3() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);
    println!("Success!");
}
fn sum(x: i32, y: i32) -> i32 {
    x + y //not ";"
}