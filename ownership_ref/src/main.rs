/// Conclusion:
/// If you invoke Option<>.as_ref().unwrap() this creates a new owned variable without ownership transfer
#[allow(dead_code)]
fn simple_ownership() {

    let owner = Some(100_u8);
    let borrow_ref = owner.as_ref().unwrap();

    println!("owner = {}", owner.unwrap());
    println!("borrow_ref = {}", borrow_ref);
}

#[allow(dead_code)]
fn vector_ownership() {

    let owner = Some(vec![1,2,3,4]);
    let copy_owner = owner.as_ref().unwrap();

    println!("owner = {:?}", owner.as_ref().unwrap());
    println!("copy_owner = {:?}", copy_owner);
}

fn main() {
    vector_ownership();
}
