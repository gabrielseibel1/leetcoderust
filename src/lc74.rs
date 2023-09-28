use std::cmp::Ordering;

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    binary_search_matrix(&matrix, target) == Ordering::Equal
}

fn binary_search_matrix(matrix: &[Vec<i32>], target: i32) -> Ordering {
    if matrix.len() == 1 {
        return binary_search_vector(&matrix[0], target);
    }
    let mid = matrix.len() / 2;
    match binary_search_vector(&matrix[mid], target) {
        Ordering::Equal => Ordering::Equal,
        Ordering::Greater => binary_search_matrix(&matrix[0..mid], target),
        Ordering::Less => binary_search_matrix(&matrix[mid..matrix.len()], target),
    }
}

fn binary_search_vector(vector: &[i32], target: i32) -> Ordering {
    if vector.len() == 1 {
        return vector[0].cmp(&target);
    }
    let mid = vector.len() / 2;
    match vector[mid].cmp(&target) {
        Ordering::Equal => Ordering::Equal,
        Ordering::Greater => binary_search_vector(&vector[0..mid], target),
        Ordering::Less => binary_search_vector(&vector[mid..vector.len()], target),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_matrix() {
        let result = search_matrix(
            vec![
                vec![1, 3, 5, 7],
                vec![10, 11, 16, 20],
                vec![23, 30, 34, 60],
            ],
            3,
        );
        assert!(result);
        let result = search_matrix(
            vec![
                vec![1, 3, 5, 7],
                vec![10, 11, 16, 20],
                vec![23, 30, 34, 60],
            ],
            13,
        );
        assert!(!result);
        let result = search_matrix(
            vec![
                vec![1, 3, 5, 7, 9],
                vec![10, 11, 16, 20, 21],
                vec![23, 30, 34, 60, 67],
                vec![70, 73, 81, 90, 91],
                vec![92, 95, 100, 103, 105],
                vec![110, 120, 130, 140, 150],
            ],
            91,
        );
        assert!(result);
        let result = search_matrix(
            vec![
                vec![1, 3, 5, 7, 9],
                vec![10, 11, 16, 20, 21],
                vec![23, 30, 34, 60, 67],
                vec![70, 73, 81, 90, 91],
                vec![92, 95, 100, 103, 105],
                vec![110, 120, 130, 140, 150],
            ],
            87,
        );
        assert!(!result);
    }
}