const DEFAULT_CAPACITY: usize = 4096;

/// A simple symbol table
#[derive(Debug)]
pub struct Symtable {
    strings: String,
    curr_offset: usize,
}

impl Symtable {
    /// Creates a new symbol table; its initial capacity is 4096.
    pub fn new() -> Symtable {
        return Symtable::with_capacity(DEFAULT_CAPACITY);
    }


    /// Creates a new symbol table with an initial capacity.
    pub fn with_capacity(cap: usize) -> Symtable {
        return Symtable {
            strings: String::with_capacity(cap),
            curr_offset: 0
        };
    }


    /// Adds a string to the symbol table and returns
    /// its offset and its length.  `add` does not
    /// check if the string already exists, the caller
    /// must ensure that.
    pub fn add(&mut self, s: &str) -> (usize, usize) {
        let r = (self.curr_offset, s.len());
        self.strings.push_str(s);
        self.curr_offset += s.len();
        return r;
    }

    /// Gets the string at the given offset and of the
    /// given length.
    /// This function panics if `offset` or `offset+len`
    /// are out of bounds.
    pub fn get(&self, offset: usize, len: usize) -> &str {
        return &self.strings[offset .. offset+len];
    }
}
