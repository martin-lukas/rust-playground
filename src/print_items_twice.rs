fn print_out_items(items: &Vec<u32>) {
    for i in items {
        println!("{}", i);
    }
}

#[test]
fn test_print_out_items_twice() {
    let items = vec![1, 2, 3];
    // Pass it in as an immutable reference (not mutable NOR borrowed value)
    print_out_items(&items);
    print_out_items(&items);
}