use std::cell::RefCell;

pub fn scope() {
    let x = RefCell::new(true);
    let _ = *x.borrow_mut() && *x.borrow_mut();
}

pub fn scope_expanded() {
    let x = RefCell::new(true);
    {
        let t1 = x.borrow_mut();

        *t1 && {
            let t2 = x.borrow_mut();
            *t2
        }
    };
}

pub fn scope_fixed() {
    let x = RefCell::new(true);
    let a = *x.borrow_mut();
    let _ = a && *x.borrow_mut();
}
