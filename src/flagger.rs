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

fn fill(count: usize) -> usize {
    let mut num: usize = 0;
    for i in 0..count {
        num |= 1 << i;
    }
    num
}

pub mod commands {
    use super::*;
    pub fn fill_command(count: usize) -> usize {
        fill(count)
    }

    pub fn set_command(count: usize) -> usize {
        if count == 0 {
            return 0;
        }
        1 << (count - 1)
    }
    pub fn invert_command(count: usize, width: usize) -> usize {
        if count == 0 {
            return 0;
        }
        fill(width) ^ (1 << (count - 1))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_fills_binary_numbers() {
        assert_eq!(commands::fill_command(0), 0b0 as usize);
        assert_eq!(commands::fill_command(1), 0b1 as usize);
        assert_eq!(commands::fill_command(2), 0b11 as usize);
        assert_eq!(commands::fill_command(3), 0b111 as usize);
        assert_eq!(commands::fill_command(4), 0b1111 as usize);
    }
    #[test]
    fn it_fills_binary_numbers_for_powers_of_two() {
        for i in 1..32 {
            let number = 1 << i;
            assert_eq!(commands::fill_command(i), number - 1);
        }
    }

    #[test]
    fn it_sets_a_binary_digit() {
        assert_eq!(commands::set_command(0), 0);
        for i in 1..32 {
            assert_eq!(commands::set_command(i), 1 << (i - 1));
        }
    }

    #[test]
    fn it_inverts_digits() {
        //it is 1-indexed
        assert_eq!(commands::invert_command(0, 0), 0);
        assert_eq!(commands::invert_command(0, 1), 0);

        assert_eq!(commands::invert_command(1, 4), 0b1110);
        assert_eq!(commands::invert_command(2, 4), 0b1101);
        assert_eq!(commands::invert_command(3, 4), 0b1011);
        assert_eq!(commands::invert_command(4, 4), 0b0111);
    }

    #[test]
    fn it_demonstrates_that_inverting_and_setting_together_form_fill() {
        let width: usize = 32;
        for i in 1..width {
            let ored = commands::invert_command(i, width) | commands::set_command(i);
            assert_eq!(ored, commands::fill_command(width));
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
