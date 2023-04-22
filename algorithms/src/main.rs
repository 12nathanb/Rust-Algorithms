fn main() {}

pub fn binary_search(values: Vec<i32>, target: i32) -> i32 {
    let (mut low, mut high) = (0, values.len());

    while low <= high + 1 {
        let middle = (low + high) / 2;

        if target == values[middle] {
            return values[middle];
        } else if target < values[middle] {
            high = middle;
        } else {
            low = middle;
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_test() {
        assert_eq!(7, binary_search(vec![2, 3, 5, 7, 8, 10, 12, 15, 18, 20], 7));
        assert_eq!(
            73,
            binary_search(
                vec![
                    2, 3, 5, 7, 8, 10, 12, 15, 18, 20, 22, 24, 26, 30, 40, 50, 60, 65, 70, 71, 72,
                    73, 74, 75, 80, 90, 100
                ],
                73
            )
        );
    }
}
