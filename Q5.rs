fn find_median(nums: &[i32]) -> f64 {
    let len = nums.len();
    if len == 0 {
        panic!("Input array is empty");
    }
    if len % 2 == 0 {
    
        let mid1 = nums[len / 2 - 1] as f64;
        let mid2 = nums[len / 2] as f64;
        return (mid1 + mid2) / 2.0;
    } else {
    
        return nums[len / 2] as f64;
    }
}

fn main() {
    let nums1 = vec![1, 2, 3, 4, 5];
    let nums2 = vec![1, 2, 3, 4, 5, 6];
    let median1 = find_median(&nums1);
    let median2 = find_median(&nums2);
    println!("Median of {:?} is {}", nums1, median1);
    println!("Median of {:?} is {}", nums2, median2);
}
 