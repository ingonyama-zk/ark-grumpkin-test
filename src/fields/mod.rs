#[cfg(feature = "scalar_field")]
pub mod fr;
#[cfg(feature = "scalar_field")]
pub use self::fr::*;

#[cfg(feature = "curve")]
pub mod fq;
#[cfg(feature = "curve")]
pub use self::fq::*;

#[cfg(all(feature = "curve", test))]
mod tests;