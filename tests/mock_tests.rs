fn setup() {}
fn teardown() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vacuity() {
        setup();
        assert!(true);
        teardown();
    }

    #[test]
    fn add_two() {
        setup();
        assert_eq!(4, 2 + 2);
        teardown();
    }

    #[test]
    fn dont_add_two() {
        setup();
        assert_ne!(2, 2 + 2);
        teardown();
    }
}
