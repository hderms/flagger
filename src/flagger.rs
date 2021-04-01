use std::str::FromStr;

use clap::Clap;

#[derive(Clap, Clone)]
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
    use super::*;
    pub fn fill_command(count: usize) -> usize {
        fill(count)
    }

    pub fn set_command(count: usize) -> usize {
        1 << (count - 1)
    }
    pub fn invert_command(count: usize, width: usize) -> usize {
        fill(width) ^ (1 << (count - 1))
    }
}
pub fn fill(count: usize) -> usize {
    let mut num: usize = 0;
    for i in 0..count {
        num |= 1 << i;
    }
    num
}
