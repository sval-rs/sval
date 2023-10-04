#![feature(test)]

extern crate test;

struct RecordTuple {
    before: u8,
    flatten: Option<FlattenRecordTuple>,
    after: u8,
}

impl sval::Value for RecordTuple {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        stream.record_tuple_begin(None, None, None, None)?;

        stream_record_struct_fields(&mut *stream, self.before)?;

        if let Some(ref flatten) = self.flatten {
            sval_flatten::flatten_to_record_tuple(&mut *stream, flatten, self.before as isize)?;
        }

        stream_record_struct_fields(&mut *stream, self.after)?;

        stream.record_tuple_end(None, None, None)
    }
}

struct FlattenRecordTuple {
    flatten: u8,
}

impl sval::Value for FlattenRecordTuple {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        stream.record_tuple_begin(None, None, None, None)?;

        sval_flatten::flatten_to_record_tuple(&mut *stream, &self.flatten, 0)?;

        stream.record_tuple_end(None, None, None)
    }
}

fn stream_record_struct_fields<'sval>(
    stream: &mut (impl sval::Stream<'sval> + ?Sized),
    count: u8,
) -> sval::Result {
    for i in 0..count {
        stream.record_tuple_value_begin(
            None,
            &sval::Label::new(FIELD_NAMES[i as usize]),
            &sval::Index::new(i as usize).with_tag(&sval::tags::VALUE_OFFSET),
        )?;
        stream.u8(i)?;
        stream.record_tuple_value_end(
            None,
            &sval::Label::new(FIELD_NAMES[i as usize]),
            &sval::Index::new(i as usize).with_tag(&sval::tags::VALUE_OFFSET),
        )?;
    }

    Ok(())
}

struct NullStream;

impl<'sval> sval::Stream<'sval> for NullStream {
    #[inline(never)]
    fn null(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn bool(&mut self, _: bool) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn text_fragment_computed(&mut self, _: &str) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn text_end(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn i64(&mut self, _: i64) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn f64(&mut self, _: f64) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn seq_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn seq_value_end(&mut self) -> sval::Result {
        Ok(())
    }

    #[inline(never)]
    fn seq_end(&mut self) -> sval::Result {
        Ok(())
    }
}

#[bench]
fn unflatten_60(b: &mut test::Bencher) {
    let record = RecordTuple {
        before: 30,
        flatten: None,
        after: 30,
    };

    b.iter(|| sval::stream(&mut NullStream, &record))
}

#[bench]
fn flatten_60(b: &mut test::Bencher) {
    let record = RecordTuple {
        before: 20,
        flatten: Some(FlattenRecordTuple { flatten: 20 }),
        after: 20,
    };

    b.iter(|| sval::stream(&mut NullStream, &record))
}

#[bench]
fn json_unflatten_60(b: &mut test::Bencher) {
    let record = RecordTuple {
        before: 30,
        flatten: None,
        after: 30,
    };

    b.iter(|| sval_json::stream_to_string(&record))
}

#[bench]
fn json_flatten_60(b: &mut test::Bencher) {
    let record = RecordTuple {
        before: 20,
        flatten: Some(FlattenRecordTuple { flatten: 20 }),
        after: 20,
    };

    b.iter(|| sval_json::stream_to_string(&record))
}

const FIELD_NAMES: &[&'static str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
    "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32",
    "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48",
    "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "60", "61", "62", "63", "64",
    "65", "66", "67", "68", "69", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "80",
    "81", "82", "83", "84", "85", "86", "87", "88", "89", "90", "91", "92", "93", "94", "95", "96",
    "97", "98", "99", "100", "101", "102", "103", "104", "105", "106", "107", "108", "109", "110",
    "111", "112", "113", "114", "115", "116", "117", "118", "119", "120", "121", "122", "123",
    "124", "125", "126", "127", "128", "129", "130", "131", "132", "133", "134", "135", "136",
    "137", "138", "139", "140", "141", "142", "143", "144", "145", "146", "147", "148", "149",
    "150", "151", "152", "153", "154", "155", "156", "157", "158", "159", "160", "161", "162",
    "163", "164", "165", "166", "167", "168", "169", "170", "171", "172", "173", "174", "175",
    "176", "177", "178", "179", "180", "181", "182", "183", "184", "185", "186", "187", "188",
    "189", "190", "191", "192", "193", "194", "195", "196", "197", "198", "199", "200", "201",
    "202", "203", "204", "205", "206", "207", "208", "209", "210", "211", "212", "213", "214",
    "215", "216", "217", "218", "219", "220", "221", "222", "223", "224", "225", "226", "227",
    "228", "229", "230", "231", "232", "233", "234", "235", "236", "237", "238", "239", "240",
    "241", "242", "243", "244", "245", "246", "247", "248", "249", "250", "251", "252", "253",
    "254",
];
