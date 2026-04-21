fn binary_search(arr : &[i32], val : i32) -> i32 {
    let mut l : i32 = 0;
    let mut r : i32 = arr.len() as i32 -1;

    while l <= r{
        let mid : i32 = l + (r - l) / 2; //find the mid point
        
        if arr[mid as usize] == val {
            return mid;
        }else if arr[mid as usize] < val {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }
    return -1;

}

fn main() {
    println!("Hello, world!");
    let array = [1, 2, 3, 4, 5, 6, 7];
    let val = 5;
    let index : i32 = binary_search(&array, val);
    println!("result = {index}");
}

