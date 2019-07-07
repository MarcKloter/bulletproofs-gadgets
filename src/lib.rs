//------------------------------------------------------------------------
// External dependencies
//------------------------------------------------------------------------
extern crate bulletproofs;
extern crate curve25519_dalek;
extern crate merlin;
extern crate pkcs7;
extern crate rand;

//------------------------------------------------------------------------
// Public modules
//------------------------------------------------------------------------

//------------------------------------------------------------------------
// Private modules
//------------------------------------------------------------------------
#[macro_use]
mod macros;
mod mimc_hash;
mod conversions;
mod gadget;
mod bounds_proof;
