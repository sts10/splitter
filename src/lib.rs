use memchr::memchr;

pub fn remove_through_first_char(l: &str, ch: char) -> String {
    if l.contains(ch) {
        let mut word_vec = l.split(ch).collect::<Vec<&str>>();
        word_vec.remove(0);
        return word_vec.join(&ch.to_string());
    } else {
        l.to_string()
    }
}

pub fn remove_through_first_char_variant_1(s: &str, ch: char) -> &str {
    let mut split = s.splitn(2, ch);
    let before_ch = split
        .next()
        .expect("there should always be at least one part");
    match split.next() {
        Some(after_ch) => after_ch,
        None => before_ch,
    }
}

pub fn remove_through_first_char_variant_2(s: &str, ch: char) -> &str {
    // This is perhaps cleaner, but it performs an (unnecessary) allocation
    // for the Vec, although a very small one (just two fat pointers).
    let split: Vec<&str> = s.splitn(2, ch).collect();
    match &split[..] {
        [_before_ch, after_ch] => after_ch,
        [whole] => whole,
        _ => unreachable!("Not one or two parts?"),
    }
}

pub fn remove_through_first_char_variant_3(s: &str, ch: char) -> &str {
    // This does not use str::splitn(), but probably has an extra bounds check.
    // But perhaps the compiler will optmize it out? I don't know, you'd need
    // to measure.
    match s.find(ch) {
        None => s, // not found => return the whole string
        Some(pos) => &s[pos + 1..],
    }
}

pub fn remove_through_first_char_variant_4(s: &str, ch: char) -> &str {
    // Using memchr library
    match memchr(ch as u8, s.as_bytes()) {
        None => s, // not found => return the whole string
        Some(pos) => &s[pos + 1..],
    }
}

pub fn remove_through_first_char_variant_5(s: &str, ch: char) -> &str {
    // New in Rust 1.52 https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_once
    match s.split_once(ch) {
        Some(split_pair) => split_pair.1,
        None => s,
    }
}

use stringzilla::StringZilla;
pub fn remove_through_first_char_variant_6(s: &str, ch: char) -> &str {
    // https://github.com/ashvardanian/StringZilla?tab=readme-ov-file#quick-start-rust-
    match s.sz_find(&[ch as u8]) {
        None => s, // not found => return the whole string
        Some(pos) => &s[pos + 1..],
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Debug;

    fn do_test<S>(f: fn(&'static str, char) -> S)
    where
        // The return type can be a String or a &str:
        S: Debug + for<'a> PartialEq<&'a str>,
    {
        let test_line = "word1 word2 word3";
        assert_eq!(f(test_line, ' '), "word2 word3");

        let test_line = "word4";
        assert_eq!(f(test_line, ' '), "word4");

        let test_line = "1293\tword5";
        assert_eq!(f(test_line, '\t'), "word5");

        let test_line = "";
        assert_eq!(f(test_line, '\t'), "");
    }

    #[test]
    fn it_works() {
        do_test(remove_through_first_char);
        do_test(remove_through_first_char_variant_1);
        do_test(remove_through_first_char_variant_2);
        do_test(remove_through_first_char_variant_3);
        do_test(remove_through_first_char_variant_4);
        do_test(remove_through_first_char_variant_5);
        do_test(remove_through_first_char_variant_6);
    }
}
