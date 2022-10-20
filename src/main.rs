fn main() {
    println!("Hello, world!");
    println!("{}", my_add(2, 3));
}

fn my_add(a: i8, b: i8) -> i8 {
    a + b
}

#[cfg(test)]
mod tests {
    // importing names from outer
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(my_add(2, 3), 5);
    }
}
