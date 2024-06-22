// TODO: base implementation (+)
pub fn selectionsort_dsa_rs(arr: &mut [i32]) {
    

    let mut temp: i32 = 0;

    for i in 0..arr.len() - 1 {

        let mut min_val: i32 = arr[i];
        let mut min_index: usize = i;

        for j in i + 1..arr.len() {
            if arr[j] < min_val {
                min_val = arr[j];
                min_index = j;
            }
        }

        temp = arr[i];
        arr[i] = arr[min_index];
        arr[min_index] = temp;
    }
}
