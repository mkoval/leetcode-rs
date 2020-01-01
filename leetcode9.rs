struct Digits {
    radix: i32,
    val: i32,
}

impl Digits {
    pub fn new(val: i32, radix: i32) -> Self {
        assert!(val >= 0);
        assert!(radix > 1);
        Self{val: val, radix: radix}
    }
}

impl Iterator for Digits {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.val > 0 {
            let digit = self.val % self.radix;
            self.val /= self.radix;
            Some(digit)
        } else {
            None
        }
    }
}


impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        const radix: i32 = 10;
        
        if x < 0 {
            return false;
        }
        let digits: Vec<i32> = Digits::new(x, radix).collect();
        let half_len = (digits.len() + 1) / 2;
        digits.iter()
            .take(half_len)
            .zip(digits.iter().rev())
            .all(|(d1, d2)| d1 == d2)
    }
}
