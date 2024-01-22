use std::collections::HashMap;

// this Debug impl isn't much help.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct InternedSymbol(usize);

pub(crate) struct Interns {
    ix_to_sym: Vec<Vec<u8>>,
    sym_to_ix: HashMap<Vec<u8>, usize>,
}

pub(super) const TRUE: InternedSymbol = InternedSymbol(1);
pub(super) const FALSE: InternedSymbol = InternedSymbol(2);

impl Interns {
    pub(super) fn new() -> Interns {
        let mut i = Interns {
            ix_to_sym: vec![],
            sym_to_ix: HashMap::new(),
        };
        let t = i.intern("true");
        assert_eq!(TRUE, t);
        let f = i.intern("false");
        assert_eq!(FALSE, f);
        i
    }

    pub(super) fn intern<S: AsRef<[u8]>>(&mut self, s: S) -> InternedSymbol {
        let e = self.sym_to_ix.entry(s.as_ref().to_vec());
        InternedSymbol(*e.or_insert_with_key(|key| {
            self.ix_to_sym.push(key.clone());
            self.ix_to_sym.len()
        }))
    }

    pub(super) fn resolve(&self, i: InternedSymbol) -> &[u8] {
        self.ix_to_sym
            .get(i.0 - 1)
            .expect("should never get an invalid intern index")
    }
}
