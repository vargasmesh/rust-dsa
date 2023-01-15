fn insertion_sort<T: PartialOrd + Copy>(list: &mut [T]) {
    for i in 1..list.len() {
        let key = list[i];
        let mut j = i;

        while j > 0 && list[j - 1] > key {
            list[j] = list[j - 1];
            j -= 1;
        }

        list[j] = key;
    }
}

#[cfg(test)]
mod tests {
    use crate::insertion_sort::insertion_sort;

    use super::super::is_sorted;

    #[test]
    fn it_is_sorted() {
        let mut list = [3, 0, 9, 1, 5];
        insertion_sort(&mut list);
        println!("{:?}", list);
        assert_eq!(true, is_sorted(&list));
    }
}
