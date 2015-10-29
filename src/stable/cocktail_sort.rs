pub fn cocktail_sort<T: PartialOrd>(slice: &mut [T]) {
    cocktail_sort_by(slice, PartialOrd::lt);
}

pub fn cocktail_sort_by<T, F>(slice: &mut [T], compare: F) where F: Fn(&T, &T) -> bool {
    for i in 0..slice.len()/2 {
        let mut complete_flag = true;
        for j in i..slice.len()-i-1 {
            if compare(&slice[j+1], &slice[j]) {
                slice.swap(j, j+1);
                complete_flag = false;
            }
        }
        if complete_flag {break;}
        else { complete_flag = true; }
        for j in (i..slice.len()-i-2).rev() {
            if compare(&slice[j+1], &slice[j]) {
                slice.swap(j, j+1);
                complete_flag = false;
            }
        }
        if complete_flag {break;}
    }
}
