extern crate add_one;


#[test]
fn test_add_one() {
    assert_eq!(add_one::add_one(1), 2);
}
