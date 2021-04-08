fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        insta::assert_snapshot!("asdf", @"asdf")
    }
}
