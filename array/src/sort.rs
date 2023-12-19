
fn _merge<T: Ord + Copy>(arr: &mut [T], aux: &mut [T]) {
    aux.clone_from_slice(arr);
    let n = arr.len();
    let mid = arr.len() / 2;
    let (mut i, mut j) = (aux[..mid].iter(), aux[mid..].iter());
    let mut k = arr.iter_mut();
    while let (Some(&a), Some(&b)) = (i.clone().next(), j.clone().next()) {
        if let Some(value) = k.next() {
            if a <= b {
                *value = a.clone();
                i.next();
            } else {
                *value = b.clone();
                j.next();
            }
        }
    }
    let x = n - k.len();
    if i.len() > 0 {
        arr[x..].clone_from_slice(i.as_slice());
    }
    if j.len() > 0 {
        arr[x..].clone_from_slice(j.as_slice());
    }
}

pub fn _merge_sort<T: Ord + Copy>(arr: &mut [T], aux: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);
    let (aux_left, aux_right) = aux.split_at_mut(mid);
    _merge_sort(left, aux_left);
    _merge_sort(right, aux_right);
    _merge(arr, aux);
}

