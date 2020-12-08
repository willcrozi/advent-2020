use std::mem;

/// Returns the left and right halves of `s` split at the first index where the pattern `p` matches.
pub fn split_when<F: Fn(char) -> bool>(s: &str, f: F) -> Option<(&str, &str)> {
    s.find(f).map(|i| s.split_at(i))
}

pub fn split_once<'a>(s: &'a str, delimiter: &str) -> Option<(&'a str, &'a str)> {
    s.find(delimiter)
        .map(|index| (&s[0..index], &s[(index + delimiter.len())..]))

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