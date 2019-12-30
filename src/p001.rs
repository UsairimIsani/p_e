//If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
//Find the sum of all the multiples of 3 or 5 below 1000.
//                              | |
//  _ __   __ _ ___ ___  ___  __| |
// | '_ \ / _` / __/ __|/ _ \/ _` |
// | |_) | (_| \__ \__ \  __/ (_| |
// | .__/ \__,_|___/___/\___|\__,_|
// | |
// |_|
pub fn p001(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum = sum + i;
        }
    }
    sum
}
#[cfg(test)]
#[test]
fn test_prob_1() {
    assert_eq!(23, p001(10));
    println!("{}", p001(1000));
}
