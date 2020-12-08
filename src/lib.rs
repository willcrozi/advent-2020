use std::mem;

pub trait StrExt {
    /// Returns the left and right halves of `s` split at the first index where the pattern `p` matches.
    fn split_when<F: Fn(char) -> bool>(&self, f: F) -> Option<(&str, &str)>;

    /// Returns the result of splitting this string slice at the first occurrence of `delimiter`.
    /// The resulting halves both exclude the delimiter. Returns `None` if `delimiter` does not
    /// match within the string.
    ///
    /// Substitute for `core::str::split_once` without relying on nightly channel.
    fn split_once_(&self, delimiter: &str) -> Option<(&str, &str)>;
}

impl StrExt for str {
    fn split_when<F: Fn(char) -> bool>(&self, f: F) -> Option<(&str, &str)> {
        self.find(f).map(|i| self.split_at(i))
    }

    fn split_once_(&self, delimiter: &str) -> Option<(&str, &str)> {
        self.find(delimiter)
            .map(|index| (&self[0..index], &self[(index + delimiter.len())..]))

    }
}


/// Returns an iterator that iterates over the paragraphs in `input`. A paragraph is any consecutive
/// lines of text separated by one or more blank lines, i.e. lines that are empty or consist only of
/// whitespace.
pub fn parse_paras(input: &str) -> impl Iterator<Item=String> + '_ {
    let mut para_acc = input.lines()
        .fold(ParaAcc::new(), |acc, line| acc.parse_line(line));

    // Complete last paragraph in case of no trailing newline.
    if para_acc.in_para { para_acc.paras.push(para_acc.cur_para); }

    para_acc.paras.into_iter()
        .map(|para| para.join("\n"))
}

// Paragraph 'accumulator'
pub struct ParaAcc<'s> {
    paras: Vec<Vec<&'s str>>,
    cur_para: Vec<&'s str>,
    in_para: bool,
}

impl<'s> ParaAcc<'s> {
    fn new() -> Self { ParaAcc { paras: vec![], cur_para: vec![], in_para: false } }

    fn parse_line(mut self, line: &'s str) -> Self {
        if is_blank(line) {
            if self.in_para {
                assert!(self.cur_para.len() > 0);
                self.paras.push(mem::replace(&mut self.cur_para, vec![]));
                self.in_para = false;
            }
        } else {
            self.in_para = true;
            self.cur_para.push(line);
        }

        return self;

        ///////////////////////////////////////////////////////////
        fn is_blank(s: &str) -> bool {
            s.chars().fold(true, |blank, c| match c {
                ' ' | '\t' => true && blank,
                _ => false,
            })
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////