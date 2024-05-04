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

pub fn quicksort_dsa_rs(arr: &mut [i32]) {
    if arr.len() >= 2 {
        let pivot_value = arr[arr.len() / 2];

        // swap first element with pivot
        let temp = arr[0];
        arr[0] = pivot_value;
        arr[arr.len() / 2] = temp;

        let mut swap_index = 1;
        for i in 1..arr.len() {
            if arr[i] <= pivot_value {
                if i != swap_index {
                    // swap i with swapindex
                    let temp = arr[i];
                    arr[i] = arr[swap_index];
                    arr[swap_index] = temp;
                }

                swap_index += 1;
            }
        }

        // swap pivot with swapindex - 1
        let temp = arr[swap_index - 1];
        arr[swap_index - 1] = arr[0];
        arr[0] = temp;

        quicksort_dsa_rs(&mut arr[..swap_index - 1]);
        quicksort_dsa_rs(&mut arr[swap_index..]);
    } else {
        return;
    }
}
