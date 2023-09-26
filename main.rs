fn find_median(arr_1: &Vec<usize>, arr_2: &Vec<usize>) -> f64 {
    let mut merged_arr: Vec<usize> = Vec::new();

    if arr_1.len() == 0 && arr_2.len() == 0 {
        panic!("arrays are empty!");
    }
    merged_arr.extend(arr_1.iter());
    merged_arr.extend(arr_2.iter());

    merged_arr.sort();

    let len = merged_arr.len();

    if len == 0 {
        return 0.0;
    }

    match len.checked_rem(2) {
        Some(1) => {
            let middle_index = len / 2;
            return merged_arr[middle_index] as f64;
        }

        Some(0) => {
            let middle_index_1 = (len / 2) - 1;
            let middle_index_2 = len / 2;

            let middle_value_1 = merged_arr[middle_index_1];
            let middle_value_2 = merged_arr[middle_index_2];

            return (middle_value_1 + middle_value_2) as f64 / 2.0;
        }

        _ => unreachable!(),
    }

}

fn main() {
    // Example usage
    let mut arr1 = vec![1, 2, 3];
    let mut arr2 = vec![4, 5, 6, 7];
    let median = find_median(&arr1, &arr2);
    println!("Median: {}", median);

    arr1 = vec![1, 2, 3, 1, 2, 3];
    arr2 = vec![3, 2, 1, 3, 2, 1];
    let median = find_median(&arr1, &arr2);
    println!("Median: {}", median);
}

