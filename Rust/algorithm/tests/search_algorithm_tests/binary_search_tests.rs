use algorithms::search_algorithm::binary_search::search;

#[test]
fn empty_slice_returns_none() {
    let empty: [i32; 0] = [];
    assert_eq!(search(&empty, &1), None);
}

#[test]
fn single_element_found() {
    assert_eq!(search(&[5], &5), Some(0));
}

#[test]
fn single_element_not_found() {
    assert_eq!(search(&[5], &3), None);
    assert_eq!(search(&[5], &7), None);
}

#[test]
fn finds_first_element() {
    assert_eq!(search(&[1, 3, 5, 7, 9], &1), Some(0));
}

#[test]
fn finds_last_element() {
    assert_eq!(search(&[1, 3, 5, 7, 9], &9), Some(4));
}

#[test]
fn finds_middle_element() {
    assert_eq!(search(&[1, 3, 5, 7, 9], &5), Some(2));
}

#[test]
fn finds_every_element_in_odd_length_slice() {
    let numbers = [1, 3, 5, 7, 9, 11, 13];
    for (index, value) in numbers.iter().enumerate() {
        assert_eq!(search(&numbers, value), Some(index));
    }
}

#[test]
fn finds_every_element_in_even_length_slice() {
    let numbers = [2, 4, 6, 8, 10, 12];
    for (index, value) in numbers.iter().enumerate() {
        assert_eq!(search(&numbers, value), Some(index));
    }
}

#[test]
fn target_smaller_than_all_returns_none() {
    assert_eq!(search(&[10, 20, 30], &5), None);
}

#[test]
fn target_larger_than_all_returns_none() {
    assert_eq!(search(&[10, 20, 30], &40), None);
}

#[test]
fn target_between_elements_returns_none() {
    assert_eq!(search(&[10, 20, 30], &25), None);
}

#[test]
fn works_with_negative_numbers() {
    let numbers = [-10, -5, -1, 0, 3, 8];
    assert_eq!(search(&numbers, &-5), Some(1));
    assert_eq!(search(&numbers, &0), Some(3));
    assert_eq!(search(&numbers, &-7), None);
}

#[test]
fn duplicates_return_index_of_an_equal_element() {
    let numbers = [1, 2, 2, 2, 3];
    let index = search(&numbers, &2).expect("2 is present");
    assert_eq!(numbers[index], 2);
}

#[test]
fn works_with_strings() {
    let words = ["apple", "banana", "cherry", "date"];
    assert_eq!(search(&words, &"cherry"), Some(2));
    assert_eq!(search(&words, &"fig"), None);
}

#[test]
fn works_with_owned_strings() {
    let words = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    assert_eq!(search(&words, &"b".to_string()), Some(1));
}

#[test]
fn agrees_with_std_binary_search_on_large_input() {
    let numbers: Vec<u32> = (0..1000).map(|n| n * 3).collect();
    for target in 0..3000 {
        let expected = numbers.binary_search(&target).ok();
        assert_eq!(search(&numbers, &target), expected);
    }
}
