fn selection_sort<T: PartialOrd + Copy>(list: &mut [T]) {
    let n = list.len();
    for i in 0..n {
        let mut min = i;

        for j in i + 1..n {
            if list[j] < list[min] {
                min = j
            };
        }
        list.swap(i, min)
    }
}

#[cfg(test)]
mod tests {
    use crate::selection_sort::selection_sort;

    use super::super::is_sorted;

    #[test]
    fn it_is_sorted() {
        let mut list = [3, 0, 9, 1, 5];
        selection_sort(&mut list);
        assert_eq!(true, is_sorted(&list));
    }
}
