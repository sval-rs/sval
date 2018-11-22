#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;

#[macro_use]
mod error;

pub mod value;
pub mod stream;

pub use self::{
    error::Error,
    value::Value,
    stream::Stream,
};

pub fn stream(value: impl Value, mut stream: impl Stream) -> Result<(), Error> {
    value.stream(value::Stream::begin(&mut stream))
}