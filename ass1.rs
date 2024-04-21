#[allow(dead_code)]
pub fn quicksort<T: Ord>(arr: &mut [T]) {
    _quicksort(arr, 0, (arr.len() - 1) as isize);
}

#[allow(dead_code)]
fn _quicksort<T: Ord>(arr: &mut [T], left: isize, right: isize) {
    if left <= right {
        let partition_idx = partition(arr, 0, right);

        _quicksort(arr, left, partition_idx - 1);
        _quicksort(arr, partition_idx + 1, right);
    }
}

#[allow(dead_code)]
fn partition<T: Ord>(arr: &mut [T], left: isize, right: isize) -> isize {
    let pivot = right;
    let mut i: isize = left as isize - 1;

    for j in left..=right - 1 {
        if arr[j as usize] <= arr[pivot as usize] {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, pivot as usize);

    i + 1
}

#[allow(dead_code)]
fn selection_sort(list: &mut [i64]) {
  for i in 0..list.len() {
    let mut small = i;
    for j in (i + 1)..list.len() {
      if list[j] < list[small] {
        small = j;
      }
    }
    list.swap(small, i);
  }
}


#[allow(dead_code)]
fn insertion_sort<T: Ord>(list: &mut [T]) {
  for i in 1..list.len() {
    for j in (1..i + 1).rev() {
      if list[j - 1] <= list[j] { break; }
      list.swap(j - 1, j)
    }
  }
}