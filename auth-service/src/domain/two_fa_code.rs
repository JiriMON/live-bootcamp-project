use rand::prelude::*;
use color_eyre::eyre::{eyre, Context, Result};

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self> { // Updated!
        let code_as_u32 = code.parse::<u32>().wrap_err("Invalid 2FA code")?; // Updated!

        if (100_000..=999_999).contains(&code_as_u32) {
            Ok(Self(code))
        } else {
            Err(eyre!("Invalid 2FA code")) // Updated!
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        // Use the `rand` crate to generate a random 2FA code.
        // The code should be 6 digits (ex: 834629)
        Self(rand::thread_rng().gen_range(100000..1000000).to_string())
    }
}

// Implement AsRef<str> for TwoFACode
impl AsRef<str> for TwoFACode{
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::TwoFACode;


    #[test]
    fn non_valid_code_is_rejected() {
        let two_fa_code: String = "cdcd".to_string();
        assert!(TwoFACode::parse(two_fa_code).is_err());
    }

    #[test]
    fn short_code_is_rejected() {
        let two_fa_code: String = "99999".to_string();
        assert!(TwoFACode::parse(two_fa_code).is_err());
    }

    #[test]
    fn long_code_is_rejected() {
        let two_fa_code: String = "1000000".to_string();
        assert!(TwoFACode::parse(two_fa_code).is_err());
    }

    #[test]
    fn valid_code_is_accepted() {
        let two_fa_code: String = "123456".to_string();
        assert!(TwoFACode::parse(two_fa_code).is_ok());
    }

    #[test]
    fn valid_auto_generated_code_is_accepted() {
        let two_fa_code= TwoFACode::default();
        println!("code: {}",two_fa_code.as_ref());
        assert!(TwoFACode::parse(two_fa_code.as_ref().to_string()).is_ok());
    }

}