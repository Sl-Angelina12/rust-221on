#[test]
fn test1() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); // 4байта
    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);
    println!("Success!");
}

#[test]
fn test2() {
    let c1 = '中'; //" " в ' '
    print_char(c1);
}
fn print_char(c : char) {
    println!("{}", c);
}

#[test]
fn test3() {
    let _f: bool = false;
    let t = true;
    if t {
        println!("Success!");
    }
}

#[test]
fn test4() {
    let f = false; //було тру
    let t = false; //було let t = true && false;
    assert_eq!(t, f);
    println!("Success!");
}

#[test]
fn test5() {
    let _v: () = ();
    let _s = (2, 3); //not _v? _v:(i32, i32) помилка
    assert_eq!(_v, implicitly_ret_unit());
    println!("Success!");
}
fn implicitly_ret_unit() {
    println!("I will return a ()");
}

#[test]
fn test6() {
    let unit: () = ();
    assert_eq!(size_of_val(&unit), 0);
    println!("Success!");
}