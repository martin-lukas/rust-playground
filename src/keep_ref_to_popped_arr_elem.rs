#[test]
fn keep_ref_to_popped_arr_elem() {
    #[derive(Debug)]
    struct Foo {}

    let mut vector = vec![Foo {}, Foo {}, Foo {}];
    let last_foo = vector.last();

    vector.pop();

    // I can't have immutable and mutable borrow at the same time.
    // If I try to borrow immutable borrow (ref), and then mutable borrow with pop,
    // I can't use again the immutable one - only one imm/m borrow at a time.

    // println!("last foo {:?}", last_foo);
}