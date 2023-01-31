use core::{cmp::Ordering, mem};

pub fn from_slice(json: &str) -> &JsonSlice {
    JsonSlice::new(json)
}

#[repr(transparent)]
pub struct JsonSlice(str);

impl JsonSlice {
    pub fn new(src: &str) -> &JsonSlice {
        unsafe { mem::transmute::<&str, &JsonSlice>(src) }
    }
}

impl sval::Value for JsonSlice {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        let mut reader = JsonSliceReader::new(&self.0);

        while reader.stream_resume(&mut *stream)? {}

        Ok(())
    }
}

pub struct JsonSliceReader<'a> {
    src: &'a [u8],
    head: usize,
    in_str: bool,
    stack: Stack,
    position: Position,
}

impl<'a> JsonSliceReader<'a> {
    pub fn new(src: &'a str) -> JsonSliceReader<'a> {
        JsonSliceReader {
            src: src.as_bytes(),
            head: 0,
            in_str: false,
            stack: Stack::new(),
            position: Position::Root,
        }
    }

    fn map_begin<'b>(&mut self, mut stream: impl sval::Stream<'b>) -> sval::Result {
        match self.position {
            Position::SeqEmpty | Position::SeqElem => stream.seq_value_begin()?,
            Position::MapValue => stream.map_value_begin()?,
            Position::Root => (),
            _ => todo!(),
        }

        self.stack.push_map()?;
        self.position = Position::MapEmpty;

        stream.map_begin(None)
    }

    fn map_key_end<'b>(&mut self, mut stream: impl sval::Stream<'b>) -> sval::Result {
        if !matches!(self.position, Position::MapKey) {
            todo!();
        }

        self.position = Position::MapValue;

        stream.map_key_end()
    }

    fn map_end<'b>(&mut self, mut stream: impl sval::Stream<'b>) -> sval::Result
    where
        'a: 'b,
    {
        match self.position {
            Position::MapEmpty => (),
            Position::MapValue => {
                stream.map_value_end()?;
            }
            _ => todo!(),
        }

        self.stack.pop_map()?;
        self.position = self.stack.position();

        stream.map_end()
    }

    fn seq_begin<'b>(&mut self, mut stream: impl sval::Stream<'b>) -> sval::Result
    where
        'a: 'b,
    {
        match self.position {
            Position::SeqEmpty | Position::SeqElem => stream.seq_value_begin()?,
            Position::MapValue => stream.map_value_begin()?,
            Position::Root => (),
            _ => todo!(),
        }

        self.stack.push_seq()?;
        self.position = Position::SeqEmpty;

        stream.seq_begin(None)
    }

    fn seq_end<'b>(&mut self, mut stream: impl sval::Stream<'b>) -> sval::Result
    where
        'a: 'b,
    {
        match self.position {
            Position::SeqEmpty => (),
            Position::SeqElem => {
                stream.seq_value_end()?;
            }
            _ => todo!(),
        }

        self.stack.pop_seq()?;
        self.position = self.stack.position();

        stream.seq_end()
    }

    fn map_value_seq_value_end<'b>(&mut self, mut stream: impl sval::Stream<'b>) -> sval::Result {
        match self.position {
            Position::SeqElem => stream.seq_value_end(),
            Position::MapValue => {
                self.position = Position::MapKey;

                stream.map_value_end()
            }
            _ => todo!(),
        }
    }

    fn str_begin<'b>(&mut self, mut stream: impl sval::Stream<'b>) -> sval::Result {
        match self.position {
            Position::SeqEmpty | Position::SeqElem => {
                self.position = Position::SeqElem;

                stream.seq_value_begin()
            }
            Position::MapEmpty => {
                self.position = Position::MapKey;

                stream.map_key_begin()
            }
            Position::MapKey => stream.map_key_begin(),
            Position::MapValue => stream.map_value_begin(),
            Position::Root => Ok(()),
        }
    }

    fn value_begin<'b>(&mut self, mut stream: impl sval::Stream<'b>) -> sval::Result {
        match self.position {
            Position::SeqEmpty | Position::SeqElem => {
                self.position = Position::SeqElem;

                stream.seq_value_begin()
            }
            Position::MapValue => stream.map_value_begin(),
            Position::Root => Ok(()),
            _ => todo!(),
        }
    }

    fn maybe_done<'b>(&mut self) -> sval::Result<bool> {
        if self.head < self.src.len() {
            Ok(true)
        } else {
            self.stack.finish()?;

            Ok(false)
        }
    }

    fn stream_resume<'b, S: sval::Stream<'b> + ?Sized>(
        &mut self,
        stream: &mut S,
    ) -> sval::Result<bool>
    where
        'a: 'b,
    {
        if self.in_str {
            let (fragment, partial, head) = str_fragment(self.src, self.head)?;

            self.head = head;

            return if !partial {
                stream.text_fragment(fragment)?;
                stream.text_end()?;

                self.in_str = false;

                self.maybe_done()
            } else {
                stream.text_fragment(fragment)?;

                return Ok(true);
            };
        }

        while self.head < self.src.len() {
            match self.src[self.head] {
                // Begin a string
                b'"' => {
                    self.head += 1;

                    self.str_begin(&mut *stream)?;

                    let (fragment, partial, head) = str_fragment(self.src, self.head)?;

                    self.head = head;

                    stream.text_begin(None)?;

                    stream.text_fragment(fragment)?;

                    // If the string is complete (with no escapes)
                    // then we can yield it directly
                    return if !partial {
                        stream.text_end()?;

                        self.maybe_done()
                    }
                    // If the string has escapes then yield this fragment
                    // The next time we loop through we'll grab the next one
                    else {
                        self.in_str = true;

                        return Ok(true);
                    };
                }
                // Start a map
                b'{' => {
                    self.head += 1;

                    self.map_begin(&mut *stream)?;

                    return Ok(true);
                }
                // End a map
                b'}' => {
                    self.head += 1;

                    self.map_end(&mut *stream)?;

                    return self.maybe_done();
                }
                // Begin a seq
                b'[' => {
                    self.head += 1;

                    self.seq_begin(&mut *stream)?;

                    return Ok(true);
                }
                // End a seq
                b']' => {
                    self.head += 1;

                    self.seq_end(&mut *stream)?;

                    return self.maybe_done();
                }
                // End a map key
                b':' => {
                    self.head += 1;

                    self.map_key_end(&mut *stream)?;

                    return Ok(true);
                }
                // End either a map value or seq elem
                b',' => {
                    self.head += 1;

                    self.map_value_seq_value_end(&mut *stream)?;

                    return Ok(true);
                }
                // The boolean value `true`
                b't' => {
                    if let Some(b"true") = self.src.get(self.head..self.head + 4) {
                        self.head += 4;

                        self.value_begin(&mut *stream)?;

                        stream.bool(true)?;

                        return self.maybe_done();
                    } else {
                        todo!()
                    }
                }
                // The boolean value `false`
                b'f' => {
                    if let Some(b"false") = self.src.get(self.head..self.head + 5) {
                        self.head += 5;

                        self.value_begin(&mut *stream)?;

                        stream.bool(false)?;

                        return self.maybe_done();
                    } else {
                        todo!()
                    }
                }
                // The value `null`
                b'n' => {
                    if let Some(b"null") = self.src.get(self.head..self.head + 4) {
                        self.head += 4;

                        self.value_begin(&mut *stream)?;

                        stream.null()?;

                        return self.maybe_done();
                    } else {
                        todo!()
                    }
                }
                // Whitespace
                b' ' | b'\t' | b'\r' | b'\n' => {
                    self.head += 1;
                }
                // Numbers
                b'0'..=b'9' | b'-' => {
                    let (n, head) = number(self.src, self.head)?;

                    self.head = head;

                    self.value_begin(&mut *stream)?;

                    stream.tagged_begin(Some(&sval_json::tags::JSON_NATIVE), None, None)?;
                    stream.tagged_begin(Some(&sval::tags::NUMBER), None, None)?;
                    stream.text_begin(Some(n.len()))?;
                    stream.text_fragment(n)?;
                    stream.text_end()?;
                    stream.tagged_end(Some(&sval::tags::NUMBER), None, None)?;
                    stream.tagged_end(Some(&sval_json::tags::JSON_NATIVE), None, None)?;

                    return self.maybe_done();
                }
                _ => todo!(),
            }
        }

        self.maybe_done()
    }
}

#[derive(Debug, Clone, Copy)]
enum Position {
    Root,
    MapEmpty,
    MapKey,
    MapValue,
    SeqEmpty,
    SeqElem,
}

// Instead of keeping a traditional stack or bitmap that identifies the kind
// of value we're inside (either a map or seq), we keep a pair of integers.
// Each integer is dependent on the other. When we add to one, we always
// increment it so it's higher than the other. That means if we start a map
// but then try to end a seq we'll overflow and pick up the mismatch. We can
// guarantee {} [] are balanced this way up to quite a deep level of nesting
// on realistic documents using 128 bits. We can tell whether we're in a map
// or a sequence by looking at the higher number.
#[derive(Debug, Clone, Copy)]
struct Stack {
    map: u64,
    seq: u64,
}

impl Stack {
    fn new() -> Self {
        Stack { map: 0, seq: 0 }
    }

    fn push_map(&mut self) -> sval::Result {
        self.map = self
            .map
            .checked_add(1 + self.seq)
            .ok_or(sval::Error::new())?;
        Ok(())
    }

    fn pop_map(&mut self) -> sval::Result {
        self.map = self
            .map
            .checked_sub(1 + self.seq)
            .ok_or(sval::Error::new())?;
        Ok(())
    }

    fn push_seq(&mut self) -> sval::Result {
        self.seq = self
            .seq
            .checked_add(1 + self.map)
            .ok_or(sval::Error::new())?;
        Ok(())
    }

    fn pop_seq(&mut self) -> sval::Result {
        self.seq = self
            .seq
            .checked_sub(1 + self.map)
            .ok_or(sval::Error::new())?;
        Ok(())
    }

    fn position(&self) -> Position {
        match self.map.cmp(&self.seq) {
            Ordering::Greater => Position::MapValue,
            Ordering::Less => Position::SeqElem,
            Ordering::Equal => Position::Root,
        }
    }

    fn finish(&mut self) -> sval::Result {
        if self.map == 0 && self.seq == 0 {
            Ok(())
        } else {
            sval::error()
        }
    }
}

fn number(src: &[u8], mut head: usize) -> sval::Result<(&str, usize)> {
    let start = head;

    // TODO: Proper number parser
    while head < src.len() {
        match src[head] {
            b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' => {
                head += 1;
            }
            _ => break,
        }
    }

    let str = core::str::from_utf8(&src[start..head]).map_err(|_| sval::Error::new())?;

    Ok((str, head))
}

fn str_fragment(src: &[u8], mut head: usize) -> sval::Result<(&str, bool, usize)> {
    let start = head;

    if src[head] == b'\\' {
        head += 1;

        match src[head] {
            b'n' => return Ok(("\n", true, head + 1)),
            b'r' => return Ok(("\r", true, head + 1)),
            b'"' => return Ok(("\"", true, head + 1)),
            b'\\' => return Ok(("\\", true, head + 1)),
            _ => todo!(),
        }
    }

    // Scan through the input until we reach the end or an escaped character
    let mut partial = false;
    while head < src.len() {
        match src[head] {
            b'\\' => {
                partial = true;
                break;
            }
            b'"' => break,
            _ => {
                head += 1;
            }
        }
    }

    let str = core::str::from_utf8(&src[start..head]).map_err(|_| sval::Error::new())?;

    Ok((str, partial, if partial { head } else { head + 1 }))
}
