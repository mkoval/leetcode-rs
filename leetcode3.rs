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

const RADIX: i32 = 10;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x < 0 {
            -Self::reverse(-x)
        } else {
            let (output, _) = Digits::new(x, RADIX)
                .collect::<Vec<i32>>()
                .iter()
                .rev()
                .fold((0, 1), |(output, base), digit|
                    (output + base * digit, RADIX * base));
            output
        }
    }
}
