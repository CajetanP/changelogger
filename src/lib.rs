

// TODO: should return result with proper error handling
pub fn add_exercise(language: &str, name: &str, source: &str) {
    println!("adding exercise: [{}] {} ({})", language, name, source);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_fn() {
        assert_eq!(test_fn(), 8);
    }

}
