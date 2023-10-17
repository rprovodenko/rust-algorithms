pub fn bubble_sort(mut vector: Vec<i32>) -> Vec<i32> {
    let mut outer_index = 0;
    let length = vector.len();

    while outer_index < length {
        let mut inner_index = outer_index + 1;
        while inner_index < length {
            if vector[inner_index] < vector[inner_index - 1] {
                let tmp = vector[inner_index];
                vector[inner_index] = vector[inner_index - 1];
                vector[inner_index - 1] = tmp;
            }
            inner_index += 1;
        }
        outer_index += 1;
    }

    return vector;
}
