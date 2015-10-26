use std::ptr::{replace, read};

pub fn insertion_sort<T: PartialOrd>(slice: &mut [T]) {
    insertion_sort_by(slice, PartialOrd::lt);
}

pub fn insertion_sort_by<T, F>(slice: &mut [T], compare: F) where F: Fn(&T, &T) -> bool {
    for i in (1..slice.len()) {
        let insert_idx = find_insert_idx(slice, i, &compare);
        insert(slice, i, insert_idx);
    }
}

#[inline]
fn find_insert_idx<T, F>(slice: &[T], elem_idx: usize, compare: &F) -> usize where F: Fn(&T, &T) -> bool {
    let mut insert_idx = elem_idx;
    while insert_idx > 0 {
        if compare(&slice[insert_idx - 1], &slice[elem_idx]) {
            break;
        }
        insert_idx -= 1;
    }
    return insert_idx;
}


#[inline]
fn insert<T>(slice: &mut [T], elem_idx: usize, insert_idx: usize) {
    let slice_ptr = slice.as_mut_ptr();
    let elem: T;
    unsafe { elem = read(slice_ptr.offset(elem_idx as isize)); } 
    for i in (insert_idx..elem_idx).rev() {
        unsafe {
            replace(slice_ptr.offset((i+1) as isize), read(slice_ptr.offset(i as isize)));
        }
    }
    unsafe { replace(slice_ptr.offset(insert_idx as isize), elem); }
}
