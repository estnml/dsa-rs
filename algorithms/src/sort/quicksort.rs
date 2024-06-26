// ! after partitioning procedure, pivot element is on the sorted position.
// ! elements on the left side of the pivot are smaller.
// ! elements on the right side of the pivot are greater.
// ! apply the partitioning procedure for both sides recursively.
// ! there must be at least 2 elements to apply partitioning procedure.

// roadmap:
// TODO: 1) implement base quick sort algorithm for i32 type. (+)
// TODO: 2)? implement some algorithms for selecting the pivot element and analyse. (-)
// TODO: 3) make implementation generic. (-)
// TODO: 4) adapt algorithm for usage with traits



pub fn quicksort_dsa_rs<T>(arr: &mut [T])
where
    T: Ord + Copy,
{
    if arr.len() >= 2 {
        
        let pivot_value = arr[arr.len() / 2];

        // move pivot to the first position
        arr.swap(0, arr.len() / 2);

        let mut swap_index = 1;
        for i in 1..arr.len() {
            if arr[i] <= pivot_value {
                if i != swap_index {
                    // swap i with swapindex
                    arr.swap(i, swap_index);
                }

                swap_index += 1;
            }
        }

        // swap pivot with swapindex - 1
        if swap_index - 1 != 0 {
            arr.swap(0, swap_index - 1);
        }

        quicksort_dsa_rs(&mut arr[..swap_index - 1]);
        quicksort_dsa_rs(&mut arr[swap_index..]);
    } else {
        return;
    }
}
