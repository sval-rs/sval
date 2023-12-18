use crate::{default_stream, Error, Result, Stream, StreamEnum, Unsupported};

use super::owned_label;

pub(super) struct FlatStreamEnum<S> {
    stream: S,
    queue: Queue,
}

#[derive(Debug)]
struct NestedVariant {
    tag: Option<sval::Tag>,
    label: Option<sval::Label<'static>>,
    index: Option<sval::Index>,
}

impl<'sval, S: StreamEnum<'sval>> FlatStreamEnum<S> {
    pub fn new(stream: S) -> Self {
        FlatStreamEnum {
            stream,
            queue: Default::default(),
        }
    }

    pub fn push(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> Result {
        self.queue.push_back(NestedVariant {
            tag: tag.cloned(),
            label: if let Some(label) = label {
                Some(owned_label(label)?)
            } else {
                None
            },
            index: index.cloned(),
        })
    }

    pub fn end(self) -> Result<S::Ok> {
        self.value_or_recurse(|stream| stream.end(), |stream| stream.end())
    }

    fn value_or_recurse(
        mut self,
        value: impl FnOnce(Self) -> Result<S::Ok>,
        nested: impl FnOnce(FlatStreamEnum<S::Nested>) -> Result<<S::Nested as StreamEnum<'sval>>::Ok>,
    ) -> Result<S::Ok> {
        if let Some(variant) = self.queue.pop_front() {
            self.stream.nested(
                variant.tag.as_ref(),
                variant.label.as_ref(),
                variant.index.as_ref(),
                |variant| {
                    nested(FlatStreamEnum {
                        stream: variant,
                        queue: self.queue,
                    })
                },
            )
        } else {
            value(self)
        }
    }
}

impl<'sval, S: StreamEnum<'sval>> Stream<'sval> for FlatStreamEnum<S> {
    type Ok = S::Ok;

    type Map = Unsupported<S::Ok>;

    type Enum = Unsupported<S::Ok>;

    fn value<V: sval::Value + ?Sized>(self, value: &'sval V) -> Result<Self::Ok> {
        self.value_or_recurse(
            |stream| default_stream::value(stream, value),
            |stream| stream.value(value),
        )
    }

    fn value_computed<V: sval::Value + ?Sized>(self, value: &V) -> Result<Self::Ok> {
        self.value_or_recurse(
            |stream| default_stream::value_computed(stream, value),
            |stream| stream.value_computed(value),
        )
    }

    fn null(self) -> Result<Self::Ok> {
        Err(Error::invalid_value(
            "enum variants must be wrapped in a tag-carrying value",
        ))
    }

    fn bool(self, _: bool) -> Result<Self::Ok> {
        Err(Error::invalid_value(
            "enum variants must be wrapped in a tag-carrying value",
        ))
    }

    fn i64(self, _: i64) -> Result<Self::Ok> {
        Err(Error::invalid_value(
            "enum variants must be wrapped in a tag-carrying value",
        ))
    }

    fn f64(self, _: f64) -> Result<Self::Ok> {
        Err(Error::invalid_value(
            "enum variants must be wrapped in a tag-carrying value",
        ))
    }

    fn text_computed(self, _: &str) -> Result<Self::Ok> {
        Err(Error::invalid_value(
            "enum variants must be wrapped in a tag-carrying value",
        ))
    }

    fn tag(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> Result<Self::Ok> {
        self.value_or_recurse(
            |stream| stream.stream.tag(tag, label, index),
            |stream| Stream::tag(stream, tag, label, index),
        )
    }

    fn tagged<V: sval::Value + ?Sized>(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        value: &'sval V,
    ) -> Result<Self::Ok> {
        self.value_or_recurse(
            |stream| stream.stream.tagged(tag, label, index, value),
            |stream| Stream::tagged(stream, tag, label, index, value),
        )
    }

    fn tagged_computed<V: sval::Value + ?Sized>(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        value: &V,
    ) -> Result<Self::Ok> {
        self.value_or_recurse(
            |stream| stream.stream.tagged_computed(tag, label, index, value),
            |stream| Stream::tagged_computed(stream, tag, label, index, value),
        )
    }

    fn map_begin(self, _: Option<usize>) -> Result<Self::Map> {
        Ok(Unsupported::default())
    }

    fn enum_begin(
        self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> Result<Self::Enum> {
        Ok(Unsupported::default())
    }
}

#[derive(Default)]
struct Queue {
    #[cfg(feature = "alloc")]
    inner: crate::std::collections::VecDeque<NestedVariant>,
    #[cfg(not(feature = "alloc"))]
    inner: Option<NestedVariant>,
}

impl Queue {
    fn push_back(&mut self, variant: NestedVariant) -> Result {
        #[cfg(feature = "alloc")]
        {
            self.inner.push_back(variant);
            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            todo!()
        }
    }

    fn pop_front(&mut self) -> Option<NestedVariant> {
        #[cfg(feature = "alloc")]
        {
            self.inner.pop_front()
        }
        #[cfg(not(feature = "alloc"))]
        {
            self.inner.take()
        }
    }
}
