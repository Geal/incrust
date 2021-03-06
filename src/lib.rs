// Copyright (c) 2016 Incrust developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![allow(unstable_features)]
#![feature(question_mark)]
#![feature(specialization)]
#![feature(associated_type_defaults)]

#![cfg_attr(feature = "quiet", allow(warnings))]

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(items_after_statements, match_bool))]


extern crate marksman_escape;
#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate nom;


pub mod abc;
pub mod incrust;
pub mod loader;
pub mod parser;
pub mod renderer;
pub mod template;
pub mod types;

pub use self::abc::{Loader};
pub use self::incrust::Incrust;
pub use self::loader::{DictLoader, FilesystemLoader, GroupLoader, NamespaceLoader};
pub use self::types::abc::{Args, EntityId, Type, BType, ex};
pub use self::types::context::{Context};
pub use self::types::function::{Function};
pub use self::template::Template;
