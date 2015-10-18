pub fn heap_sort<T: PartialOrd>(slice: &mut [T]) {
    heap_sort_by(slice, PartialOrd::lt);
}

pub fn heap_sort_by<T, F>(slice: &mut [T], compare: F) where F: Fn(&T, &T) -> bool {
    build_heap(slice, &compare);
    for i in (1..slice.len()).rev() {
        slice.swap(0, i);
        heapify(&mut slice[0..i], 0usize, &compare);
    }
}

#[inline]
fn get_chrildren(index: usize) -> (usize, usize) {
    (index * 2 + 1, index * 2 + 2)
}

#[inline]
fn build_heap<T, F>(slice: &mut [T], compare: &F) where F: Fn(&T, &T) -> bool {
    for i in (0..slice.len()/2).rev() {
        heapify(slice, i, &compare);
    }
}

#[inline]
fn heapify<T, F>(slice: &mut [T], index: usize, compare: &F) where F: Fn(&T, &T) -> bool {
    let mut index = index;
    loop {
        let (l, r) = get_chrildren(index);
        if l >= slice.len() { break; }
        let smaller_index = 
            if r < slice.len() && compare(&slice[l], &slice[r]) { r }
            else { l };
        if compare(&slice[index], &slice[smaller_index]) {
            slice.swap(index, smaller_index);
            index = smaller_index;
        } else {
            break;
        }
    }
}

