use dispatchtable::{DispatchTable, Dispatch};

fn add(params: &(isize, isize)) -> isize {
    params.0 + params.1
}

fn sub(params: &(isize, isize)) -> isize {
    params.0 + params.1
}


fn main() {
    let mut table = DispatchTable::new();
    table.insert("add", Box::new(add));
    table.insert("sub", Box::new(sub));

    assert_eq!(table.call(&"add", &(1, 2)), 3);
    assert_eq!(table.call(&"sub", &(5, 2)), 3);
}