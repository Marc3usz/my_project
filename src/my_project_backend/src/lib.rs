use std::cell::RefCell;

thread_local! {
    static MSG: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[ic_cdk::query]
fn read_msg() -> Vec<String> {
    MSG.with(|chat| (*chat.borrow()).clone())
}

#[ic_cdk::update]
fn add_msg(msg: String) {
    MSG.with(|chat| chat.borrow_mut().push(msg));
}
