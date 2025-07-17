pub(crate) const AUTHOR: &str = "Dave Williams <dave@dave.io> <https://dave.io>";
pub(crate) const LONG_ABOUT: &str = r#"jig: a collection of wonderful things

jig is a collection of CLI tools and utilities which I've
wanted to have available at the command line at some point.
It's fairly chaotic, and I encourage you to embrace the chaos.

If you find something particularly useful, or have ideas for
features or adjustments, please let me know!

-- Dave Williams <dave@dave.io> <https://dave.io>"#;
pub(crate) const NAME: &str = "jig";
pub(crate) const SHORT_ABOUT: &str = "jig: a collection of wonderful things";

// Suppress warnings for test compilation where these constants
// are exported but not used within the constants module itself
#[cfg(test)]
#[allow(dead_code)]
const _SUPPRESS_UNUSED_WARNINGS: () = {
    let _ = (AUTHOR, LONG_ABOUT, NAME, SHORT_ABOUT);
};
