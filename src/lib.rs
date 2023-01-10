pub fn is_sorted<T: Ord>(iterable: &[T]) -> bool {
    for i in 1..iterable.len() {
        if iterable[i] < iterable[i - 1] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_empty() {
        let empty_vec: Vec<i32> = vec![];
        assert_eq!(true, is_sorted(&empty_vec));
    }

    #[test]
    fn it_is_sorted() {
        assert_eq!(true, is_sorted(&[1, 2, 3, 4, 5]));
    }
    #[test]
    fn it_is_not_sorted() {
        assert_eq!(false, is_sorted(&[3, 1, 0, 9, 2]));
    }
}
