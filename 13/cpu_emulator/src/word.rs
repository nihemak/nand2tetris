use std::fmt;
use std::ops;

#[derive(Clone, Debug, Copy)]
pub struct Word {
    value: u16,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Binary for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#018b}", self.value)
    }
}

impl Word {
    pub fn new() -> Self {
        Word { value: 0b0000_0000_0000_0000 }
    }

    pub fn from(value: u16) -> Self {
        Word { value }
    }

    pub fn to_u16(&self) -> u16 {
        self.value
    }

    pub fn to_i16(&self) -> i16 {
        if self.value & 0b1000_0000_0000_0000 == 0 {
            self.value as i16
        }
        else {
            -((!self.value).wrapping_add(0b0000_0000_0000_0001) as i16)
        }
    }
}

impl ops::Not for Word {
    type Output = Self;

    fn not(self) -> Self::Output {
        Word {
            value: !self.value
        }
    }
}

impl ops::BitAnd for Word {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Word {
            value: self.value & rhs.value
        }
    }
}

impl ops::BitOr for Word {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Word {
            value: self.value | rhs.value
        }
    }
}

impl ops::Add for Word {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Word {
            value: self.value.wrapping_add(other.value)
        }
    }
}

impl ops::Neg for Word {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Word {
            value: (!self.value).wrapping_add(0b0000_0000_0000_0001)
        }
    }
}


impl ops::Sub for Word {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word() {
        let a = Word::new();
        let b = Word::from(0b0000_0000_0000_0101);

        assert_eq!(0b0000_0000_0000_0000, a.to_u16());
        assert_eq!(0b0000_0000_0000_0000, a.to_i16());
        assert_eq!(0b0000_0000_0000_0101, b.to_u16());
        assert_eq!(0b0000_0000_0000_0101, b.to_i16());

        let a = !a;
        assert_eq!(0b1111_1111_1111_1111, a.to_u16());
        assert_eq!(-1, a.to_i16());

        let a = -a;
        assert_eq!(0b0000_0000_0000_0001, a.to_u16());
        assert_eq!(1, a.to_i16());

        let a = a | b;
        assert_eq!(0b0000_0000_0000_0101, a.to_u16());
        assert_eq!(5, a.to_i16());

        let a = a + b;
        assert_eq!(0b0000_0000_0000_1010, a.to_u16());
        assert_eq!(10, a.to_i16());

        let a = a & b;
        assert_eq!(0b0000_0000_0000_0000, a.to_u16());
        assert_eq!(0, a.to_i16());

        let a = a - b;
        assert_eq!(0b1111_1111_1111_1011, a.to_u16());
        assert_eq!(-5, a.to_i16());
    }
}