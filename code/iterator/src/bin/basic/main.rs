fn main() {
    // ANCHOR: into_iter
    let v1 = vec![1, 2, 3, 4, 5];
    for v in v1 {
        println!("{v}");
    }

    // the following line will give error cause v1 is moved into the loop.
    // loop takes ownership of v1
    // println!("{:?}", v1);

    // same as above
    let v1 = vec![1, 2, 3, 4, 5];
    let mut iter = v1.into_iter();
    while let Some(i) = iter.next() {
        println!("{i}");
    }
    // ANCHOR_END: into_iter

    
    // ANCHOR: iter
    let v1 = vec![1, 2, 3, 4, 5];
    for v in &v1 {
        println!("{v}");
    }
    // v1 is still valid here cause the loop iterates over &v1
    println!("{v1:?}");
    
    // same as above, but using iter method
    let v1 = vec![1, 2, 3, 4, 5];
    for v in v1.iter() {
        println!("{v}");
    }
    // v1 is still valid here cause the loop iterates over &v1
    println!("{v1:?}");
    // ANCHOR_END: iter

    // ANCHOR: iter_mut
    let mut v1 = vec![1, 2, 3, 4, 5];
    for v in &mut v1 {
        *v = *v + *v
    }
    println!("{v1:?}"); // =>  [2, 4, 5, 8, 10]
    
    // same as above but using inter_mut method
    let mut v1 = vec![1, 2, 3, 4, 5];
    for v in v1.iter_mut() {
        *v = *v + *v
    }
    println!("{v1:?}"); // =>  [2, 4, 5, 8, 10]
    // ANCHOR_END: iter_mut
    
}
