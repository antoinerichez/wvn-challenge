fn max_area(height: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, height.len() - 1);
    let mut max_area = 0;

    while left < right {
        let h = height[left].min(height[right]);
        let w = (right - left) as i32;
        max_area = max_area.max(h * w);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}

fn test(input: Vec<i32>, expected: i32) {
    let result = max_area(input);
    println!("{:?} == {} ? {}", result, expected, result == expected)
}


pub fn execute() -> () {
    test(vec![1,8,6,2,5,4,8,3,7], 49);
    test(vec![2,1,8,6,4,6,5,5], 25);
    test(vec![2,1,8,6,4,6,5,5], 25);
    test(vec![1,1], 1)
}