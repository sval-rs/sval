pub struct OwnedTag(Inner);

enum Inner {
    Owned(String),
    Static(&'static str),
}
