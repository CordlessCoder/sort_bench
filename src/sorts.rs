pub trait SortingMethod<T> {
    fn name() -> String;
    fn stable() -> bool;
    fn sort(data: &mut [T]);
}

pub struct RustStdSort;
impl<T: Ord> SortingMethod<T> for RustStdSort {
    fn name() -> String {
        "Rust Standard library".to_string()
    }
    fn stable() -> bool {
        false
    }
    fn sort(data: &mut [T]) {
        data.sort()
    }
}

pub struct QuickSort;
impl<T: PartialOrd> SortingMethod<T> for QuickSort {
    fn name() -> String {
        "Verq's Quicksort".to_string()
    }
    fn stable() -> bool {
        false
    }
    fn sort(data: &mut [T]) {
        quickersort(data)
    }
}

pub struct BubbleSort;
impl<T: PartialOrd> SortingMethod<T> for BubbleSort {
    fn name() -> String {
        "Basic bubblesort".to_string()
    }
    fn stable() -> bool {
        false
    }
    fn sort(data: &mut [T]) {
        bubble_sort(data)
    }
}

pub struct InsertionSort;
impl<T: PartialOrd> SortingMethod<T> for InsertionSort {
    fn name() -> String {
        "Basic insertion sort".to_string()
    }
    fn stable() -> bool {
        false
    }
    fn sort(data: &mut [T]) {
        insertion_sort(data)
    }
}

/// Quicksort falling back to insertion sort for shorter slices
fn quickersort<T: PartialOrd>(mut data: &mut [T]) {
    // An array of length <= 1 is always sorted
    loop {
        // Use insertion sort for anything shorter than 20 elements
        if data.len() <= 20 {
            if data.len() >= 2 {
                insertion_sort(data);
            }
            return;
        }
        // Partition array and get the pivot index
        let pivot = partition(data);
        let right;
        (data, right) = data.split_at_mut(pivot);
        // Recurse into the shorter side to optimize for stack space
        if data.len() < right.len() {
            quickersort(data);
            data = right;
        } else {
            quickersort(right);
        }
    }

    #[inline]
    fn partition<T: PartialOrd>(data: &mut [T]) -> usize {
        if data.len() < 2 {
            return 1;
        };
        let (data, pivot) = data.split_at_mut(data.len() - 1);
        let pivot = &mut pivot[0];

        let mut slow = 0;
        if data[0] <= *pivot {
            slow += 1;
        }
        for fast in 1..data.len() {
            if data[fast] <= *pivot {
                // SAFETY: slow starts with a value of 0 | 1, fast starts at 1, every iteration
                // fast += 1 and slow += 0 or 1, therefore for every iteration slow <= fast holds.
                // fast is proven to be in-bounds on the previous line(checked indexing)
                unsafe { swap_unchecked(data, slow, fast) };
                slow += 1;
            }
        }
        if slow != data.len() {
            std::mem::swap(&mut data[slow], pivot);
        }
        return slow;
    }

    unsafe fn swap_unchecked<T>(data: &mut [T], idx1: usize, idx2: usize) {
        let ptr = data.as_mut_ptr();
        std::ptr::swap(ptr.add(idx1), ptr.add(idx2));
    }
}
fn insertion_sort<T: PartialOrd>(data: &mut [T]) {
    // This makes the upper bound of i the last valid index
    for i in 1..data.len() {
        // This makes the upper bound of j = i - 1
        for j in (0..i).rev() {
            unsafe {
                // SAFETY: at the upper bound, j + 1 = (i - 1) + 1 = i, and i is
                // known to be a valid index
                if data.get_unchecked(j) >= data.get_unchecked(j + 1) {
                    swap_unchecked(data, j, j + 1)
                } else {
                    break;
                }
            }
        }
    }
    unsafe fn swap_unchecked<T>(data: &mut [T], idx1: usize, idx2: usize) {
        let ptr = data.as_mut_ptr();
        std::ptr::swap(ptr.add(idx1), ptr.add(idx2));
    }
}

pub fn bubble_sort<T: PartialOrd>(data: &mut [T]) {
    for end in (0..data.len()).rev() {
        for i in 0..end {
            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
            }
        }
    }
}
