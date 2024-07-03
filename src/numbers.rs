use anyhow::Result;
use fraction::{Fraction, ToPrimitive};
use regex::Regex;
use std::str::FromStr;

pub fn parse_number(input: &str) -> Result<f32> {
    // This feels very hacky and ugly and I'm too lazy to figure out a better way
    
    let mut input = input.to_string();
    if !input.is_ascii() {
        let mut num = 0.0;
        let vulgar_regex = Regex::new("([½¼¾⅐⅑⅒⅓⅔⅕⅖⅗⅘])")?;
        if vulgar_regex.is_match(&input) {
            if let Some(c) = vulgar_regex.captures(&input) {
                if let Some(n) = c.get(1) {
                    num += Fraction::from_unicode_str(n.as_str())?
                        .to_f32()
                        .unwrap_or(0.0);
                }
            }
            input = vulgar_regex
                .replace_all(&input, "")
                .to_string()
                .trim()
                .to_owned();
        }

        if !input.is_empty() {
            num += Fraction::from_unicode_str(&input)?
                .to_f32()
                .unwrap_or_default();
        }

        return Ok(num);
    }

    Ok(Fraction::from_str(&input)?.to_f32().unwrap_or(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() -> Result<()> {
        let input = "1";

        assert_eq!(parse_number(input)?, 1.0);

        Ok(())
    }

    #[test]
    fn mixed_vulgar() -> Result<()> {
        let input = "½";

        assert_eq!(parse_number(input)?, 0.5);

        Ok(())
    }

    #[test]
    fn mixed_vulgar_spaced() -> Result<()> {
        let input = "1 ½";

        assert_eq!(parse_number(input)?, 1.5);

        Ok(())
    }
}
