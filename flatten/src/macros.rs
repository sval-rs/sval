macro_rules! impl_stream_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn value<V: sval::Value + ?Sized>(&mut self, v: &'sval V) -> sval::Result {
                let $bind = self;
                ($($forward)*).value(v)
            }

            fn value_computed<V: sval::Value + ?Sized>(&mut self, v: &V) -> sval::Result {
                let $bind = self;
                ($($forward)*).value_computed(v)
            }

            #[inline]
            fn null(&mut self) -> sval::Result {
                let $bind = self;
                ($($forward)*).null()
            }

            #[inline]
            fn u8(&mut self, value: u8) -> sval::Result {
                let $bind = self;
                ($($forward)*).u8(value)
            }

            #[inline]
            fn u16(&mut self, value: u16) -> sval::Result {
                let $bind = self;
                ($($forward)*).u16(value)
            }

            #[inline]
            fn u32(&mut self, value: u32) -> sval::Result {
                let $bind = self;
                ($($forward)*).u32(value)
            }

            #[inline]
            fn u64(&mut self, value: u64) -> sval::Result {
                let $bind = self;
                ($($forward)*).u64(value)
            }

            #[inline]
            fn u128(&mut self, value: u128) -> sval::Result {
                let $bind = self;
                ($($forward)*).u128(value)
            }

            #[inline]
            fn i8(&mut self, value: i8) -> sval::Result {
                let $bind = self;
                ($($forward)*).i8(value)
            }

            #[inline]
            fn i16(&mut self, value: i16) -> sval::Result {
                let $bind = self;
                ($($forward)*).i16(value)
            }

            #[inline]
            fn i32(&mut self, value: i32) -> sval::Result {
                let $bind = self;
                ($($forward)*).i32(value)
            }

            #[inline]
            fn i64(&mut self, value: i64) -> sval::Result {
                let $bind = self;
                ($($forward)*).i64(value)
            }

            #[inline]
            fn i128(&mut self, value: i128) -> sval::Result {
                let $bind = self;
                ($($forward)*).i128(value)
            }

            #[inline]
            fn f32(&mut self, value: f32) -> sval::Result {
                let $bind = self;
                ($($forward)*).f32(value)
            }

            #[inline]
            fn f64(&mut self, value: f64) -> sval::Result {
                let $bind = self;
                ($($forward)*).f64(value)
            }

            #[inline]
            fn bool(&mut self, value: bool) -> sval::Result {
                let $bind = self;
                ($($forward)*).bool(value)
            }

            #[inline]
            fn text_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
                let $bind = self;
                ($($forward)*).text_begin(num_bytes)
            }

            #[inline]
            fn text_end(&mut self) -> sval::Result {
                let $bind = self;
                ($($forward)*).text_end()
            }

            #[inline]
            fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
                let $bind = self;
                ($($forward)*).text_fragment(fragment)
            }

            #[inline]
            fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
                let $bind = self;
                ($($forward)*).text_fragment_computed(fragment)
            }

            #[inline]
            fn binary_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
                let $bind = self;
                ($($forward)*).binary_begin(num_bytes)
            }

            #[inline]
            fn binary_end(&mut self) -> sval::Result {
                let $bind = self;
                ($($forward)*).binary_end()
            }

            #[inline]
            fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
                let $bind = self;
                ($($forward)*).binary_fragment(fragment)
            }

            #[inline]
            fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
                let $bind = self;
                ($($forward)*).binary_fragment_computed(fragment)
            }

            #[inline]
            fn map_begin(&mut self, num_entries: Option<usize>) -> sval::Result {
                let $bind = self;
                ($($forward)*).map_begin(num_entries)
            }

            #[inline]
            fn map_end(&mut self) -> sval::Result {
                let $bind = self;
                ($($forward)*).map_end()
            }

            #[inline]
            fn map_key_begin(&mut self) -> sval::Result {
                let $bind = self;
                ($($forward)*).map_key_begin()
            }

            #[inline]
            fn map_key_end(&mut self) -> sval::Result {
                let $bind = self;
                ($($forward)*).map_key_end()
            }

            #[inline]
            fn map_value_begin(&mut self) -> sval::Result {
                let $bind = self;
                ($($forward)*).map_value_begin()
            }

            #[inline]
            fn map_value_end(&mut self) -> sval::Result {
                let $bind = self;
                ($($forward)*).map_value_end()
            }

            #[inline]
            fn seq_begin(&mut self, num_entries: Option<usize>) -> sval::Result {
                let $bind = self;
                ($($forward)*).seq_begin(num_entries)
            }

            #[inline]
            fn seq_end(&mut self) -> sval::Result {
                let $bind = self;
                ($($forward)*).seq_end()
            }

            #[inline]
            fn seq_value_begin(&mut self) -> sval::Result {
                let $bind = self;
                ($($forward)*).seq_value_begin()
            }

            #[inline]
            fn seq_value_end(&mut self) -> sval::Result {
                let $bind = self;
                ($($forward)*).seq_value_end()
            }

            #[inline]
            fn tagged_begin(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>) -> sval::Result {
                let $bind = self;
                ($($forward)*).tagged_begin(tag, label, index)
            }

            #[inline]
            fn tagged_end(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>) -> sval::Result {
                let $bind = self;
                ($($forward)*).tagged_end(tag, label, index)
            }

            #[inline]
            fn tag(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>) -> sval::Result {
                let $bind = self;
                ($($forward)*).tag(tag, label, index)
            }

            #[inline]
            fn tag_hint(&mut self, tag: &sval::Tag) -> sval::Result {
                let $bind = self;
                ($($forward)*).tag_hint(tag)
            }

            #[inline]
            fn record_begin(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>, num_entries: Option<usize>) -> sval::Result {
                let $bind = self;
                ($($forward)*).record_begin(tag, label, index, num_entries)
            }

            #[inline]
            fn record_value_begin(&mut self, tag: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
                let $bind = self;
                ($($forward)*).record_value_begin(tag, label)
            }

            #[inline]
            fn record_value_end(&mut self, tag: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
                let $bind = self;
                ($($forward)*).record_value_end(tag, label)
            }

            #[inline]
            fn record_end(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>) -> sval::Result {
                let $bind = self;
                ($($forward)*).record_end(tag, label, index)
            }

            #[inline]
            fn tuple_begin(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>, num_entries: Option<usize>) -> sval::Result {
                let $bind = self;
                ($($forward)*).tuple_begin(tag, label, index, num_entries)
            }

            #[inline]
            fn tuple_value_begin(&mut self, tag: Option<&sval::Tag>, index: &sval::Index) -> sval::Result {
                let $bind = self;
                ($($forward)*).tuple_value_begin(tag, index)
            }

            #[inline]
            fn tuple_value_end(&mut self, tag: Option<&sval::Tag>, index: &sval::Index) -> sval::Result {
                let $bind = self;
                ($($forward)*).tuple_value_end(tag, index)
            }

            #[inline]
            fn tuple_end(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>) -> sval::Result {
                let $bind = self;
                ($($forward)*).tuple_end(tag, label, index)
            }

            #[inline]
            fn record_tuple_begin(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>, num_entries: Option<usize>) -> sval::Result {
                let $bind = self;
                ($($forward)*).record_tuple_begin(tag, label, index, num_entries)
            }

            #[inline]
            fn record_tuple_value_begin(&mut self, tag: Option<&sval::Tag>, label: &sval::Label, index: &sval::Index) -> sval::Result {
                let $bind = self;
                ($($forward)*).record_tuple_value_begin(tag, label, index)
            }

            #[inline]
            fn record_tuple_value_end(&mut self, tag: Option<&sval::Tag>, label: &sval::Label, index: &sval::Index) -> sval::Result {
                let $bind = self;
                ($($forward)*).record_tuple_value_end(tag, label, index)
            }

            #[inline]
            fn record_tuple_end(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>) -> sval::Result {
                let $bind = self;
                ($($forward)*).record_tuple_end(tag, label, index)
            }

            #[inline]
            fn enum_begin(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>) -> sval::Result {
                let $bind = self;
                ($($forward)*).enum_begin(tag, label, index)
            }

            #[inline]
            fn enum_end(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>) -> sval::Result {
                let $bind = self;
                ($($forward)*).enum_end(tag, label, index)
            }
        }
    };
}
