use anyhow::{anyhow, Error};
use bigdecimal::BigDecimal;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathConst {
    Pi,
    Tau,
    E,
    Phi,
    C,  // Speed of light (m/s)
    H,  // Planck (Js)
    G,  // Gravitational constant (m^3/(kg s^2))
    R,  // Gas constant (J/(mol K))
    Na, // Avogadro's number (mol^-1)
    Kb, // Boltzmann constant (J/K)
    Ec, // Electron charge (C)
}

impl MathConst {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pi => "pi",
            Self::Tau => "tau",
            Self::E => "e",
            Self::Phi => "phi",
            Self::C => "c",
            Self::H => "h",
            Self::G => "g",
            Self::R => "r",
            Self::Na => "na",
            Self::Kb => "kb",
            Self::Ec => "ec",
        }
    }
}

impl fmt::Display for MathConst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<MathConst> for BigDecimal {
    fn from(value: MathConst) -> Self {
        match value {
            MathConst::Pi => BigDecimal::from_str("3.1415926535897932384626433832795028841971").unwrap(),
            MathConst::Tau => BigDecimal::from_str("6.2831853071795864769252867665590057683942").unwrap(),
            MathConst::E => BigDecimal::from_str("2.7182818284590452353602874713526624977572").unwrap(),
            MathConst::Phi => BigDecimal::from_str("1.6180339887498948482045868343656381177203").unwrap(),
            MathConst::C => BigDecimal::from_str("299792458").unwrap(),
            MathConst::H => BigDecimal::from_str("6.62607015e-34").unwrap(),
            MathConst::G => BigDecimal::from_str("6.67430e-11").unwrap(),
            MathConst::R => BigDecimal::from_str("8.314462618").unwrap(),
            MathConst::Na => BigDecimal::from_str("6.02214076e23").unwrap(),
            MathConst::Kb => BigDecimal::from_str("1.380649e-23").unwrap(),
            MathConst::Ec => BigDecimal::from_str("1.602176634e-19").unwrap(),
        }
    }
}

impl TryFrom<&str> for MathConst {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "pi" => Ok(Self::Pi),
            "tau" => Ok(Self::Tau),
            "e" => Ok(Self::E),
            "phi" => Ok(Self::Phi),
            "c" => Ok(Self::C),
            "h" => Ok(Self::H),
            "g" => Ok(Self::G),
            "r" => Ok(Self::R),
            "na" => Ok(Self::Na),
            "kb" => Ok(Self::Kb),
            "ec" => Ok(Self::Ec),
            _ => Err(anyhow!("Unknown math constant: {}", value)),
        }
    }
}