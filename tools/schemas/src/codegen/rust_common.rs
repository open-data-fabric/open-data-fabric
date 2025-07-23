use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::OnceLock;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static RESERVED_WORDS: OnceLock<HashSet<&'static str>> = OnceLock::new();

pub fn format_ident<'a>(ident: &'a str) -> Cow<'a, str> {
    let reserved = RESERVED_WORDS.get_or_init(|| {
        let mut set = HashSet::new();
        set.insert("type");
        set
    });

    if reserved.contains(ident) {
        Cow::Owned(format!("r#{ident}"))
    } else {
        Cow::Borrowed(ident)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
