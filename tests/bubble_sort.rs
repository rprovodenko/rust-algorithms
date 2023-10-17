// use rust_algorithms::bubble_sort;
use rust_algorithms::bubble_sort::bubble_sort;

fn is_sorted(vector: &Vec<i32>) -> bool {
    let mut previous: Option<i32> = None;
    for el in vector.iter() {
        match previous {
            Some(x) => {
                if x > *el {
                    return false;
                }
            }
            _ => {
                previous = Some(*el);
            }
        }
    }

    true
}

#[test]
fn it_works() {
    let vector = Vec::new();
    bubble_sort(vector);
}

#[test]
fn it_works_for_one_element() {
    let mut vector: Vec<i32> = vec![1];
    vector = bubble_sort(vector);
    assert!(is_sorted(&vector));
}

#[test]
fn it_works_for_two_elements_sorted() {
    let mut vector: Vec<i32> = vec![1, 2];
    vector = bubble_sort(vector);
    assert!(is_sorted(&vector));
}

#[test]
fn it_works_for_two_elements_unsorted() {
    let mut vector: Vec<i32> = vec![2, 1];
    vector = bubble_sort(vector);
    assert!(is_sorted(&vector));
}

#[test]
fn it_works_for_a_bigger_vector() {
    let mut vector: Vec<i32> = vec![
        213, 12, 123432, 11224, 6867, 32413, 78787653, 1213, 122, 4446, 146575, 1123245, 654654,
        1231,
    ];
    vector = bubble_sort(vector);
    assert!(is_sorted(&vector));
}
