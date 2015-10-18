pub fn bubble_sort<T: PartialOrd>(slice: &mut [T]) {
    bubble_sort_by(slice, PartialOrd::lt);
}

pub fn bubble_sort_by<T, F>(slice: &mut [T], compare: F) where F: Fn(&T, &T) -> bool {
    for i in (1..slice.len()) {
        let mut complete_flag = true;
        for j in (0..slice.len()-i) {
            if compare(&slice[j+1], &slice[j]) {
                slice.swap(j, j+1);
                complete_flag = false;
            }
        }
        if complete_flag { break; }
    }
}
