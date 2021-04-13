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
