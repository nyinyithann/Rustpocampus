fn main() {
    // ANCHOR: into_iter
    let v1 = vec![1,2,3,4,5];
    for v in v1 {
        println!("{v}");
    }
    // error cause v1 is moved into the loop.
    // loop takes ownership of v1
    // println!("{:?}", v1);
      
    let v1 = vec![1,2,3,4,5];
    let mut iter = v1.into_iter();
    while let Some(i) = iter.next() {
        println!("{i}");
    }
    // ANCHOR_END: into_iter

    
    // let v1 = vec![1,2,3,4,5];
    // for v in v2 {
    //     
    // }
}
