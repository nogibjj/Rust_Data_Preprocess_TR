pub fn calculate_median(values: &[f64]) -> f64 {
    let mut sorted_values = values.to_owned();
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = sorted_values.len();
    if len % 2 == 1 {
        sorted_values[len / 2]
    } else {
        (sorted_values[len / 2 - 1] + sorted_values[len / 2]) / 2.0
    }
}
