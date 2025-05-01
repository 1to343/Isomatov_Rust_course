#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    // TODO: your code goes here.
    // unimplemented!()
    if k == 0 {
        return vec![vec![]];
    }
    if arr.len() < k {
        return vec![];
    }
    let choosen = arr[0];
    let include_first = combinations(&arr[1..], k - 1)
        .iter()
        .map(|v| {
            let mut elem = vec![choosen];
            elem.extend_from_slice(v.as_slice());
            elem
        })
        .collect::<Vec<Vec<i32>>>();

    let exclude_first = combinations(&arr[1..], k);
    let mut result = include_first;
    result.extend_from_slice(exclude_first.as_slice());
    result
}
