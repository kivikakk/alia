use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct InternedSymbol(usize);

pub(super) struct Interns {
    ix_to_sym: Vec<Vec<u8>>,
    sym_to_ix: HashMap<Vec<u8>, usize>,
}

impl Interns {
    pub(super) fn new() -> Interns {
        Interns {
            ix_to_sym: vec![],
            sym_to_ix: HashMap::new(),
        }
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
