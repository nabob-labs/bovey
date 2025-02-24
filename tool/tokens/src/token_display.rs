use {
    bovey_account_decoder::parse_token::real_number_string_trimmed,
    bovey_sdk::native_token::lamports_to_bov,
    std::{
        fmt::{Debug, Display, Formatter, Result},
        ops::Add,
    },
};

const BOV_SYMBOL: &str = "◎";

#[derive(PartialEq, Eq)]
pub enum TokenType {
    Bov,
    OcpToken,
}

pub struct Token {
    amount: u64,
    decimals: u8,
    token_type: TokenType,
}

impl Token {
    fn write_with_symbol(&self, f: &mut Formatter) -> Result {
        match &self.token_type {
            TokenType::Bov => {
                let amount = lamports_to_bov(self.amount);
                write!(f, "{BOV_SYMBOL}{amount}")
            }
            TokenType::OcpToken => {
                let amount = real_number_string_trimmed(self.amount, self.decimals);
                write!(f, "{amount} tokens")
            }
        }
    }

    pub fn bov(amount: u64) -> Self {
        Self {
            amount,
            decimals: 9,
            token_type: TokenType::Bov,
        }
    }

    pub fn ocp_token(amount: u64, decimals: u8) -> Self {
        Self {
            amount,
            decimals,
            token_type: TokenType::OcpToken,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_with_symbol(f)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_with_symbol(f)
    }
}

impl Add for Token {
    type Output = Token;

    fn add(self, other: Self) -> Self {
        if self.token_type == other.token_type {
            Self {
                amount: self.amount + other.amount,
                decimals: self.decimals,
                token_type: self.token_type,
            }
        } else {
            self
        }
    }
}
