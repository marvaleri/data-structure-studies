#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_get() {
        let mut vec = MyVec::new();

        vec.push(10);
        vec.push(20);

        assert_eq!(vec.get(0), Some(&10));
        assert_eq!(vec.get(1), Some(&20));
        
        assert_eq!(vec.get(2), None);
    }
}
