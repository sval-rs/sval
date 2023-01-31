#[cfg(feature = "std")]
mod std_support {
    use crate::{
        std::io::{self, Read},
        Stream,
    };

    /**
    Stream the bytes of a generic reader as a bitstring.
    */
    pub fn stream_read<'sval, R: Read>(
        stream: &mut (impl Stream<'sval> + ?Sized),
        mut read: R,
    ) -> io::Result<()> {
        let mut buf = [0; 32];

        stream
            .binary_begin(None)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to begin a bitstring"))?;

        loop {
            match read.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => stream.binary_fragment_computed(&buf[..n]).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "failed to stream a bitstring fragment",
                    )
                })?,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            }
        }

        stream
            .binary_end()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to end a bitstring"))
    }
}

#[cfg(feature = "std")]
pub use self::std_support::*;
