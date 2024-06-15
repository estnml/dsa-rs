pub fn binary_search_dsa_rs1(search_list: &[i32], search_item: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = search_list.len() - 1;
    let mut mid = 0;

    while low < high {
        mid = (high + low) / 2;
        if search_list[mid] == search_item {
            return Some(mid);
        } else if search_item < search_list[mid] {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    None
}

fn binary_search_2(search_list: &[i32], search_item: i32) {}
