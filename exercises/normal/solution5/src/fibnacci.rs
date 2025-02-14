pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut sum = 0;
    for i in 0..=threshold {
if i % 2 != 0 && is_fib(i) {
    sum += i;
}
    }
    sum + 1
}
fn is_fib(num: u32) -> bool {
    let mut a = 0;
    let mut b = 1;
    while b <= num {
        if b == num {
            return true;
        }
        let temp = a;
        a = b;
        b = temp + b;
    }
    return false;
}