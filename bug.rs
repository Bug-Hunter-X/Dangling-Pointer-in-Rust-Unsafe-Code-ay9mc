fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    // ... some code that modifies the vector ...
    unsafe {
        // Incorrect: Dereferencing ptr after v is dropped
        println!("{:?}", *ptr);
    }
}