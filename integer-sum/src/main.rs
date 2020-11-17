fn sum_integer_sets(t : &[u32]) -> Option<u32> {
    if t.len() > 0 {
        let mut total : u32 = 0;
        for item in t.iter() {
            match total.checked_add(*item) {
                Some(s) => total = s,
                None => return None,
            }
        }
        Some(total)
    } else {
        None
    }
}


fn main() {
    let arr = vec![1,2,3,4,5,6,7,8,9,10];
    println!("sum of array is:{:?}", sum_integer_sets(&arr));
}
