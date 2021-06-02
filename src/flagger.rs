use std::str::FromStr;

use clap::Clap;

#[derive(Clap, Clone, Debug, PartialEq)]
pub enum Representation {
    Hex,
    HexUp,
    Binary,
}

impl FromStr for Representation {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Representation::Hex),
            "b" => Ok(Representation::Binary),
            "X" => Ok(Representation::HexUp),
            _ => Err("no match"),
        }
    }
}

pub fn format_output(number: usize, representation: Representation) -> String {
    match representation {
        Representation::Hex => {
            format!("{:#x}", number)
        }

        Representation::HexUp => {
            format!("{:#X}", number)
        }
        Representation::Binary => {
            format!("{:#b}", number)
        }
    }
}

pub mod commands {
    ///"Fill" a number with a sequence of binary 1s like 0b1111
    /// Examples:
    /// ```
    /// use flagger::fill;
    /// const LOWER_4: usize = fill(4);
    /// assert_eq!(LOWER_4, 0b1111)
    /// ```
    pub const fn fill(count: usize) -> usize {
        if count == 0 {
            0
        } else {
            (1 << count) - 1
        }
    }

    /// return a flag with a single bit set
    /// Examples:
    /// ```
    /// use flagger::set;
    /// const THIRD_BIT_SET: usize = set(3);
    /// assert_eq!(THIRD_BIT_SET, 0b0100)
    /// ```
    pub const fn set(count: usize) -> usize {
        if count == 0 {
            return 0;
        }
        1 << (count - 1)
    }

    /// return the inverse of having set a single bit
    /// requires the desired width of the number to be known
    /// Examples:
    /// ```
    /// use flagger::invert;
    /// const ALL_BUT_THIRD_BIT_SET: usize = invert(3, 4);
    /// assert_eq!(ALL_BUT_THIRD_BIT_SET, 0b1011)
    /// ```
    pub const fn invert(count: usize, width: usize) -> usize {
        if count == 0 {
            return 0;
        }
        fill(width) ^ (1 << (count - 1))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::flagger::commands::fill;

    #[test]
    fn it_fills_binary_numbers() {
        assert_eq!(fill(0), 0b0 as usize);
        assert_eq!(fill(1), 0b1 as usize);
        assert_eq!(fill(2), 0b11 as usize);
        assert_eq!(fill(3), 0b111 as usize);
        assert_eq!(fill(4), 0b1111 as usize);
    }

    #[test]
    fn it_fills_binary_numbers_for_powers_of_two() {
        for i in 1..32 {
            let number = 1 << i;
            assert_eq!(commands::fill(i), number - 1);
        }
    }

    #[test]
    fn it_sets_a_binary_digit() {
        assert_eq!(commands::set(0), 0);
        for i in 1..32 {
            assert_eq!(commands::set(i), 1 << (i - 1));
        }
    }

    #[test]
    fn it_inverts_digits() {
        //it is 1-indexed
        assert_eq!(commands::invert(0, 0), 0);
        assert_eq!(commands::invert(0, 1), 0);

        assert_eq!(commands::invert(1, 4), 0b1110);
        assert_eq!(commands::invert(2, 4), 0b1101);
        assert_eq!(commands::invert(3, 4), 0b1011);
        assert_eq!(commands::invert(4, 4), 0b0111);
    }

    #[test]
    fn it_demonstrates_that_inverting_and_setting_together_form_fill() {
        let width: usize = 32;
        for i in 1..width {
            let ored = commands::invert(i, width) | commands::set(i);
            assert_eq!(ored, commands::fill(width));
        }
    }

    #[test]
    fn it_formats_output() {
        assert_eq!(format_output(1, Representation::Binary), "0b1");
        assert_eq!(format_output(2, Representation::Binary), "0b10");
        assert_eq!(format_output(3, Representation::Binary), "0b11");
        assert_eq!(format_output(4, Representation::Binary), "0b100");
        //same as decimal for 1 to 9
        for i in 1..9 {
            assert_eq!(format_output(i, Representation::Hex), format!("0x{}", i));
        }
    }

    #[test]
    fn it_parses_representations_from_string() {
        assert_eq!(Representation::from_str("x"), Ok(Representation::Hex));
        assert_eq!(Representation::from_str("X"), Ok(Representation::HexUp));
        assert_eq!(Representation::from_str("b"), Ok(Representation::Binary));
    }
}
