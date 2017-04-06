//! # The `LLaMaPUn` library in Rust
//! Language and Mathematics Processing and Understanding
//! Common data structures and algorithms for semi-structured NLP on math-rich documents.

#![feature(slice_patterns)]
#![feature(type_ascription)]
#![feature(box_patterns)]
#![deny(missing_docs,
        trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        unused_qualifications)]

extern crate libxml;
extern crate libc;
extern crate regex;
extern crate unidecode;
extern crate gnuplot;
extern crate rustmorpha;
extern crate walkdir;
extern crate senna;
extern crate crypto;
#[macro_use] extern crate lazy_static;

#[macro_use] pub mod util;
pub mod dnm;
pub mod data;
pub mod stopwords;
pub mod tokenizer;
pub mod ngrams;
pub mod patterns;
pub mod pattern_example_adaption;
