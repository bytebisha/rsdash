#[allow(dead_code)]

/**
* Add two numbers
*
* # Arguments
* `augend` - The first number to add
* `addend` - The second number to add
*
* # Example
* let sum = add(1, 2);
* assert_eq!(sum, 3);
*
*/

pub fn add(augend: i32, addend: i32) -> i32 {
    augend + addend
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
