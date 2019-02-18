use std::hash::{Hash,Hasher};
use fnv::FnvHashMap;

#[derive(Eq,PartialEq,Copy,Clone,Hash,Debug)]
pub struct InternedString {
    index: usize,
}

pub struct StringInterner {
    indices: FnvHashMap<InternedStringRef, usize>,
    values: Vec<Box<str>>,
}

impl StringInterner {
    pub fn new() -> StringInterner {
        StringInterner {
            indices: FnvHashMap::default(),
            values: Vec::new(),
        }
    }

    pub fn intern(&mut self, string: String) -> InternedString {
        let key = InternedStringRef::from_str(&string);
        match self.indices.get(&key) {
            Some(&index) => InternedString { index: index },
            None => {
                let boxed_str = string.into_boxed_str();
                let key = InternedStringRef::from_str(&boxed_str);
                let index = self.values.len();
                self.values.push(boxed_str);
                self.indices.insert(key, index);
                InternedString { index: index }
            }
        }
    }

    pub fn resolve(&self, symbol: InternedString) -> &str {
        &self.values[symbol.index]
    }
}

#[derive(Eq,Debug,Copy,Clone)]
struct InternedStringRef {
    reference: *const str,
}

impl InternedStringRef {
    fn to_str(&self) -> &str {
        // It is safe to convert from a raw pointer here as we never remove strings
        // from the values vector so the pointer will always be valid.
        unsafe { &* self.reference }
    }

    fn from_str(string: &str) -> InternedStringRef {
        InternedStringRef { reference: string as *const str }
    }
}

impl Hash for InternedStringRef {
    fn hash<H:  Hasher>(&self, state: &mut H) {
        self.to_str().hash(state);
    }
}

impl PartialEq for InternedStringRef {
    fn eq(&self, other: &InternedStringRef) -> bool {
        self.to_str() == other.to_str()
    }
}

#[cfg(test)]
mod test {
    use fnv::FnvHasher;
    use super::*;

    #[test]
    fn interned_string_refs_equal() {
        let a = "test";
        let b = "test";
        let a_ref = InternedStringRef::from_str(&a);
        let b_ref = InternedStringRef::from_str(&b);

        assert_eq!(a_ref, b_ref);
    }

    #[test]
    fn interned_string_refs_not_equal() {
        let a = "test";
        let b = "test two";
        let a_ref = InternedStringRef::from_str(&a);
        let b_ref = InternedStringRef::from_str(&b);

        assert_ne!(a_ref, b_ref);
    }

    #[test]
    fn interned_string_refs_hash_equal() {
        let a = "test";
        let b = "test";
        let a_ref = InternedStringRef::from_str(&a);
        let b_ref = InternedStringRef::from_str(&b);
        let mut a_hash_builder = FnvHasher::default();
        let mut b_hash_builder = FnvHasher::default();
        a_ref.hash(&mut a_hash_builder);
        b_ref.hash(&mut b_hash_builder);

        assert_eq!(a_hash_builder.finish(), b_hash_builder.finish());
    }

    #[test]
    fn interned_string_refs_hash_not_equal() {
        let a = "test";
        let b = "test two";
        let a_ref = InternedStringRef::from_str(&a);
        let b_ref = InternedStringRef::from_str(&b);
        let mut a_hash_builder = FnvHasher::default();
        let mut b_hash_builder = FnvHasher::default();
        a_ref.hash(&mut a_hash_builder);
        b_ref.hash(&mut b_hash_builder);

        assert_ne!(a_hash_builder.finish(), b_hash_builder.finish());
    }

    #[test]
    fn interned_string_resolve() {
        let s = "test";
        let mut interner = StringInterner::new();
        let interned = interner.intern(s.to_string());
        let resolved = interner.resolve(interned);

        assert_eq!(s, resolved);
    }

    #[test]
    fn interned_string_equal() {
        let a = "test";
        let b = "test";
        let mut interner = StringInterner::new();
        let a_interned = interner.intern(a.to_string());
        let b_interned = interner.intern(b.to_string());

        assert_eq!(a_interned, b_interned);
        assert_eq!(interner.indices.len(), 1);
        assert_eq!(interner.values.len(), 1);
    }

    #[test]
    fn interned_string_not_equal() {
        let a = "test";
        let b = "test two";
        let mut interner = StringInterner::new();
        let a_interned = interner.intern(a.to_string());
        let b_interned = interner.intern(b.to_string());

        assert_ne!(a_interned, b_interned);
        assert_eq!(interner.indices.len(), 2);
        assert_eq!(interner.values.len(), 2);
    }
}
