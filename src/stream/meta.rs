#[derive(Clone)]
pub struct MapMeta {
    name: Option<&'static str>,
    // TODO: Consider making this `Tag<'static>`? key_tags?
    // Should we combine with variant_tags?
    keys: &'static [&'static str],
    keys_sorted: bool,
    size_hint: Option<usize>,
}

impl Default for MapMeta {
    fn default() -> Self {
        Self::new()
    }
}

pub fn map_meta() -> MapMeta {
    Default::default()
}

impl MapMeta {
    pub fn new() -> Self {
        MapMeta {
            name: None,
            keys: &[],
            keys_sorted: true,
            size_hint: None,
        }
    }

    pub fn name(&self) -> Option<&'static str> {
        self.name
    }

    pub fn key(&self, key: &str) -> Option<&'static str> {
        if self.keys_sorted {
            self.keys.binary_search(&key).ok().map(|idx| self.keys[idx])
        } else {
            self.keys.iter().copied().find(|k| *k == key)
        }
    }

    pub fn size_hint(&self) -> Option<usize> {
        self.size_hint
    }

    pub fn with_name(mut self, name: impl Into<Option<&'static str>>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_keys(mut self, keys: &'static [&'static str]) -> Self {
        self.keys = keys;
        self.keys_sorted = false;
        self
    }

    pub fn with_keys_sorted(mut self, keys: &'static [&'static str]) -> Self {
        self.keys = keys;
        self.keys_sorted = true;
        self
    }

    pub fn with_size_hint(mut self, size_hint: impl Into<Option<usize>>) -> Self {
        self.size_hint = size_hint.into();
        self
    }
}

#[derive(Clone)]
pub struct SeqMeta {
    name: Option<&'static str>,
    size_hint: Option<usize>,
}

impl Default for SeqMeta {
    fn default() -> Self {
        Self::new()
    }
}

pub fn seq_meta() -> SeqMeta {
    Default::default()
}

impl SeqMeta {
    pub fn new() -> Self {
        SeqMeta {
            name: None,
            size_hint: None,
        }
    }

    pub fn name(&self) -> Option<&'static str> {
        self.name
    }

    pub fn size_hint(&self) -> Option<usize> {
        self.size_hint
    }

    pub fn with_name(mut self, name: impl Into<Option<&'static str>>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_size_hint(mut self, size_hint: impl Into<Option<usize>>) -> Self {
        self.size_hint = size_hint.into();
        self
    }
}
