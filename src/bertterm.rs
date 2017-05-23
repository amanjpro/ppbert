use std::fmt::{self, Write};

use num::bigint;

use symtable::Symtable;

pub const DEFAULT_INDENT_WIDTH: usize = 2;
pub const DEFAULT_MAX_TERMS_PER_LINE: usize = 4;


#[derive(Debug, PartialEq)]
pub enum BertTerm {
    Nil,
    Int(i32),
    BigInt(bigint::BigInt),
    Float(f64),
    Atom { offset: usize, length: usize },
    Tuple(Vec<BertTerm>),
    List(Vec<BertTerm>),
    Map(Vec<BertTerm>, Vec<BertTerm>),
    String(Vec<u8>),
    Binary(Vec<u8>)
}

impl BertTerm {
    fn is_basic(&self) -> bool {
        match *self {
            BertTerm::Int(_)
            | BertTerm::BigInt(_)
            | BertTerm::Float(_)
            | BertTerm::Atom { offset: _, length: _ }
            | BertTerm::String(_)
            | BertTerm::Binary(_)
            | BertTerm::Nil => true,
            BertTerm::List(_)
            | BertTerm::Tuple(_)
            | BertTerm::Map(_, _) => false
        }
    }
}


pub struct PrettyPrinter<'a> {
    term: &'a BertTerm,
    indent_width: usize,
    max_terms_per_line: usize,
    symtable: &'a Symtable
}

impl <'a> fmt::Display for PrettyPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_term(self.term, f, 0)
    }
}

impl <'a> PrettyPrinter<'a> {
    /// Creates a pretty printer for `term` where sub-terms
    /// are indented with a width of `indent_width` and a
    /// maximum of `max_terms_per_line` basic terms (i.e.,
    /// integers, floats, strings) can be printed per line.
    pub fn new(term: &'a BertTerm,
               indent_width: usize,
               max_terms_per_line: usize,
               symtable: &'a Symtable) -> Self {
        PrettyPrinter { term, indent_width, max_terms_per_line, symtable }
    }


    fn write_term(&self, term: &BertTerm, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        match *term {
            BertTerm::Nil => f.write_str("[]"),
            BertTerm::Int(n) => write!(f, "{}", n),
            BertTerm::BigInt(ref n) => write!(f, "{}", n),
            BertTerm::Float(x) => write!(f, "{}", x),
            BertTerm::Atom { offset, length } => {
                let s = self.symtable.get(offset, length);
                f.write_str(s)
            }
            BertTerm::String(ref bytes) => self.write_string(bytes, f, "\"", "\""),
            BertTerm::Binary(ref bytes) => self.write_string(bytes, f, "<<\"", "\">>"),
            BertTerm::List(ref terms) => self.write_collection(terms, f, depth, '[', ']'),
            BertTerm::Tuple(ref terms) => self.write_collection(terms, f, depth, '{', '}'),
            BertTerm::Map(ref keys, ref vals) => self.write_map(keys, vals, f, depth)
        }
    }


    fn write_string(&self,
                    bytes: &[u8],
                    f: &mut fmt::Formatter,
                    open: &str,
                    close: &str) -> fmt::Result {
        f.write_str(open)?;
        for &b in bytes {
            if is_printable(b) {
                f.write_char(b as char)?;
            } else {
                write!(f, "\\x{:02x}", b)?;
            }
        }
        f.write_str(close)
    }


    fn write_collection(&self,
                        terms: &[BertTerm],
                        f: &mut fmt::Formatter,
                        depth: usize,
                        open: char,
                        close: char) -> fmt::Result {
        let multi_line = !self.is_small_collection(terms);

        // Every element will have the same indentation,
        // so pre-compute it once.
        let prefix =
            if multi_line {
                self.indentation(depth+1)
            } else {
                String::new()
            };

        f.write_char(open)?;
        let mut comma = "";
        for t in terms {
            f.write_str(comma)?;
            f.write_str(&prefix)?;
            self.write_term(t, f, depth + 1)?;
            comma = ", ";
        }

        if multi_line {
            f.write_str(&self.indentation(depth))?;
        }

        f.write_char(close)
    }


    fn write_map(&self,
                 keys: &[BertTerm],
                 vals: &[BertTerm],
                 f: &mut fmt::Formatter,
                 depth: usize) -> fmt::Result {
        let multi_line =
            !self.is_small_collection(keys) || !self.is_small_collection(vals);
        let prefix =
            if multi_line {
                self.indentation(depth+1)
            } else {
                String::new()
            };

        f.write_str("#{")?;
        let mut comma = "";
        for i in 0 .. keys.len() {
            f.write_str(comma)?;
            f.write_str(&prefix)?;
            self.write_term(&keys[i], f, depth + 1)?;
            f.write_str(" => ")?;
            self.write_term(&vals[i], f, depth + 1)?;
            comma = ", ";
        }

        if multi_line {
            f.write_str(&self.indentation(depth))?;
        }
        f.write_str("}")
    }

    fn is_small_collection(&self, terms: &[BertTerm]) -> bool {
        terms.len() <= self.max_terms_per_line &&
            terms.iter().all(BertTerm::is_basic)
    }

    fn indentation(&self, depth: usize) -> String {
        ::std::iter::once('\n')
            .chain((0 .. depth * self.indent_width).map(|_| ' '))
            .collect()
    }
}



fn is_printable(b: u8) -> bool {
    b >= 0x20 && b <= 0x7e
}
