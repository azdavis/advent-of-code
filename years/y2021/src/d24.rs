//! First I tried brute-force, that doesn't work. 9^14 is pretty big. Assuming
//! we try 100_000 numbers per second, it'll take
//!
//! ```not_rust
//! 9^14 / 100000 / 60 / 60 / 24 / 365 =~= 7
//! ```
//!
//! years to finish. Nope.
//!
//! So I tried optimizing by removing dead code, simplifying operations, etc.
//! That also wasn't enough.
//!
//! I also tried using the knowledge that each input is in the range 1..=9 to
//! track 'range operations': e.g.
//!
//! - `a..=b + 5 = (a+5)..=(b+5)`
//! - `a..=b + c..=d = (a+c)..=(b+d)`
//! - `a..=b % c = if b < c then a..=b else 0..=(c-1)`
//!
//! But then I just ended up with a bunch of ranges, with no knowledge of how
//! the inputs contributed to the ranges. And trying to track the inputs quickly
//! got out of hand.
//!
//! So, defeated, I went to the subreddit, and was vindicated in the fact that
//! it basically seems you have to 'analyze' the input for patterns and
//! essentially do the problem by hand (ish). Not wanting to do that, but also
//! wanting to be done with AoC for this year, I downloaded a Python solution
//! from the subreddit, ran it on my input, and moved on with my life.
