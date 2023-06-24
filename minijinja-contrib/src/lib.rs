//! MiniJinja-Contrib is a utility crate for [MiniJinja](https://github.com/mitsuhiko/minijinja)
//! that adds support for certain utilities that are too specific for the MiniJinja core.  This is
//! usually because they provide functionality that Jinja2 itself does not have.
//!
//! To add all of these to an environment you can use the [`add_to_environment`] function.
//!
//! ```
//! use minijinja::Environment;
//!
//! let mut env = Environment::new();
//! minijinja_contrib::add_to_environment(&mut env);
//! ```
#![cfg_attr(docsrs, feature(doc_cfg))]

use minijinja::Environment;

/// Utility filters.
pub mod filters;

/// Globals
pub mod globals;

/// Registers all features of this crate with an [`Environment`].
///
/// All the filters that are available will be added, same with global
/// functions that exist.
pub fn add_to_environment(env: &mut Environment) {
    env.add_filter("pluralize", filters::pluralize);
    #[cfg(feature = "datetime")]
    {
        env.add_filter("datetimeformat", filters::datetimeformat);
        env.add_filter("timeformat", filters::timeformat);
        env.add_filter("dateformat", filters::dateformat);
        env.add_function("now", globals::now);
    }
}
