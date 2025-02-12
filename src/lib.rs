//! Big Bag Of Words
//!
//! The "Big Bag Of Words" is used in text analysis and
//! machine learning.  It reduces a text to a collection of
//! words, each with a count of the number of occurrences.
//!
//! This implementation uses zero-copy strings when
//! reasonably possible to improve performance and reduce
//! memory usage.
//!
//! Words are separated by whitespace, and consist of a
//! span of one or more consecutive letters (any Unicode
//! code point in the "letter" class) with no internal
//! punctuation: leading and trailing punctuation are
//! removed.
//!
//! For example, the text
//!
//! ```text
//! "It ain't over untïl it ain't, over."
//! ```
//!
//! contains the sequence of words `"It"`, `"over"`,
//! `"untïl"`, `"it"`, `"over"`.
//!
//! Words in the bag containing uppercase letters will be
//! represented by their lowercase equivalent.
use std::borrow::Cow;
use std::collections::BTreeMap;

/// Each key in this struct's map is a word in some
/// in-memory text document. The corresponding value is the
/// count of occurrences.
#[derive(Debug, Default, Clone)]
pub struct Bbow<'a>(BTreeMap<Cow<'a, str>, usize>);

fn is_word(word: &str) -> bool {
    !word.is_empty() && word.chars().all(|c| c.is_alphabetic())
}

fn has_uppercase(word: &str) -> bool {
    word.chars().any(char::is_uppercase)
}

impl<'a> Bbow<'a> {
    /// Make a new empty target words list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse the `target` text and add the sequence of
    /// valid words contained in it to this BBOW.
    ///
    /// This is a "builder method": calls can be
    /// conveniently chained to build up a BBOW covering
    /// multiple texts.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new().extend_from_text("Hello world.");
    /// assert_eq!(2, bbow.len());
    /// assert_eq!(1, bbow.match_count("hello"));
    /// ```
    pub fn extend_from_text(mut self, target: &'a str) -> Self {
        target.split_whitespace().for_each(|mut word: &str| {
            word = word.trim_matches(|c: char| !c.is_alphabetic());
            if is_word(word) {
                if has_uppercase(word) {
                    self.0
                        .entry(Cow::Owned(word.to_lowercase()))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                } else {
                    self.0
                        .entry(Cow::Borrowed(word))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        });

        self
    }

    /// Report the number of occurrences of the given
    /// `keyword` that are indexed by this BBOW. The keyword
    /// should be lowercase and not contain punctuation, as
    /// per the rules of BBOW: otherwise the keyword will
    /// not match and 0 will be returned.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("b b b-banana b");
    /// assert_eq!(3, bbow.match_count("b"));
    /// ```
    pub fn match_count(&self, keyword: &str) -> usize {
        let count = self.0.get(&Cow::from(keyword));
        match count {
            Some(c) => *c,
            None => 0,
        }
    }

    pub fn words(&'a self) -> impl Iterator<Item = &'a str> {
        self.0.keys().map(|w| w.as_ref())
    }

    /// Count the overall number of words contained in this BBOW:
    /// multiple occurrences are considered separate.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("Can't stop this! Stop!");
    /// assert_eq!(3, bbow.count());
    /// ```
    pub fn count(&self) -> usize {
        let mut total_count: usize = 0;
        self.0.values().for_each(|count| total_count += count);

        total_count
    }

    /// Count the number of unique words contained in this BBOW,
    /// not considering number of occurrences.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("Can't stop this! Stop!");
    /// assert_eq!(2, bbow.len());
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Is this BBOW empty?
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[test]
fn bbow_basic_tests() {
    let bbow = Bbow::new().extend_from_text("Hello world.. World ..wOrld!");
    assert_eq!(2, bbow.len());
    assert_eq!(1, bbow.match_count("hello"));
    assert_eq!(3, bbow.match_count("world"));
    assert_eq!(4, bbow.count());
}

#[test]
fn bbow_more_tests() {
    let s = "c🤚an't hasn't Բարեւ ընկերներ!";
    let bbow = Bbow::new().extend_from_text(s);
    assert_eq!(2, bbow.len());
    assert_eq!(1, bbow.match_count("ընկերներ"));
    assert_eq!(0, bbow.match_count("hasn't"));
    assert_eq!(1, bbow.match_count("բարեւ"));
}

#[test]
fn bbow_even_more_tests() {
    let s = "running 4 tests
test src/lib.rs - Bbow<'a>::match_count (line 95) ... ok
test src/lib.rs - Bbow<'a>::extend_from_text (line 60) ... ok
test src/lib.rs - Bbow<'a>::len (line 136) ... ok
test src/lib.rs - Bbow<'a>::count (line 118) ... ok";
    let bbow = Bbow::new()
        .extend_from_text(s)
        .extend_from_text("A quIck brown Fox jumps oVer a lazy DOG.");
    assert_eq!(4, bbow.match_count("ok"));
    assert_eq!(4, bbow.match_count("line"));
    assert_eq!(4, bbow.match_count("test"));
    assert_eq!(1, bbow.match_count("tests"));
    assert_eq!(1, bbow.match_count("running"));
    assert_eq!(0, bbow.match_count("..."));
    assert_eq!(0, bbow.match_count("Bbow"));
    assert_eq!(0, bbow.match_count("bbow"));
    assert_eq!(0, bbow.match_count("4"));
    assert_eq!(2, bbow.match_count("a"));
    assert_eq!(1, bbow.match_count("quick"));
    assert_eq!(1, bbow.match_count("brown"));
    assert_eq!(1, bbow.match_count("fox"));
    assert_eq!(1, bbow.match_count("jumps"));
    assert_eq!(1, bbow.match_count("over"));
    assert_eq!(1, bbow.match_count("lazy"));
    assert_eq!(1, bbow.match_count("dog"));
    assert_eq!(13, bbow.len());
}
