#[test]
fn test1() {
    let s: &str = "hello, world";

    println!("Success!");
}

#[test]
fn test2() {
    let s: &str = "hello, world";
    greetings(s)
}
    fn greetings(s: &str) {
    println!("{}",s)
}

#[test]
fn test3() {
    let mut s = String::from("");
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
    println!("{}", s);
    println!("Success!");
}

#[test]
fn test4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}

#[test]
fn test5() {
    let mut s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

#[test]
fn test6() {
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");
    let s3: String = s1 + s2.as_str(); //String -> &str
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}

#[test]
fn test7() {
    let s: &str = "hello, world";
    greetings(String::from(s))
}

fn greetings(s: String) {
    println!("{}", s)
}

#[test]
fn test8() {
    let s: String = "hello, world".to_string();
    let s1: &str = s.as_str();

    println!("Success!");
}

#[test]
fn test9() {
    // You can use escapes to write bytes by their hexadecimal values
    // Fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

#[test]
fn test10() {
    let s1: String = String::from("hi,中国");
    let h: &str = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("{}", h1);

    println!("Success!");
}

#[test]
fn test11() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}
