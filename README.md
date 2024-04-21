## Sorting library

# Usage
This Rust library provides implementations of various sorting algorithms, including:

- Quicksort
- Selection Sort
- Insertion Sort

To use this library in your Rust project, add the following to your Cargo.toml file:
```
[dependencies]
sorting_algorithms = { git = "https://github.com/stagepalete/bt2_ass1.git" }
```

Then, in your Rust code, you can import and use the sorting algorithms like this:

```
use sorting_algorithms::{quicksort, selection_sort, insertion_sort};

fn main() {
    let mut numbers = vec![4, 2, 3, 1];
    
    // Example: Using Quicksort
    quicksort(&mut numbers);
    println!("Sorted numbers using Quicksort: {:?}", numbers);
    
    // Example: Using Selection Sort
    selection_sort(&mut numbers);
    println!("Sorted numbers using Selection Sort: {:?}", numbers);
    
    // Example: Using Insertion Sort
    insertion_sort(&mut numbers);
    println!("Sorted numbers using Insertion Sort: {:?}", numbers);
}
```

# Examples

```
use sorting_algorithms::quicksort;

fn main() {
    let mut numbers = vec![4, 2, 3, 1];
    
    // Using Quicksort
    quicksort(&mut numbers);
    println!("Sorted numbers using Quicksort: {:?}", numbers);
}
```

```
use sorting_algorithms::selection_sort;

fn main() {
    let mut numbers = vec![4, 2, 3, 1];
    
    // Using Selection Sort
    selection_sort(&mut numbers);
    println!("Sorted numbers using Selection Sort: {:?}", numbers);
}
```

```
use sorting_algorithms::insertion_sort;

fn main() {
    let mut numbers = vec![4, 2, 3, 1];
    
    // Using Insertion Sort
    insertion_sort(&mut numbers);
    println!("Sorted numbers using Insertion Sort: {:?}", numbers);
}
```

