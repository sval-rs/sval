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

/**
Stream the structure of a [`Value`] using the given [`Stream`]. 
*/
pub fn stream(value: impl Value, mut stream: impl Stream) -> Result<(), Error> {
    let stream = value::Stream::begin(&mut stream)?;
    
    value.stream(stream)
}