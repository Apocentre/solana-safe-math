Solana Safe Math
===

```
 use solana_safe_math::{SafeMath};
 
 fn process_init_escrow(
   accounts: &[AccountInfo],
   amount: u64,
   program_id: &Pubkey
 ) -> ProgramResult {
   let val = 10_u64;
  
   val.safe_add(amount)?;
   val.safe_sub(amount)?;
   val.safe_mul(amount)?;
   val.safe_div(amount)?;
   val.safe_pow(8_u32)?;
 }
```

Works with `u128`, `u64`, `u32`, `u16` and `u8`
