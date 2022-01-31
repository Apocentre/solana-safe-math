//! # Anchor Safe Math
//!
//! `anchor_safe_math` is a collection of helper numeric operation functions that removes the 
//! verbosity of checking for overflow, underflow and division by zero errors.
//! 
//! # Examples
//!
//! ```
//! use solana_safe_math::{SafeMath};
//! use solana_program::{entrypoint::ProgramResult};
//! 
//! fn process_init_escrow(
//!   accounts: &[AccountInfo],
//!   amount: u64,
//!   program_id: &Pubkey
//! ) -> ProgramResult {
//!   let val = 10_u64;
//!  
//!   val.safe_add(amount)?;
//!   val.safe_sub(amount)?;
//!   val.safe_mul(amount)?;
//!   val.safe_div(amount)?;
//!   val.safe_pow(8_u32)?;
//! }
//! ```
use solana_program::program_error::ProgramError;
use thiserror::Error;
use std::{
  result::Result as StdResult
};

#[derive(Error, Debug, Copy, Clone)]
pub enum ErrorCode {
  #[error("overflow")]
  Overflow,
  #[error("underflow")]
  Underflow,
  #[error("division by zero")]
  DivisionByZero,
}

impl From<ErrorCode> for ProgramError {
  fn from(e:ErrorCode) -> Self {
    ProgramError::Custom(e as u32)
  }
}

/// Defines a set of safe math operations that return a `ProgramError` which is expected in an anchor instruction execution.
pub trait SafeMath {
  type Output;

  fn safe_add(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError>;
  fn safe_sub(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError>;
  fn safe_div(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError>;
  fn safe_mul(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError>;
  fn safe_pow(&self, exp: u32) -> StdResult<Self::Output, ProgramError>;
}

macro_rules! safe_math {
  ($type: ident) => {
    /// $type implementation of the SafeMath trait
    impl SafeMath for $type {
      type Output = $type;

      fn safe_add(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError> {
        match self.checked_add(rhs) {
          Some(result) => Ok(result),
          None => return Err(ErrorCode::Overflow.into())
        }
      }
    
      fn safe_sub(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError> {
        match self.checked_sub(rhs) {
          Some(result) => Ok(result),
          None => return Err(ErrorCode::Underflow.into())
        }
      }

      fn safe_mul(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError> {
        match self.checked_mul(rhs) {
          Some(result) => Ok(result),
          None => return Err(ErrorCode::Underflow.into())
        }
      }

      fn safe_div(&self, rhs: Self::Output) -> StdResult<Self::Output, ProgramError> {
        match self.checked_div(rhs) {
          Some(result) => Ok(result),
          None => return Err(ErrorCode::DivisionByZero.into())
        }
      }

      fn safe_pow(&self, exp: u32) -> StdResult<Self::Output, ProgramError> {
        match self.checked_pow(exp) {
          Some(result) => Ok(result),
          None => return Err(ErrorCode::Overflow.into())
        }
      }
    }
  }
}

safe_math!(u128);
safe_math!(u64);
safe_math!(u32);
safe_math!(u16);
safe_math!(u8);
