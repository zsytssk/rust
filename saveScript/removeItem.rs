fn main() {
    let mut arr = vec![4, 2, 3];
    let mut index = 0;
    let match_val = 4;

    {
        for (i, item) in arr.iter().enumerate() {
            if item == &match_val {
                index = i;
            }
        }
    }
    arr.remove(index);
    println!("{:?}", arr);
}
