use nanoid::nanoid;

const NANOID_LENGTH: usize = 25;
const NANOID_ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Generate a new unique ID
pub fn id() -> String {
    nanoid!(NANOID_LENGTH, &NANOID_ALPHABET)
}

pub fn id_with_length(length: usize) -> String {
    nanoid!(length, &NANOID_ALPHABET)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_generates_correct_length() {
        for _ in 0..100 {
            let id = id();
            assert_eq!(id.len(), NANOID_LENGTH);
        }
    }

    #[test]
    fn id_generates_correct_characters() {
        for _ in 0..100 {
            let id = id();
            assert!(id.chars().all(|ch| NANOID_ALPHABET.contains(&ch)));
        }
    }

    #[test]
    fn id_no_repeats() {
        let mut prev_id = String::new();
        for _ in 0..10_000 {
            let id = id();
            assert_ne!(id, prev_id);
            prev_id = id;
        }
    }

    #[test]
    fn id_with_length_generates_correct_length() {
        for i in 1..100 {
            let id = id_with_length(i);
            assert_eq!(id.len(), i);
        }
    }
}
