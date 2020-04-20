

fn main() {
	println!("Hello world.");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_bad_add() {
        assert_eq!(1, 3);
    }
}