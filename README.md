 #Big Bag Of Words

 The "Big Bag Of Words" is used in text analysis and
 machine learning.  It reduces a text to a collection of
 words, each with a count of the number of occurrences.

 This implementation uses zero-copy strings when
 reasonably possible to improve performance and reduce
 memory usage.

 Words are separated by whitespace, and consist of a
 span of one or more consecutive letters (any Unicode
 code point in the "letter" class) with no internal
 punctuation: leading and trailing punctuation are
 removed.

 For example, the text

 ```text
 "It ain't over untïl it ain't, over."
 ```

 contains the sequence of words `"It"`, `"over"`,
 `"untïl"`, `"it"`, `"over"`.

 Words in the bag containing uppercase letters will be
 represented by their lowercase equivalent.

