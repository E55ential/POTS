/// Вычислите разность между значениями values на расстоянии offset друг от друга,
/// переходя по модулю в начало коллекции.
///
/// Элемент n результата это разность values[(n+offset)%len] - values[n].
fn offset_differences(offset: usize, values: Vec<i32>) -> Vec<i32> {
    let len = values.len();

    if len == 0 {
        return vec![];
    }

    (0..len)
        .map(|i| {
            let j = (i + offset) % len;
            values[j] - values[i]
        })
        .collect()
}

#[test]
fn test_offset_one() {
    assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
    assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
    assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
    assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
    assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
    assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_degenerate_cases() {
    assert_eq!(offset_differences(1, vec![0]), vec![0]);
    assert_eq!(offset_differences(1, vec![1]), vec![0]);
    let empty: Vec<i32> = vec![];
    assert_eq!(offset_differences(1, empty), vec![]);
}

fn main() {
    let values = vec![1, 3, 5, 7];

    let result1 = offset_differences(1, values.clone());
    println!("offset = 1: {:?}", result1);

    let result2 = offset_differences(2, values.clone());
    println!("offset = 2: {:?}", result2);

    let result3 = offset_differences(3, values.clone());
    println!("offset = 3: {:?}", result3);

    let result4 = offset_differences(4, values.clone());
    println!("offset = 4: {:?}", result4);
}
