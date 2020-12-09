use std::str::FromStr;

pub trait StrExt
    where Self: AsRef<str>,
{
    /// Returns the left and right halves of `s` split at the first index where the pattern `p` matches.
    fn split_when<F: Fn(char) -> bool>(&self, f: F) -> Option<(&str, &str)> {
        let self_: &str = self.as_ref();
        self_.find(f).map(|i| self_.split_at(i))
    }

    /// Returns the result of splitting this string slice at the first occurrence of `delimiter`.
    /// The resulting halves both exclude the delimiter. Returns `None` if `delimiter` does not
    /// match within the string.
    ///
    /// Substitute for `core::str::split_once` without relying on nightly channel.
    fn split_once_(&self, delimiter: &str) -> Option<(&str, &str)> {
        let self_: &str = self.as_ref();
        self_.find(delimiter)
            .map(|index| (&self_[0..index], &self_[(index + delimiter.len())..]))
    }

    fn paragraphs(&self, trim_newline: bool) -> Paragraphs {
        Paragraphs { text: self.as_ref(), trim_newline }
    }

    #[inline]
    /// Validates a string that is convertible to `T` according to the closure `f`.
    fn validate<T, F>(&self, f: F) -> bool
        where F: Fn(T) -> bool,
              T: FromStr, { validate_str(self.as_ref(), f) }
}

impl<S> StrExt for S where S: AsRef<str> {}

pub trait OptionExt {
    fn is_some_and_valid<T, F>(&self, f: F) -> bool
        where F: Fn(T) -> bool,
              T: FromStr;

    fn is_valid_or_none<T, F>(&self, f: F) -> bool
        where F: Fn(T) -> bool,
              T: FromStr;
}

impl<S> OptionExt for Option<S>
    where S: AsRef<str>,
{
    fn is_some_and_valid<T, F>(&self, f: F) -> bool
        where F: Fn(T) -> bool,
              T: FromStr,
    {
        let s = self.as_ref().map(|s| s.as_ref());
        validate_req_str(s, f)
    }

    fn is_valid_or_none<T, F>(&self, f: F) -> bool
        where F: Fn(T) -> bool,
              T: FromStr
    {
        let s = self.as_ref().map(|s| s.as_ref());
        validate_opt_str(s, f)
    }
}

pub struct Paragraphs<'s> {
    text: &'s str,
    trim_newline: bool,
}

const NEWLINE_LEN: usize = 1;
const CR_LEN: usize = 1;

impl<'s> Iterator for Paragraphs<'s> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut index = 0;

        // Skip any blank lines.
        let mut next_index = loop {
            let (blank, next) = scan_next_line(self.text, index);
            if blank {
                if let Some(next) = next {
                    index = next;
                } else {
                    // Iteration exhausted, set to empty slice.
                    self.text = &self.text[0..0];
                    return None;
                }
            } else {
                break next;
            }
        };

        // The beginning byte index of the paragraph.
        let start = index;
        let mut result;

        loop {
            if let Some(index) = next_index.take() {
                let (blank, next) = scan_next_line(self.text, index);
                next_index = next;

                if blank {
                    result = &self.text[start..index];
                    if self.trim_newline {
                        result = trim_newline(result);
                    };

                    self.text = &self.text[index..];
                } else {
                    // Scan next line.
                    continue;
                }
            } else {
                // End of string reached.
                result = &self.text[start..];
                self.text = &self.text[0..0];
            }

            // Paragraph finished
            break;
        }

        // Check for carriage-return control character and remove if present.
        let last = result.chars().next_back();

        if let Some('\r') = last {
            result = &result[..(result.len() - CR_LEN)];
        }

        return Some(result);

        /// Scans a line within `s` starting from byte index `start`.
        ///
        /// The returned tuple's first field is `true` if the first line in `s[start..]` is blank,
        /// otherwise `false`.
        ///
        /// The returned tuple's second field is an optional index indicating the beginning of the
        /// next line (which may be empty, in which case it will equal `s.len()`), it will have a
        /// value of `None` if the `s[start..]` is a single line of text.
        fn scan_next_line(s: &str, start: usize) -> (bool, Option<usize>) {
            let chars = s[start..].chars();
            let mut byte_pos = start;
            let mut blank = true;

            // TODO implement raw parsing of utf-8 bytes to improve efficiency here.
            for c in chars {
                byte_pos += c.len_utf8();

                if c == '\n' {
                    return (blank, Some(byte_pos));
                }

                blank = blank & c.is_whitespace();
            }

            // We reached the end of the string (no possible index for next line).
            (blank, None)
        }

        fn trim_newline(s: &str) -> &str {
            debug_assert!(s.chars().next_back() == Some('\n'));

            let end = s.len() - NEWLINE_LEN;
            let mut result = &s[..end];

            let last = result.chars().next_back();
            if let Some('\r') = last {
                result = &result[..(result.len() - CR_LEN)];
            }

            result
        }

    }
}

////////////////////////////////////////////////////////////////////////////////
// Validation
////////////////////////////////////////////////////////////////////////////////

/// Validates a string that is convertible to `T` according to the closure `f`.
pub fn validate_str<T: FromStr, F: Fn(T) -> bool>(val: &str, f: F) -> bool
    where T: FromStr,
          F: Fn(T) -> bool,
{
    T::from_str(val)
        .map(|val| f(val))
        .unwrap_or(false)
}

/// Validates a required string value according to `f`. The `None` variant is considered _invalid_.
pub fn validate_req_str<T: FromStr, F: Fn(T) -> bool>(val: Option<&str>, f: F) -> bool {
    val.map(|val| validate_str(val, f))
        .unwrap_or(false)
}

/// Validates an optional string value according to `f`. The `None` variant is considered _valid_.
pub fn validate_opt_str<T: FromStr, F: Fn(T) -> bool>(val: Option<&str>, f: F) -> bool {
    val.map(|val| validate_str(val, f))
        .unwrap_or(true)
}


////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test {
    use super::StrExt;

    static TEXT: &'static str = "\
Single line paragraph...

Multi-line...
paragraph...
\t\t \t
   \t    \t
    \t Indented paragraph...\r

No newline at end of string paragraph...";


    #[test]
    fn paragraphs_test() {

        // Without newline trimming.
        let paras = TEXT.paragraphs(false)
            .collect::<Vec<_>>();

        assert_eq!(
            paras,
            vec![
                "Single line paragraph...\n",
                "Multi-line...\nparagraph...\n",
                "    \t Indented paragraph...\r\n",
                "No newline at end of string paragraph...",
            ]
        );

        // With newline trimming.
        let paras = TEXT.paragraphs(true)
            .collect::<Vec<_>>();

        assert_eq!(
            paras,
            vec![
                "Single line paragraph...",
                "Multi-line...\nparagraph...",
                "    \t Indented paragraph...",
                "No newline at end of string paragraph...",
            ]);
    }
}