use std::fmt;

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

    pub fn not(&self) -> Self {
        Word {
            value: !self.value
        }
    }

    pub fn and(&self, word: &Word) -> Self {
        Word {
            value: self.value & word.value
        }
    }

    pub fn or(&self, word: &Word) -> Self {
        Word {
            value: self.value | word.value
        }
    }

    pub fn add(&self, word: &Word) -> Self {
        Word {
            value: self.value.wrapping_add(word.value)
        }
    }

    pub fn minus(&self) -> Self {
        Word {
            value: (!self.value).wrapping_add(0b0000_0000_0000_0001)
        }
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

        let a = a.not();
        assert_eq!(0b1111_1111_1111_1111, a.to_u16());
        assert_eq!(-1, a.to_i16());

        let a = a.minus();
        assert_eq!(0b0000_0000_0000_0001, a.to_u16());
        assert_eq!(1, a.to_i16());

        let a = a.or(&b);
        assert_eq!(0b0000_0000_0000_0101, a.to_u16());
        assert_eq!(5, a.to_i16());

        let a = a.add(&b);
        assert_eq!(0b0000_0000_0000_1010, a.to_u16());
        assert_eq!(10, a.to_i16());

        let a = a.and(&b);
        assert_eq!(0b0000_0000_0000_0000, a.to_u16());
        assert_eq!(0, a.to_i16());
    }
}