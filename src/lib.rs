#![feature(conservative_impl_trait)]

#[macro_use]
extern crate serde_json;
extern crate rand;
extern crate snap;
#[macro_use]
extern crate log;

#[macro_use]
pub mod macros;

pub mod schema;
#[macro_use]
pub mod types;
pub mod codec;
pub mod complex;
pub mod datafile;
