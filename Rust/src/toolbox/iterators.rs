pub fn power<T: Copy>(elements: &[T], length: usize) -> Vec<Vec<T>> {
    _power(elements, vec![], length)
}

fn _power<T: Copy>(elements: &[T], partial_result: Vec<T>, depth: usize) -> Vec<Vec<T>> {
    if depth == 0 {
        return vec![partial_result];
    }

    let mut result: Vec<Vec<T>> = Vec::new();

    for elmt in elements {
        let mut with_elmt: Vec<T> = partial_result.clone();
        with_elmt.push(*elmt);

        result.extend(_power(elements, with_elmt, depth - 1))
    }

    result
}

/// Generate of combinations of two elements, with given number of elements.
///
/// Examples
/// ```
/// # use aoc24::toolbox::iterators::all_combinaisons_of_two;
///
/// let mut expected = Vec::from([
///     vec![0,0,1,1],
///     vec![0,1,0,1],
///     vec![1,0,0,1],
///     vec![0,1,1,0],
///     vec![1,0,1,0],
///     vec![1,1,0,0],
/// ]);
/// expected.sort();
///
/// let mut result = all_combinaisons_of_two(0, 2, 1, 2);
/// result.sort();
/// assert_eq!(result, expected)
///
/// ```
pub fn all_combinaisons_of_two<T: Copy>(elem1: T, nb1: usize, elem2: T, nb2: usize) -> Vec<Vec<T>> {
    all_combinations_of_two_rec(elem1, nb1, elem2, nb2, vec![])
}

fn all_combinations_of_two_rec<T: Copy>(
    elem1: T,
    nb1: usize,
    elem2: T,
    nb2: usize,
    partial_result: Vec<T>,
) -> Vec<Vec<T>> {
    if nb1 == 0 && nb2 == 0 {
        return vec![partial_result];
    }

    let mut result: Vec<Vec<T>> = Vec::new();

    if nb1 > 0 {
        let mut partial_result1: Vec<T> = partial_result.clone();
        partial_result1.push(elem1);
        result.extend(all_combinations_of_two_rec(
            elem1,
            nb1 - 1,
            elem2,
            nb2,
            partial_result1,
        ));
    }

    if nb2 > 0 {
        let mut partial_result2: Vec<T> = partial_result.clone();
        partial_result2.push(elem2);
        result.extend(all_combinations_of_two_rec(
            elem1,
            nb1,
            elem2,
            nb2 - 1,
            partial_result2,
        ));
    }

    result
}
