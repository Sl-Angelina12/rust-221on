fn is_prime(n: &u32) -> bool

#[test]
fn test_is_prime() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];


    test_data
        .iter()
        .for_each(|(n, prime)|
            assert_eq!(is_prime(n), *prime)
        )
}
