# Splitter

Tests and benchmarks a handful of ways to split a string in Rust.

## What we're trying to do 

I wanted a Rust function that trims a string up to and including the first instance of a specified character.

So `fn remove_through_first_char("word1 word2 word3", ' ')` would return `"word2 word3"`. If the specified character isn't in the given string, return the string as inputted.

## An initial, naive attempt

My first working version of the function was this.

```rust
fn remove_through_first_char(l: &str, ch: char) -> String {
    if l.contains(ch) {
        let mut word_vec = l.split(ch).collect::<Vec<&str>>();
        word_vec.remove(0);
        return word_vec.join(&ch.to_string());
    } else {
        l.to_string()
    }
}
```

Basically we check if the character `ch` is present with a `contains`. If it is, we split on the given character `ch` into a vector (`word_vec`), `remove` the first element, then finally `join` the vector into a fresh new String.

While this version works in that it passes the tests in `src/lib.rs`, it is inefficient, mostly because it performs unnecessary allocations and copies strings. We can do better!

## Other approaches

[Sergey Bugaev](https://github.com/bugaevc) contributed not 1 but ultimately 4 other implementations of the function that all perform this same task more efficiently. They're all located in `src/lib.rs`. 

## Measuring performance of various implementations of the function

Using the Rust benchmarking library [Criterion](https://docs.rs/criterion/0.3.4/criterion/), we benchmarked each of the functions with varying input data sizes. You can see this code in `benches/splitters.rs`.

You should be able to run the benchmarks yourself by running `cargo bench`, but spoilers below.

![Line graph showing performance of various versions of the function](https://user-images.githubusercontent.com/10091584/114528394-a6d20d80-9c51-11eb-8e54-b21c3e2f9cba.png)

### A winner emerges

The clear winner, "V4", is this version, which uses [the memchr library](https://docs.rs/memchr/2.3.4/memchr/):

```rust
pub fn remove_through_first_char_variant_4(s: &str, ch: char) -> &str {
    match memchr(ch as u8, s.as_bytes()) {
        None => s, // not found => return the whole string
        Some(pos) => &s[pos + 1..],
    }
}
```

## Using the function in a non-trivial project

I'm now using this V4 implementation in a tool called [Tidy](https://github.com/sts10/tidy).

## Contributions

Do you have another way to perform the task described above that you think might be more efficient? Improvements to the existing functions? Feel free to create a pull request!
