pub fn add_one(i: i32) -> i32 {
    i + 1
}

#[cfg(test)]
mod tests {
    use add_one;

    #[test]
    fn test_add_one() {
        assert_eq!(add_one(1), 2);
    }

}
