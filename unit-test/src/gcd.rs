// 太难了
pub fn swap<T: Clone>(a: &mut T, b: &mut T) {
    let t = a.clone();
    *a = b.clone();
    *b = t;
}

// 大数除以小的树 取余 重复操作 直到为0 返回除数
pub fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if n > m {
            swap::<u64>(&mut m, &mut n);
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11)
}

// // 会改变原始值
// pub fn gcd1(m: &mut u64, n: &mut u64) -> u64 {
//     assert!(*n != 0 && *m != 0);
//     while *m != 0 {
//         if *n > *m {
//             swap(m, n);
//         }
//         *m = *m % *n;
//     }
//     *n
// }
