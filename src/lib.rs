#![feature(map_first_last)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate deunicode;

pub mod dict_word;
pub mod pali;
pub mod letter_groups;
pub mod error;

