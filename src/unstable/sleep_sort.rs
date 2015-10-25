extern crate num;

use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use self::num::{Unsigned, ToPrimitive};


pub fn sleep_sort<T: Unsigned>(slice: &mut [T])
    where T: Clone + Send + ToPrimitive + 'static {
    sleep_sort_by(slice, |x| num::traits::NumCast::from((*x).clone()).unwrap_or(0));
}

pub fn sleep_sort_by<T, F>(slice: &mut [T], convert: F)
    where T: Clone + Send + 'static, F: Fn(&T) -> u32 + Send + Sync + 'static {
    let convert = Arc::new(convert);
    let (tx, rx) = mpsc::channel();
    for i in slice.iter() {
        let (i, tx, convert) = ((*i).clone(), tx.clone(), convert.clone());
        thread::spawn(move || {
            thread::sleep_ms((*convert)(&i));
            tx.send(i).ok().expect("Send failed");
        });
    }
    
    for i in slice.iter_mut() {
        *i = rx.recv().ok().expect("Receive failed");
    }
}
