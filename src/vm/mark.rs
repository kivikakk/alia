use std::collections::HashMap;

use super::{InternedSymbol, Val};

pub(super) struct Mark {
    // submarks

    // consts // fns // macros
    // ^--- these all occupy the same namespace!
    submarks: HashMap<InternedSymbol, Mark>,
    binds: HashMap<InternedSymbol, Val>,
}

impl Mark {
    fn new() -> Self {
        Mark {
            submarks: HashMap::new(),
            binds: HashMap::new(),
        }
    }

    pub(super) fn builtins() -> Mark {
        let mut m = Mark::new();
        m
    }
}
