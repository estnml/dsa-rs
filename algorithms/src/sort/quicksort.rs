

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

pub fn quicksort_pv_first(arr: &mut [i32]) {
    if arr.len() >= 2 {
        let pivot_index = 0;
        let pivot_value = arr[pivot_index];
        let mut store_index = pivot_index + 1;

        for i in pivot_index + 1..arr.len() {
            if arr[i] <= pivot_value {
                // swap if i and store index are not pointing to same position
                if i != store_index {
                    let temp = arr[i];
                    arr[i] = arr[store_index];
                    arr[store_index] = temp;
                }

                store_index += 1;
            }
        }

        let temp = arr[pivot_index];
        arr[pivot_index] = arr[store_index - 1];
        arr[store_index - 1] = temp;

        quicksort_pv_first(&mut arr[..store_index - 1]);
        quicksort_pv_first(&mut arr[store_index..]);
    } else {
        return;
    }
}

