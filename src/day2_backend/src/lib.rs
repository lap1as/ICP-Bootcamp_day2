use std::cell::RefCell;

thread_local! {
    static ENTRIES: RefCell<Vec<String>> = RefCell::default();
}

#[ic_cdk::query]
fn greet(name: String, last_name: i8) -> String {
    format!("Hello, {} {}!", name, last_name)
}

#[ic_cdk::update]
fn add_entry(entry: String) {
    ENTRIES.with(|entries| {
        entries.borrow_mut().push(entry)
    });
}

#[ic_cdk::query]
fn get_entries() -> Vec<String> {
    ENTRIES.with(|entries|{
        entries.borrow().clone()
    })
}