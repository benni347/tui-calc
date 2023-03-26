pub fn divide(x: i128, y: i128) -> i128 {
    let a = x.abs() as u128;
    let b = y.abs() as u128;

    if a == 0 || b == 0 {
        return 0;
    }

    let n = 64 - (a.max(b)).leading_zeros();
    let m = (n + 31) / 32;
    let mut u = vec![0u32; m as usize];
    let mut v = vec![0u32; m as usize];

    for i in 0..m {
        u[i as usize] = (a >> (32 * i)) as u32;
        v[i as usize] = (b >> (32 * i)) as u32;
    }

    let mut w = vec![0u32; 2 * m as usize];
    for i in 0..m {
        let mut carry = 0;
        for j in 0..m {
            let t = u[i as usize] as u128 * v[j as usize] as u128
                + w[i as usize + j as usize] as u128
                + carry as u128;
            w[i as usize + j as usize] = t as u32;
            carry = (t >> 32) as u32;
        }
        w[i as usize + m as usize] = carry;
    }

    while w.len() > 1 && w.last().unwrap() == &0 {
        w.pop();
    }

    let mut result = 0i128;
    let mut carry = 0i128;
    for i in (0..w.len()).rev() {
        let t = result as u128 * (1u128 << 32) + w[i] as u128 + carry as u128;
        result = (t & 0x7fff_ffff) as i128;
        carry = (t >> 31) as i128;
    }

    if (x < 0) != (y < 0) {
        -result
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_burnikel_ziegler() {
        assert_eq!(divide(0, 0), 0);
        assert_eq!(divide(0, 5), 0);
        assert_eq!(divide(5, 0), 0);
        assert_eq!(divide(1, 1), 1);
        assert_eq!(divide(123, 456), 56088);
        assert_eq!(divide(-123, 456), -56088);
        assert_eq!(divide(123, -456), -56088);
        assert_eq!(divide(-123, -456), 56088);
        assert_eq!(
            divide(123456789123456789, 987654321987654321,),
            121932631137021795634352628728159141,
        );
    }
}
