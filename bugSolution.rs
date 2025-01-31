fn main() {
    let mut v = vec![1, 2, 3];
    let len = v.len();
    let v_clone = v.clone(); // Create a copy to work with
    let ptr = v_clone.as_ptr();
    // ... some code that modifies v_clone ...
    unsafe {
        // Safe: v_clone is still in scope
        for i in 0..len {
            println!("{:?}", *ptr.add(i));
        }
    }
} 