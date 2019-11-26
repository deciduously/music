---
title: Teaching Numbers How To Sing
published: false
description: Learn how to procedurally generate melodies in a variety of keys with Rust.
cover_image: https://thepracticaldev.s3.amazonaws.com/i/iuakwwcexql5u0th7gtm.jpg
tags: beginners, rust, tutorial, music
---

## Everything Is Music

> Everything is music. When I go home, I throw knickers in the oven and it's music. Crash, boom, bang!

*- [Winona Ryder](https://en.wikipedia.org/wiki/Winona_Ryder) as [Björk](https://en.wikipedia.org/wiki/Bj%C3%B6rk) on [SNL](https://en.wikipedia.org/wiki/Saturday_Night_Live)'s [Celebrity Rock 'N' Roll Jeopardy!](https://en.wikipedia.org/wiki/Celebrity_Jeopardy!_(Saturday_Night_Live)) - [2002](https://en.wikipedia.org/wiki/2002) - [YouTube](https://youtu.be/R3V94ZtmdbQ?t=190)*

In this [post]((https://en.wikipedia.org/wiki/Blog)), we'll [throw](https://en.wikipedia.org/wiki/Throwing) something [random](https://en.wikipedia.org/wiki/Random_number_generation) into, [well](https://en.wikipedia.org/wiki/Well), a [math](https://en.wikipedia.org/wiki/Mathematics)-[oven](https://en.wikipedia.org/wiki/Subroutine) and [*viola*](https://en.wikipedia.org/wiki/Viola), [music](https://en.wikipedia.org/wiki/Music)!  We'll just skip the [crash](https://en.wikipedia.org/wiki/Crash_(computing)).

In other words, we're going to teach our [computers](https://en.wikipedia.org/wiki/Personal_computer) to ["sing"](https://en.wikipedia.org/wiki/Singing) using [Rust](https://en.wikipedia.org/wiki/Rust_(programming_language)), backed by a little light [physics](https://en.wikipedia.org/wiki/Physics) and [music theory](https://en.wikipedia.org/wiki/Music_theory).

The [one-liner](https://en.wikipedia.org/wiki/One-liner_program) in the cover image [procedurally generates](https://en.wikipedia.org/wiki/Procedural_generation) a [melody](https://en.wikipedia.org/wiki/Melody) using [tools assumed to be present](https://en.wikipedia.org/wiki/Unix_philosophy) on a standard [desktop](https://en.wikipedia.org/wiki/Desktop_computer) [Linux](https://en.wikipedia.org/wiki/Linux) [distribution](https://en.wikipedia.org/wiki/Linux_distribution) like [Ubuntu](https://en.wikipedia.org/wiki/Ubuntu).  The melody produced will be composed of notes along a single [octave](https://en.wikipedia.org/wiki/Octave) in a hardcoded [key](https://en.wikipedia.org/wiki/Key_(music)):

{% youtube uLhQQSKhTok %}

By the end of this post our program will:

1. Support a full range of key signatures of different types.
1. Use a full 108-key extended [piano](https://en.wikipedia.org/wiki/Piano) [keyboard](https://en.wikipedia.org/wiki/Musical_keyboard).
1. Produce any arbitrary tone we ask for.
1. Play back sequences recorded in a rudimentary [notation format](https://en.wikipedia.org/wiki/Musical_notation).
1. Encourage further extension with lots of Rust-y goodness.
1. Compile and run on Windows, MacOS, or Linux with no code changes (I tried all three).

However, at the end of the day, it's just the thing in the cover image.

The completed code can be found on [GitHub](https://github.com/deciduously/music).

## Table of Contents

- [Preamble](#preamble)
- [The Meme](#the-meme)
- [The Program](#the-program)
  * [Project Structure](#project-structure)
  * [Random Bytes](#random-bytes)
  * [Mapping Bytes To Notes](#mapping-bytes-to-notes)
    - [A Little Physics](#a-little-physics)
      * [Sine Waves](#sine-waves)
      * [Pitch](#pitch)
      * [Singing](#singing)
    - [A Little Music Theory](#a-little-music-theory)
      * [Scales](#scales)
      * [Cents](#cents)
      * [Scientific Pitch Notation](#scientific-pitch-notation)
      * [Circle of Fifths](#circle-of-fifths)
      * [Diatonic Modes](#diatonic-modes)
      * [Non Heptatonic Scales](#non-heptatonic-scales)
      * [Key](#key)
    - [Back To The Bytes](#back-to-the-bytes)
  * [Listen To Your Files](#listen-to-your-files)
  * [Write Your Own Tunes](#write-your-own-tunes)
- [Challenges](#challenges)

## Preamble

*[top](#table-of-contents)*

This tutorial is aimed at [beginners](https://en.wikipedia.org/wiki/Novice) (and up) who are comfortable solving problems with at least one [imperative](https://en.wikipedia.org/wiki/Imperative_programming) [language](https://en.wikipedia.org/wiki/Programming_language).  It does not matter if that's [JavaScript](https://en.wikipedia.org/wiki/JavaScript) or [Python](https://en.wikipedia.org/wiki/Python_(programming_language)) or [Object Pascal](https://en.wikipedia.org/wiki/Object_Pascal), I just assume you know the [basic](https://en.wikipedia.org/wiki/Syntax_(programming_languages)) [building](https://en.wikipedia.org/wiki/Semantics_(computer_science)) [blocks](https://en.wikipedia.org/wiki/Standard_library) of [creating a program](https://en.wikipedia.org/wiki/Computer_programming).  You do not need any prior knowledge of physics or music theory, but there will be a tiny smattering of [elementary algebra](https://en.wikipedia.org/wiki/Elementary_algebra).  I promise it's quick.

There's a bunch of fairly [idiomatic](https://en.wikipedia.org/wiki/Programming_idiom) [Rust](https://www.rust-lang.org/) throughout this write-up, but don't worry if that's not what you're here for.  You can choose to skip all the code snippets entirely and still come out knowing how it all works.

I have two disclaimers:

1. [There are](https://en.wikipedia.org/wiki/Existence) [217](https://en.wikipedia.org/wiki/217_(number)) [links](https://en.wikipedia.org/wiki/Hyperlink) [here](https://en.wikipedia.org/wiki/Boston), [173](https://en.wikipedia.org/wiki/173_(number)) [of them](https://en.wikipedia.org/wiki/Element_(mathematics)) [to](https://en.wikipedia.org/wiki/Codomain) [Wikipedia](https://en.wikipedia.org/wiki/Main_Page).  [If](https://en.wikipedia.org/wiki/Conditional_(computer_programming)) [you're](https://en.wikipedia.org/wiki/You) [that](https://en.wikipedia.org/wiki/Autodidacticism) [kind](https://en.wikipedia.org/wiki/Impulsivity) [of](https://en.wikipedia.org/wiki/Preposition_and_postposition) [person](https://en.wikipedia.org/wiki/Person), [set](https://en.wikipedia.org/wiki/Innovation) [rules](https://en.wikipedia.org/wiki/Law).
1. Further to Point 1, most of this I learned myself on Wikipedia, some of it while writing this post.  The rest is what I remember from [high school](https://en.wikipedia.org/wiki/High_school_(North_America)) as a [band geek](https://en.wikipedia.org/wiki/Euphonium), which was over [ten years](https://en.wikipedia.org/wiki/Decade) [ago](https://en.wikipedia.org/wiki/Past).  I do believe it's generally on the mark, but I am making no claims of authority.  If you see something, [say something](https://en.wikipedia.org/wiki/Allen_Kay#Advertisements).

## The Meme

*[top](#table-of-contents)*

This post was inspired by [this](https://www.reddit.com/r/linuxmasterrace/comments/dyqri7/like_god_would_have_wanted/) [meme](https://en.wikipedia.org/wiki/Internet_meme) I saw when I was *attempting* to casually browse [Reddit](https://en.wikipedia.org/wiki/Reddit):

![the meme](https://i.redd.it/uirqnamnjpz31.jpg)

I couldn't let myself just scroll past that one ([clearly](https://en.wikipedia.org/wiki/Diatribe).  Here's a version of the [`bash`](https://en.wikipedia.org/wiki/Bash_(Unix_shell)) [pipeline](https://en.wikipedia.org/wiki/Pipeline_(Unix)) at the bottom with slightly different hard-coded values, taken from [this blog post](https://blog.robertelder.org/bash-one-liner-compose-music/) by [Robert Elder](https://www.robertelder.org/) that explores it:

```bash
cat /dev/urandom | hexdump -v -e '/1 "%u\n"' | awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }' | xxd -r -p | aplay -c 2 -f S32_LE -r 16000
```

The linked blogpost is considerably more brief and assumes a greater degree of background knowledge than this one, but that's not to discredit it at as a fantastic source.  That write-up and Wikipedia were all I needed to complete this translation, and I had absolutely not a clue how this whole thing worked going in.

I've gotta be honest - I didn't even try the `bash` and immediately dove into the pure Rust solution.  Nevertheless, it serves as a solid [30,000ft](https://en.wikipedia.org/wiki/Flight_level) [roadmap](https://en.wikipedia.org/wiki/Plan):

1. `cat /dev/urandom`: Get a stream of random binary data.
1. `hexdump -v -e '/1 "%u\n"'`: Convert binary to 8-bit base-10 integers (0-255).
1. `awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }'`: Map integers to pitches and return sound wave samples.
1. `xxd -r -p`: Convert hexadecimal samples back to binary.
1. `aplay -c 2 -f S32_LE -r 16000`: Play back binary samples as sound wave.

Don't worry at all if some or all of this is incomprehensible.  You don't need to have a clue how any of it works yet.  I'm not going to do what that [code](https://en.wikipedia.org/wiki/Source_code) does exactly in this post, and I'm not going to elaborate much on what any of the specific commands in the pipeline mean (read the linked post for that).   By the time we're done, you'll be able to pick apart the whole thing yourself anyway.

If you'd like the challenge of implementing this yourself from scratch in your own language, **stop right here**.  If you get stuck, this should all apply to whatever you've got going unless you've gone real funky with it - in which case, it sounds cool and you should show me.

[¡Vámonos!](https://en.wikipedia.org/wiki/Party)

## The Program

*[top](#table-of-contents)*

### Project Structure

Ensure you have at least the default stable Rust toolchain [installed](https://www.rust-lang.org/tools/install).  If you've previously installed `rustup` at any point, just issue `rustup update`.  This code was written with `rustc` [version 1.39](https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html) for [Rust 2018](https://doc.rust-lang.org/nightly/edition-guide/rust-2018/edition-changes.html).  

Then, spin up a new library project:

```txt
$ cargo new music --lib
```

Open your new `music` project directory in the environment of your choice.  If you're not already sure what to use with Rust, I recommend [Visual Studio Code](https://code.visualstudio.com/) with the [Rust Language Server](https://github.com/rust-lang/rls) installed for in-editor development support.  If you have `rustup` present, the [VS Code RLS extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) has a one-click set up.  I have been an exclusive Linux user for years and years and wrote this whole project on Windows 10 with VS Code.  Rust is cross-platform, yo.

We'll use two crates - the Rust term for external libraries - to replace the functionality not found in the Rust standard library:

* [`rand`](https://docs.rs/rand/0.7.2/rand/) - [Random number generation](https://en.wikipedia.org/wiki/Random_number_generation)
* [`rodio`](https://docs.rs/rodio/0.10.0/rodio/) - [Audio signal processing](https://en.wikipedia.org/wiki/Audio_signal)

`rand` is in place of [`/dev/urandom`](https://en.wikipedia.org/wiki//dev/random) and [`hexdump`](https://en.wikipedia.org/wiki/Hex_dump), and `rodio` will cover [`xxd`](https://www.systutorials.com/docs/linux/man/1-xxd/) and [`aplay`](https://linux.die.net/man/1/aplay).  For step 3, which is the bulk of the program, we'll just use the standard library.  If `awk` can do it in two statements, Rust can sure as heck do it in several hundred lines (don't panic).  I also use [`pretty_assertions`](https://docs.rs/pretty_assertions/0.6.1/pretty_assertions/) to make the [test runner](https://en.wikipedia.org/wiki/Unit_testing) output a little prettier:

In `Cargo.toml`:

```toml
[dependencies]

rand = "0.7"
rodio = "0.10"

[dev-dependencies]

pretty_assertions = "0.6"
```

We're going to organize this program into three components.  The core of it all will be a library of types and relationships between them, which will live in `src/lib.rs`.  Most of our code will go here, this is where we'll model the domain.  This file is already created for you, with a stub `test` module.  We're going to make that module it's own file instead, `src/test.rs`.  In this file, we'll define unit tests to automatically verify the logic we create in `lib.rs` is correct.

Next, open up `src/lib.rs`.  This is where we'll define our library of types and trait implementations.  Make it look like this:

```rust
#[cfg(test)]
mod test;

pub const GREETING: &'static str = "Cool Tunes (tm)";
```

Next, create a new file at `src/test.rs` to hold the test module:

```rust
use super::*;
use pretty_assertions::assert_eq;

#[test]
fn cool_greeting() {
    assert_eq!(GREETING, "Cool Tunes (tm)");
}
```

Anything in this file marked `#[test]` will run as a test.  This file is only compiled and run when you run `cargo test`.  Try it now - the first build will take the longest:

```txt
$ cargo test
   Compiling music v0.1.0 (C:\Users\you\code\music)
    Finished dev [unoptimized + debuginfo] target(s) in 0.53s
     Running target\debug\deps\music-b4c9ecce4c26cf65.exe

running 1 tests
test test::cool_greeting ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target\debug\deps\mod-d894091c6e952d62.exe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests music

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Finally, create a directory called `src/bin`.  This optional module is where Cargo will by default expect an executable, if present.  Our program will include a command-line interface to interact with the types we define.  Logic concerned with tht interface will live here.  Place a file at `src/bin/mod.rs`:

```rust
use music::*;

fn main() {
    println!("{}", GREETING);
}
```

We can access any types we define in `src/lib.rs` here by importing everything there (at, least, things that are marked `pub`, or public) at the top via the project name.  Give it a go with `cargo run`:

```txt
$ cargo run
   Compiling music v0.1.0 (C:\Users\you\code\music)
    Finished dev [unoptimized + debuginfo] target(s) in 0.57s
     Running `target\debug\mod.exe`
Cool Tunes (tm)
```

The *coolest* tunes.  Your `music` directory should look something like the following:

```txt
│   .gitignore
│   Cargo.lock
│   Cargo.toml
│
└───src
    │   lib.rs
    │   test.rs
    │
    └───bin
            mod.rs
```

This is a good time for an initial commit:

```txt
$ git add .
$ git commit -m "Initial Commit"
```

You can run a faster compilation pass with `cargo check` if you just want the compiler to verify your code's integrity, not produce a binary.

Now that everything has a place to go, let's source us some bytes.

### Random Bytes

*[top](#table-of-contents)*

The first part of the one-liner is  `cat /dev/urandom | hexdump -v -e '/1 "%u\n"'`, which gets a source of random bytes (8-bit binary values) and shows them to the user formatted as base-10 integers.  The `rand` crate can give us random 8-bit integers out of the box by ["turbofish"](https://docs.serde.rs/syn/struct.Turbofish.html)ing a type: `random::<u8>()` will produce a random [unsigned](https://en.wikipedia.org/wiki/Signedness) [8 bit](https://en.wikipedia.org/wiki/8-bit) integer ([`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) with the default generator settings.  The following snippet does the same thing as `cat /dev/urandom | hexdump -v -e '/1 "%u\n"'`:

```rust
use rand::random;

#[derive(Default)]
struct RandomBytes;

impl RandomBytes {
    fn new() -> Self {
        Self::default()
    }
}

impl Iterator for RandomBytes {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        Some(random::<Self::Item>())
    }
}

fn main() {
    let mut rands = RandomBytes::new();
    loop {
        println!("{}", rands.next().unwrap());
    }
}
```

Give that a go with `cargo run` - exciting stuff.  You should see random integers 0-255 until you kill the process, matching the second command run in the video demonstration (or your own terminal, try it).  What is it we're randomizing, though?

### Mapping Bytes To Notes

*[top](#table-of-contents)*

Take a closer look at step 3 of the pipeline.  Of all the steps, that code most closely resembles what we ultimately end up with:

```bash
split("0,2,4,5,7,9,11,12",a,",");
for (i = 0; i < 1; i += 0.0001)
    printf("%08X\n",
           100 * sin(1382 * exp((a[$1 % 8] / 12) * log(2)) * i))
```

This is probably still not too helpful for most - there's [magic numbers](https://en.wikipedia.org/wiki/Magic_number_(programming)) and [sines](https://en.wikipedia.org/wiki/Sine) and [logarithms](https://en.wikipedia.org/wiki/Logarithm) (oh, my) - and its written in freakin' [`AWK`](https://en.wikipedia.org/wiki/AWK).  Don't despair if this still doesn't mean much (or literally anything) to you.  We're going to explicitly define all constituent components and their relationships, and by the time we get to the real logic it will all just already work.

We can glean a bit of information at a glance, though, and depending on your current comfort with this domain you may be able to kind of understand the general idea here.  It looks like we're going to tick up floating point values by ten-thousandths from zero to one (`0.0`, `0.0001`, `0.0002`, etc.) with `for (i = 0; i < 1; i += 0.0001)`, and do... I don't know, some math - `100 * sin(1382 * exp((a[$1 % 8] / 12) * log(2)) * i)` - on each value.  In that math we're using both `i`, the current fractional part from 0 to 1, and `$1`, which is the random 8-bit integer being piped in.  Specifically, we're indexing into a list `a`:  `a[$1 % 8]`.  In other words, we're using the random byte `0-255` to select an index `0-7` from this list.  The list is defined with `split("0,2,4,5,7,9,11,12",a,",");`, which means split the first parameter string input by the third parameter  `","`, and store the resulting list of elements to the second parameter `a` (`awk` is terse).  After we do the math, we're going to print it out as an 8-digit hex number: `printf("%08X\n", someResult)` - this [`printf`](https://en.wikipedia.org/wiki/Printf_format_string) formatter means we want a [0-padded](https://en.wikipedia.org/wiki/Npm_(software)#Notable_breakages) number that's 8 digits long in [upper-case](https://en.wikipedia.org/wiki/Letter_case) [hexadecimal](https://en.wikipedia.org/wiki/Hexadecimal).  The [base 10](https://en.wikipedia.org/wiki/Decimal) integer [`42`](https://en.wikipedia.org/wiki/Phrases_from_The_Hitchhiker%27s_Guide_to_the_Galaxy#Answer_to_the_Ultimate_Question_of_Life,_the_Universe,_and_Everything_(42)) would be printed as `0000002A`.

TL;DR for each ten-thousandth between 0 and 1 `i`, select a value `n` from `[0,2,4,5,7,9,11,12]` and return the result of `100 * sin(1382 * exp((n / 12) * log(2) * i)`.

If you recognize this formula, awesome!  You can probably skim the next section.  If not, it's still [not time to panic](https://en.wikipedia.org/wiki/Phrases_from_The_Hitchhiker%27s_Guide_to_the_Galaxy#Don't_Panic).  We just need to get some fundamentals out of the way.

#### A Little Physics

*[top](#table-of-contents)*

[Sound](https://en.wikipedia.org/wiki/Sound) is composed physically of [vibrations](https://en.wikipedia.org/wiki/Vibration).  These vibrations cause perturbations in some [medium](https://en.wikipedia.org/wiki/Transmission_medium), which radiate out from the source of the vibration, and those perturbations cause [tiny oscillating variations](https://en.wikipedia.org/wiki/Sound_pressure) in local atmospheric pressure.  These variations are what we experience as sound.  When we're talking about [hearing](https://en.wikipedia.org/wiki/Hearing) a sound with our [ears](https://en.wikipedia.org/wiki/Ear), the medium is usually [air](https://en.wikipedia.org/wiki/Atmosphere_of_Earth).

##### Sine Waves

*[top](#table-of-contents)*

Sound propagates as a [wave](https://en.wikipedia.org/wiki/Wave).  In [reality](https://en.wikipedia.org/wiki/Reality) a sound contains many components but for this program we can talk about a super-simplified version that can be represented as a single [sine wave](https://en.wikipedia.org/wiki/Sine_wave):

![sine waves](https://upload.wikimedia.org/wikipedia/commons/6/6d/Sine_waves_different_frequencies.svg)

If the x-axis is time, a sine wave represents a recurring action with a smooth (or analog) oscillation between peaks.  Lots of physical phenomena are analog in nature - picture a ball getting tossed, rising and then falling.  The ball passes through every point in between the highest point it hits and the ground, so we can measure at any arbitrary instant an exact fractional height.  It doesn't fall from 8 meters to 7 meters all at once, it passes through 7.9, 7.8, 7.7, and all infinitesimally small heights in between too.  It's the same with sound.

Instead of height above the ground on the y axis, we have a [pressure gradient](https://en.wikipedia.org/wiki/Sound_pressure) from an equilibrium.  The air is getting rapidly pushed and pulled by this vibration across space as a wave.  It's still a physical phenomenon - a pressure gradient rises to a peak and then falls back to equilibrium and then below to an opposite peak, oscillating back and forth.  It doesn't just magically become a different higher value all at once.  A guitar string wobbling passes through each point in space between the two extremes it's tensing to and from, so the vibrations it causes oscillate in kind.

You can actually use [math](https://en.wikipedia.org/wiki/Fourier_transform) to represent multi-component sound waves as a single wave - the ability to do so is what enables the whole field of [telecommunications](https://en.wikipedia.org/wiki/Telecommunication).  We're not going to touch that today, partially because I don't actually know how to perform a Fourier transform myself (yet) - some of you may have learned this in high school or college [calculus](https://en.wikipedia.org/wiki/Calculus) classes, but I'm not personally formally STEM-educated and never got there in school.  One single sine wave is enough of a signal to produce a tone, so we'll keep it simple for today and I'll hit the books for next time.

There are two interesting properties of a sine wave: the [amplitude](https://en.wikipedia.org/wiki/Amplitude), which measures the current deviation from the 0 axis for a given *x*, and the [frequency](https://en.wikipedia.org/wiki/Frequency), which is how close together these peaks at maximal amplitudes are, or how frequently this recurring thing happens.  The combination of the two dictate how we perceive the sound.  The amplitude will be perceived as [volume](https://en.wikipedia.org/wiki/Loudness) and the frequency as [pitch](https://en.wikipedia.org/wiki/Pitch_(music)).

You can do cool things like frequency modulation and amplitude modulation to encode your signal as modulations of one of these properties:

![modulation](https://upload.wikimedia.org/wikipedia/commons/a/a4/Amfm3-en-de.gif)

This is how FM and AM radio process incoming sound signals to broadcast them to your radio, which can then perform the reverse and play back the original sound.   We also don't do any of that today, but you could experiment with these functions with this as a base.

##### Pitch

*[top](#table-of-contents)*

The standard unit for frequency is the [Hertz](https://en.wikipedia.org/wiki/Hertz), abbreviated `Hz`, which measures the *number of cycles per second*.  One cycle here is the distance (or time) between two peaks on the graph, or the time it takes to go all the way around the circle once:

![cycle gif](https://media.giphy.com/media/F5rQlfTXqCJ8c/giphy.gif)

According to my super scientific smartphone stopwatch observations, this gif is chugging along at a whopping 0.2Hz.

Recall above that we saw we're going to run a loop like this:  `for (i = 0; i < 1; i += 0.0001)`.  In that loop, the math we process includes the function `sin()`.  If one were to, say, calculate a bunch of points along a single cycle of a sine wave like this one, it sure seems like just such a loop could get the job done.

The higher the frequency, or closer together the peaks of (maximum positive amplitudes), the higher the pitch.

![frequency](https://upload.wikimedia.org/wikipedia/commons/e/ea/Wave_frequency.gif)

Sound is a continuous spectrum of frequency, but when we make music we tend to prefer [notes](https://en.wikipedia.org/wiki/Musical_note) at set frequencies, or pitches.  I'm using [fundamental frequency](https://en.wikipedia.org/wiki/Fundamental_frequency) and pitch interchangeably, because for this application specifically they are, but go Wiki-diving if you want to learn about the distinction and nuance at play here.  The nature of sound is super cool but super complex and outside of the scope of this post - we just want to hear some numbers sing, we don't need to hear a full orchestra.

One of the super cool things about it is the [octave](https://en.wikipedia.org/wiki/Octave).  Octaves just sound related, you know?

// TODO embed octave sound

It turns out the relationship is physical - to increase any pitch by an octave, you double the frequency.  Not only that, this fixed ratio actually holds for any arbitrary smaller or larger interval as well.  This system is called ["equal temperament"](https://en.wikipedia.org/wiki/Equal_temperament) - every pair of adjacent notes has the same ratio, regardless of how you define "adjacent".  To get halfway to the next octave, you multiply by 1.5 instead of 2.

To start working with concrete numbers, we need some sort of standard to base everything around.   Some of the world has settled on [440Hz](https://en.wikipedia.org/wiki/A440_(pitch_standard)) - it's [ISO](https://en.wikipedia.org/wiki/International_Organization_for_Standardization) [16](https://www.iso.org/standard/3601.html), at least.  It's also apparently called "The Stuttgart Pitch", which is funny.

![stuttgart](https://i.imgflip.com/3h0y3g.jpg)

Let's set up a type to represent a pitch:

```rust
type Hertz = f64;
const STANDARD_PITCH: Hertz = 440.0;

#[derive[Debug, Clone, Copy]
struct Pitch {
    frequency: Hertz,
}

impl Pitch {
    fn new(frequency: Hertz) -> Self {
        Self { frequency }
    }
}

impl Default for Pitch {
    fn default() -> Self {
        Self { frequency: STANDARD_PITCH }
    }
}
```

With this code we can use `Pitch::default()` to get our A440 pitch, or pass an arbitrary frequency: `Pitch::new(261.626) // Middle C`.

##### Singing

*[top](#table-of-contents)*

Knowing what frequency to use to produce a given pitch is all well and good, but we need to actually make the sound.  When we sing with our [voice](https://en.wikipedia.org/wiki/Human_voice), our [speech organs](https://en.wikipedia.org/wiki/Speech_organ) vibrate to produce complex multiple-component sound waves of differing frequencies.  We can program ourselves a little one-frequency "speechbox" that produces a wave programmatically instead of by physically vibrating.  To do so, we're going to [graph](https://en.wikipedia.org/wiki/Graph_of_a_function) a function of a single cycle of the target sine wave and [sample](https://en.wikipedia.org/wiki/Sampling_(signal_processing)) it.

TO do so, we need to perform an [analog-to-digital conversion](https://en.wikipedia.org/wiki/Analog-to-digital_converter).  That's a super fancy term for something that isn't that complicated conceptually.  If you already know how we're doing this part, feel free to skip this explanation.

A sine wave, as we've seen, is smooth.  However, what's a graph but a visualization of a function.  There's some function `mySineWave(x) = x` that's this wave when we put in a bunch of fractional numbers between *x* and *x*.  Each time we're back at `x` is the top of the circle - back at one, and it's gonna cycle at *x*Hz (by definition).  The  `for (i = 0; i < 1; i += 0.0001)` loop is doing exactly that, calculating a series of adjacent points at a fixed interval (`0.0001`) that satisfy the function of this wave.  That's our analog-to-digital conversion  - we've taken something smooth, a sine wave, and made it digital, or made up of discrete points.

There's a number of channels meaning is the number of frequencies - that's a cool jumping off point, but we're just working with one.  The sample rate is how many points to store each cycle, which is how high-fidelity this "digital snapshot" of the wave is.  Lots of applications use a [44.1KHz](https://en.wikipedia.org/wiki/44,100_Hz) [sample rate](https://en.wikipedia.org/wiki/Sampling_(signal_processing)#Sampling_rate) - a bit higher than 10KHz like the example.  According to the [sampling theorem](https://en.wikipedia.org/wiki/Nyquist%E2%80%93Shannon_sampling_theorem), the threshold for ensuring you've captured a sufficient sample from an analog signal is that the sample rate must be greater than twice the frequency you you're sampling.  Humans can hear about 20Hz to 20,000Hz.  This means we need at least 40,000 samples, and 44,100 exceeds that.  I [don't understand](https://en.wikipedia.org/wiki/Transition_band) the reason for the specific 4.1k overage, but it's The Standard. Similarly, [16-bit samples](https://en.wikipedia.org/wiki/Audio_bit_depth) is commonly seen thing, so who am I to say otherwise.  In this application, we're using 4.8KHz.  The maximum amplitude this struct can represent is the maximum wave that fits in a 16-bit sample, because that's the biggest *x* will ever be in either direction - `1` or `-1`.

The `rodio` crate actually has a built-in [`rodio::source::SineWave`](https://docs.rs/rodio/0.10.0/rodio/source/struct.SineWave.html).  We could give ourselves a `From` implementation to play theirs - this code should produce an A440 tone:

```rust
use rodio::{Sink, source::SineWave, default_output_device};

fn main() {
    let device = default_output_device().unwrap();
    let sink = Sink::new(&device);
    let source = SineWave::from(STANDARD_PITCH);
    sink.append(source);
    sink.sleep_until_end();
}
```

This source produces an infinite sound source at the given frequency a 48KHz sample rate.  Check out this section of the [source code](https://docs.rs/rodio/0.10.0/src/rodio/source/sine.rs.html#24) from the `rodio` crate for `SineWave`:

```rust
impl Iterator for SineWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let value = 2.0 * 3.14159265 * self.freq * self.num_sample as f32 / 48000.0;
        Some(value.sin())
    }
}
```

This `impl Iterator` block is handling the `for` loop in the cover image.  It's calculating the exact amplitude of a sine wave at some fractional point between 0 and 1.

The math, in other words, is `440.0 * Pi * (current sample / total samples)`, multiplied by some value, in this case `2.0`.  This code is calculating the sine wave at a given point within a cycle - for 0 to 1, there are 48,000 points to collect, so the current point is the sine wave of this frequency at whatever point we're at, stored as `self.num_sample`, between 0 and 1.

For some reason they've hardcoded [Pi](https://en.wikipedia.org/wiki/Pi), there are constants available like [`std::f64::consts::PI`](https://doc.rust-lang.org/std/f64/consts/constant.PI.html).  I'd be interested to know if anyone would know why that's a good choice instead of relying on the language constant!

We can go ahead and throw a quick conversion in for our `Pitch` type - could be useful for testing:

```rust
// lib.rs
impl From<Pitch> for SineWave {
    fn from(p: Pitch) -> Self {
        SineWave::new(f64::from(p) as u32)
    }
}
```

Now we can play the same tone using our own toolkit:

```diff
  fn main() {
      let device = default_output_device().unwrap();
      let sink = Sink::new(&device);
-     let source = SineWave::from(STANDARD_PITCH);
+     let source = SineWave::from(Pitch::default());
      sink.append(source);
      sink.sleep_until_end();
  }
```

Much better.  I'll briefly cover the other tidbits: `default_output_device()` attempts to find the running system's currently configured default audio device, and a [`Sink`](https://docs.rs/rodio/0.10.0/rodio/struct.Sink.html) is an abstraction for handling multiple sounds.  It works like an audio track.  You can `append()` a new `Source` of sound, and the first one appended starts the track.  A newly appended track will play after whatever is playing finishes, but a `rodioL::source::SineWive` is an infinite source.

Finally, we have to `sleep_until_end()` the thread until the sound completes playing (which for `SineWave` is never), or else the program will move right along and exit.  You'll have to kill this run with `Ctrl-C`, this sound will play forever.

One way we could solve this **whole shindig** is by simply modulating the pitch passed to `SineWave` based on the intervals we already went over.  And, like, *cool*, I guess.  We can do a lot better, though.  What exactly do those intervals mean?

#### A Little Music Theory

*[top](#table-of-contents)*

While it's great to have a voice we can sing with with, I'm sure we'd all prefer it if our program learned how to sing on key.  To get oriented, A440 is the A above Middle C on a piano:

![piano](https://upload.wikimedia.org/wikipedia/commons/thumb/2/2e/Piano_Frequencies.svg/2560px-Piano_Frequencies.svg.png)

The cyan key is Middle C, and A440 is highlighted in yellow.  The octaves on an 88-key piano are numbered as shown, so often A440 is simply denoted "A4" especially when dealing with a keyboard.  You may own a tuner that marks 440Hz/A4 specifically if you're a musician.  This pitch is used for calibrating musical instruments and tuning a group, as well as a baseline constant for calculating frequencies.

Note how each octave starts at C, not A, so A4 is actually higher in pitch than C4.  Octaves are "C-indexed" and base 8: `C D E F G A B C` is the base unmodified scale.

##### Scales

*[top](#table-of-contents)*

A [scale](https://en.wikipedia.org/wiki/Scale_(music)) is a series of notes (frequencies) defined in terms of successive intervals from a base note.  The smallest of these intervals on a piano (and most of Western music) is called a [semitone](https://en.wikipedia.org/wiki/Semitone), also called a minor second or half step.  We'll need to keep track of these as the basic unit of a keyboard interval:

```rust
struct Semitones(i8);
```

Take a look back at that piano diagram above - one semitone is the distance between two adjacent keys.  A *whole* step, or a [major second](https://en.wikipedia.org/wiki/Major_second), is equal to two semitones, or two adjacent white keys that pass over a black key.  To play from C4 to C5, you'll use 12 keys (count all the white and black keys in a bracket), so octaves are divided into 12 equal semitones.  There's a name for [each interval](https://en.wikipedia.org/wiki/Interval_(music)#Main_intervals):

```rust
#[derive(Debug, Clone, Copy)]
enum Interval {
    Unison = 0,
    Min2,
    Maj2,
    Min3,
    Maj3,
    Perfect4,
    Tritone,
    Perfect5,
    Min6,
    Maj6,
    Min7,
    Maj7,
    Octave,
}
```

By including a numeric index with `Unison = 0`, each variant also gets assigned the next successive ID.  This way we can refer to each by name but also get an integer corresponding to the number of semitones when needed: `Interval::Maj2 as i8` returns `2_i8`.

Clearly, there isn't a black key between every white key.  The piano is designed to play notes from a category of scales called [diatonic scales](https://en.wikipedia.org/wiki/Diatonic_scale), where the full range of an octave consists of five whole steps and two half steps.  We can see this visually on the keyboard - it has the same 8-length whole/half step pattern all the way through.  The distribution pattern begins on C, but the keyboard itself starts at A0 and ends at C8.  A piano is thus designed because it can play music across the full range of diatonic stales.  This is where we get those base 8 sequences.

That pattern, that the numbering system is based around, is the C [major scale](https://en.wikipedia.org/wiki/Major_scale).  Start at Middle C, the one highlighted in cyan above, and count up to the next C key, eight white keys to the left.  Each time you skip a black key is a whole step and if the two white keys are adjacent it's a half step.  These are the steps you get counting up to the next C, when the pattern repeats.  This totals 12 semitones per octave:

```txt
whole, whole, half, whole, whole, whole, half
  2  +  2   +  1  +   2   +  2  +   2  +  1   =  12  
```

TODO embed sound

It is somewhat arbitrary at least mathematically to pin C as the base of the system - doing the same exercise with the same intervals starting on a different while key will also produce a major scale but you will start using the black keys to do so.  C is the note that allows you to stick to only white keys with this interval pattern, or has no sharps or flats in the key signature.

There are a few variations of *minor* scale, but for now I'll define the [natural minor scale](https://en.wikipedia.org/wiki/Minor_scale#Natural_minor_scale).  This is what you get if you start at our good old friend A4 and march on up the white keys to A5:

```txt
whole, half, whole, whole, half, whole, whole
```

TODO embed sound

It's the same pattern, just starting at a different offset.  You can play a corresponding minor scale using only the white keys by simply starting at the sixth note of the C major scale, which is A.  Try counting it out yourself up from A4.

##### Cents

*[top](#table-of-contents)*

These discrete units are useful for working with a keyboard, but as we know, sound is analog and continuous.  We need to subdivide these intervals even more granularly, and because of equal temperament we're free to do so at any arbitrary level.  Beyond the twelve 12 semitones in an octave, each semitone is divided into 100 [cents](https://en.wikipedia.org/wiki/Cent_(music)).  This means a full octave, representing a 2:1 ratio in frequency, spans 1200 cents, and each cent can be divided without losing the ratio as well if needed:

```rust
struct Cents(f64);
```

I didn't just assign aliases as with `type Hertz = f64`, because I need to re-define how to convert to and from `Cents` and `Semitones` with the [`From`](https://doc.rust-lang.org/std/convert/trait.From.html) [trait](https://doc.rust-lang.org/book/ch10-02-traits.html).  For that, I need my very own type, not just an alias of a primitive that already can convert to and from other primitives with the standard logic.  `Semitones` to `Cents` is not the same thing as `i8` to `f64`, we have a conversion factor.   The [tuple struct](https://doc.rust-lang.org/1.37.0/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types) syntax is perfect for that.  I kept Hertz as an alias because it really is a more general unit of frequency.  It made sense to me to separate that concept from a `Pitch` that can be modulated by `Cents`.

Now, bear with me - we're going to do a little plumbing to let ourselves work at this higher level of abstraction.  We can give ourselves some conversions to the inner primitive:

```rust
impl From<Cents> for f64 {
    fn from(cents: Cents) -> Self {
        cents.0
    }
}

impl From<Semitones> for i8 {
    fn from(semitones: Semitones) -> Self {
        semitones.0
    }
}
```

Now we can encode the conversion factor:

```rust
const SEMITONE_CENTS: Cents = Cents(100.0);

impl From<Semitones> for Cents {
    fn from(semitones: Semitones) -> Self {
        Cents(i8::from(semitones) as f64 * f64::from(SEMITONE_CENTS))
    }
}
```

We can also map our `Interval` variants directly `Semitones`, to make sure they're always turned into `Cents` correctly:

```rust
impl From<Interval> for Semitones {
    fn from(i: Interval) -> Self {
        Semitones(i as i8)
    }
}
```

With that, it's easy to map `Interval`s to `Cents`:

```rust
impl From<Interval> for Cents {
    fn from(i: Interval) -> Self {
        Semitones::from(i).into()
    }
}
```

Phew!  Lots of code, but now we can operate directly in terms of `Interval` variants or anything in between and everything stays contextually tagged.

There's one more step to get from our brand new floating point `Cents` to frequencies in `Hertz` though.  Remember how Middle C was some crazy fraction, 261.626Hz?  This is because cents are a [logarithmic](https://en.wikipedia.org/wiki/Logarithmic_scale) unit, standardized around the point 440.0.  While a 2:1 ratio is nice and neat, we've been subdividing that arbitrarily wherever it makes sense to us.  Now the arithmetic isn't always so clean.  Doubling 440.0Hz will get 880.0Hz, but how would we add a semitone?  It's 100 cents, nice and neat, and there are 12 semitones - so we'd need to increase by a 12th of what doubling the number would do: `440 * 2^(1/12)`.  Looks innocuous enough, but my calculator gives me 466.164, Rust gives me 466.1637615180899 - not enough to perceptually matter, but enough that it's important that the standard is the interval ratio and not the specific amount of Hertz to add or subtract.  Those amounts will only be precise in floating point decimal representations at exact octaves from the base note, because that's integral factor after multiplying by 1 in either direction, 2 or 1/2.

Otherwise stated, the ratio between frequencies separated by a single cent is the 1200th root of 2, or 2^(1/1200).  In decimal, it's about 1.0005777895.  You wouldn't be able to hear a distinction between two tones a single cent apart.  The [just-noticeable difference](https://en.wikipedia.org/wiki/Just-noticeable_difference) is about 5 or 6 cents, or 5*2^(1/1200).  Using this math, it works out to just shy of 4 cents to cause an increase of 1Hz, more precisely around 3.9302 for a base frequency of 440.0.

Logarithmic units are helpful when the range of the y axis, in our case frequency, increases exponentially.  We know the graph of frequency to pitch does because to jump by any single octave, we double what we have - we're multiplying at each step, not adding (which results in a linear graph).  A4 is 440Hz, A5 is 880Hz, and by A6 we're already at 1,760Hz.  The graph `f(x) = x^2` looks like this:

![x_squared](https://thepracticaldev.s3.amazonaws.com/i/mkh095mgcasg1soygrb7.png)

A [logarithm](https://en.wikipedia.org/wiki/Logarithm) is the inverse of an [exponent](https://en.wikipedia.org/wiki/Exponentiation).  Our ratio had an exponent that was "1 divided by n", which is the inverse of raising something to the power of "n divided by 1", such as squaring it (n=2).  This is otherwise written as an "nth root", in the case of a cent *n* being 1,200.  This counteracts the rapid growing curve we get by constantly squaring the frequency into a more linear scaled subdivision between octaves:

![cent graph](https://upload.wikimedia.org/wikipedia/commons/thumb/3/3f/Music_intervals_frequency_ratio_equal_tempered_pythagorean_comparison.svg/550px-Music_intervals_frequency_ratio_equal_tempered_pythagorean_comparison.svg.png)

Notice it's not a straight diagonal - we haven't removed the squaring from the system, merely adjusted for it. We're taking a logarithm of something that has been squared, the frequency.  This tames the steep increase but the line is still slightly curved.  Fractional cents and tones are a much better way to deal with intervals than by concrete frequency deltas.  Knowing all this we can translate back to the frequency in Hertz of a desired pitch if we know both a base frequency and the number of cents to increase by:

![cents formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/920411bb22d357b13f69a76fa33557c707f7cb57)

Here, *a* is the initial frequency in Hertz, *b* is the target frequency, and *n* is the number of cents by which to increase *a*.

Time for the plumbing - we want to be able to ergonomically manipulate a `Pitch` in terms of this new logarithmic scale, and not worry about the specifics.  It looks like we're going to need to divide some `Cents`, using the `impl From<Cents> for f64` blocks we defined:

```rust
use std::ops::Div;

impl Div for Cents {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Cents(f64::from(self) / f64::from(rhs))
    }
}
```

This is just performing floating point division on the inner value, but keeps it wrapped up in the `Cents` context for us so we can directly use `Cents(x) / Cents(y)`.  There are a lot of unit conversions throughout this program but *all* of them are explicit and defined where we expect them.  We now know enough to manipulate a `Pitch` directly.

The [`AddAssign`](https://doc.rust-lang.org/std/ops/trait.AddAssign.html) trait gets us the `+=` operator, and can define it for any type we want on the right hand side:

```rust
use std::ops::AddAssign

impl AddAssign<Cents> for Pitch {
    fn add_assign(&mut self, rhs: Cents) {
        self.frequency *= 2.0_f64.powf((rhs / Cents::from(Interval::Octave)).into())
    }
}
```

If that's not quite clear, this is the exact equation shown above with a bit of extra noise.  Dividing `cents` by `Cents::from(Interval::Octave)` leaves us with a `Cents` type, per the above `impl Div for Cents` block.  However, we then want to pass the result to `2.0.powf(cents_ratio)`.  The compiler knows it's an `f64` here because we explicitly specified it with `2.0_f64` to use [`powf()`](https://doc.rust-lang.org/std/primitive.f64.html#method.powf).  This is a method on `f64`, so `self` in `pub fn powf(self, n: f64) -> f64` is an `f64` when this gets run.  We implemented `From<Cents> for f64` already, which provides the `T::into() -> U` method like above for free as well as `U::from(T) -> U` in any context where the target type can be inferred, like right here.

The `2.0_f64` literal is how we specify a concrete type - a `2.0` literal is just a `{float}` and stills need more context for the compiler to make into a `f32` (a.k.a. `float`) or `f64` (a.k.a. `double`) and use.  This is usually what we want, it gives us the flexibility to use this literal in any floating point context, but it also doesn't carry that information to help out with inference for other types.  Any numeric literal can take a concrete type suffix.   For example, `8` is an unqualified `{integer}` that can coerce to any unsigned integer type (`u8`,`u16`, `u32`, `u64`, or machine-dependent pointer-sized `usize`) until it's used in a specific context, but `8u8` begins its life as a `u8`, fully specified, so we know we can safely cast it to any larger type.  Any `_` found in a numeric literal is ignored and there for clarity - a 48KHz sample rate could be written `48_000.0_f64`.

Lets try to increase by a single Hertz using the value above:

```rust
fn main() {
    let mut pitch = Pitch::default();
    println!("{:?}", pitch); // Pitch { frequency: 440.0 }
    pitch += Cents(3.9302); // attempt to add one Hz
    println!("{:?}", pitch); // Pitch { frequency: 441.0000105867894 } - close enough
}
```

Instead of adding single cents at a time, it's easier to work by semitone - luckily that's pretty easy now:

```rust
impl AddAssign<Semitones> for Pitch {
    fn add_assign(&mut self, semitones: Semitones) {
        *self += Cents::from(semitones)
    }
}

fn main() {
    let mut pitch = Pitch::default();
    println!("{:?}", pitch); // Pitch { frequency: 440.0 }
    pitch += Semitones::from(Interval::Octave);
    println!("{:?}", pitch); // Pitch { frequency: 880.0 } - 2:1 ratio
}
```

Why not just go straight for intervals:

```rust
impl AddAssign<Interval> for Pitch {
    fn add_assign(&mut self, i: Interval) {
        *self += Cents::from(i)
    }
}

fn main() {
    let mut pitch = Pitch::default();
    println!("{:?}", pitch); // Pitch { frequency: 440.0 }
    pitch += Interval::Min2; // Add a semitone
    println!("{:?}", pitch); // Pitch { frequency: 466.1637615180899 }
}
```

##### Scientific Pitch Notation

*[top](#table-of-contents)*

Armed with this knowledge, we can start manipulating pitches in terms of [Scientific Pitch Notation](https://en.wikipedia.org/wiki/Scientific_pitch_notation), another fancy name for a simple concept.  The piano keyboard above was labelled according to this standard - "A4" for example.  A standard pitch is composed of two components: a note from A-G with an optional accidental and a 0-indexed octave:

```rust
#[derive(Default, Debug, Clone, Copy)]
struct StandardPitch {
    note: Note,
    octave: u8,
}
```

To show them, we just want to print them out next to each other - the first three should be `C0 C#0 D0`:

```rust
impl fmt::Display for StandardPitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.note, self.octave)
    }
}
```

The octave just starts at 0 and won't ever realistically rise above 255, so a `u8` is fine.  A note consists of a letter and optionally an accidental:

```rust
#[derive(Default, Debug, Clone, Copy)]
struct Note {
    accidental: Option<Accidental>,
    letter: NoteLetter,
}
```

For this one, we only want to display a character for an accidental if there's anything there:

```rust
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let acc_str = if let Some(a) = self.accidental {
            format!("{}", a)
        } else {
            "".to_string()
        };
        write!(f, "{:?}{}", self.letter, acc_str)
    }
}
```

The accidental itself is a simple switch:

```rust
#[derive(Debug, Clone, Copy)]
enum Accidental {
    Flat,
    Sharp,
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Accidental::*;
        let acc_str = match self {
            Flat => "♭",
            Sharp => "#",
        };
        write!(f, "{}", acc_str)
    }
}
```

The [accidentals](https://en.wikipedia.org/wiki/Accidental_(music)) are represented in strings as `♭` for flat or `#` for sharp, which lower or raise the note by one semitone (or `Interval::Min2`) respectively.  This does produce 14 possible values for 12 possible semitones - the exceptions are wherever there's no black key in between two white keys.  `F♭` should parse as `E` and `B#` should parse as `C`.  Thankfully, we've already programmed it to do that!  // TODO have we??

There is third accidental called "natural", `♮`, which cancels these out.  To represent a pitch in data we don't need it - that's a string-parsing concern.  The natural symbol is generally used for overriding a [key signature](https://en.wikipedia.org/wiki/Key_signature), which defines the default accidental for all the notes within a scale on [sheet music](https://en.wikipedia.org/wiki/Staff_(music)).  There are a series of accidentals on the margin of the staff that apply to all notes, which is how we ensure we play notes within a single given scale, or [key](https://en.wikipedia.org/wiki/Key_(music)).  However, you may choose to compose a melody that contains a note outside this key.  If we encounter something like `F#♮`, we just care that it's F.  In fact, we can stack accidentals as far as we want, we always just store the final net change in `StandardPitch`.

As for `NoteLetter`:

```rust
#[derive(Debug, Clone, Copy)]
enum NoteLetter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Default for NoteLetter {
    fn default() -> Self {
        NoteLetter::C
    }
}
```

// TODO to/from Pitch here

The notes are C-indexed, for better or for worse, so `Note::default()` should return that variant.  We'll talk more about why it's C and not A after learning about Modes below.   Don't worry, it's suitably disappointing.

Thanks to all the nested `Default` blocks, the `Default` implementation that the compiler derives for `StandardPitch` corresponds to the official base pitch of this system, `C0`.  We can use `StandardPitch::default()` to procure one - here's a [playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=943967b13c8c3a5d02201b2adb509371):

```rust
println!("{}", StandardPitch::default()); // C0
```

It's defined at a set frequency:

```rust
const C_ZERO: Hertz = 16.352;
```

This is super low - most humans bottom out around 20Hz.  The 88-key piano's lowest note is up at A0, a 9-semitone [`major sixth`](https://en.wikipedia.org/wiki/Major_sixth) higher.  Note how even though this is a different abstraction for working with pitches, the frequencies baked in to the standard are still pinned to the A440 scale.  Do a quick sanity check before abstracting further:

// TODO this should start with StandardPitch::default() and use get_offset()

```rust
fn main() {
    let mut pitch = Pitch::new(C_ZERO);
    println!("{:?}", pitch); // Pitch { frequency: 16.352 }
    for _ in 0..4 {
        pitch += Semitones::from(Interval::Octave);
    } // add 4 octaves - C0 -> C4
    println!("{:?}", pitch); // Pitch { frequency: 261.632 }
    pitch += Interval::Maj6; // C4 -> A4
    println!("{:?}", pitch); // Pitch { frequency: 440.0108218313197 } // close enough
}
```

Luckily, even being off by a full Hertz at 440 (~4 cents) is less than the just-noticeable difference of ~5-6 cents, so within the ranges we're working with that's not wrong enough to care.  To test, each piano key is [specified](https://en.wikipedia.org/wiki/Piano_key_frequencies)

##### Circle of Fifths

*[top](#table-of-contents)*

Now we can start defining scales.  When I introduced the concept, I noted that using the same intervals as the C major scale starting on a different note will also produce a major scale but you will start using the black keys.  This is called the key signature, and there's one for each variant of the major scale starting from each black key.  They're related by the [circle of fifths](https://en.wikipedia.org/wiki/Circle_of_fifths):

![circle](https://upload.wikimedia.org/wikipedia/commons/3/33/Circle_of_fifths_deluxe_4.svg)

The C major scale has all white keys.  To find the version of the major scale that adds one single black key to augment a tone, you go up 7 semitones: [`Interval::Perfect5`](https://en.wikipedia.org/wiki/Perfect_fifth).  This has a ratio 3:2.

The first major scale around the circle is [G major](https://en.wikipedia.org/wiki/G_major).  It has one sharp: A.  Go [back up](#a-little-music-theory) to the piano diagram and count up the major scale sequence from G, for example one note below the yellow A4.  You'll need the `F#` black key at the last step right before G5, but all the other hops white stick to the white keys.  [D major](https://en.wikipedia.org/wiki/D_major) will need two black keys, `F#` and `C#`.  If you continue incrementing a fifth (remember, octave is irrelevant here), you'll hit all 12 possible patterns before getting back to C.  To get through all the key signatures incrementally, one accidental at a time, you keep going up by perfect fifths.  Once you come all the way back to C, you'll have hit all 12 keys, encompassing all possible key signatures.

This diagram also shows the [relative natural minor](https://en.wikipedia.org/wiki/Relative_key) for each.  We saw how to get [A minor](https://en.wikipedia.org/wiki/A_minor) from C major, so by definition of equal temperament that interval holds all the way around.

It's true that, e.g. `D#` and `E♭` represent the same pitch - what's different is why we got there.  After the midway point, it's easier to denote 5 flats than 7 sharps, even though they mean the same tones - there's only 5 black keys to choose from, after all.

To go counter-clockwise, go up by a perfect fourth every time, which is 5 semitones.  This is known as "circle of fourths", and is more commonly associated with [jazz](https://en.wikipedia.org/wiki/Jazz) music whereas fifths are seen in more [classical](https://en.wikipedia.org/wiki/Classical_music) contexts.

// TODO show off all 12

##### Diatonic Modes

*[top](#table-of-contents)*

We can already produce the 12 transpositions from C.

Similarly, if you don't use the black keys and start on a different note and count up one octave, you will get a *different* diatonic scale, such as the natural minor scale.  These scale variations are called [`Modes`](https://en.wikipedia.org/wiki/Mode_(music)#Modern_modes), and while high-school me was terrified of and terrible at whipping out arbitrary ones on a brass instrument from memory (mental math is *not* one of my talents), they're easy to work with programmatically (and much less stressful).

The first scale I laid out, the major scale, is also known as the [`Ionian mode`](https://en.wikipedia.org/wiki/Ionian_mode).  This is the base mode, each other is some offset from this scale.  As we've seen, the key you need to start on to play this mode with no black keys (accidentals) is C.  The natural minor scale, where we started at A4 and counted up white keys, is called the [`Aeolian mode`](https://en.wikipedia.org/wiki/Aeolian_mode).  There's an absurdly fancy name for each offset.  This means we get our first seven `Scale` variants for free:

```rust
#[derive(Debug, Clone, Copy)]
enum Mode {
    Ionian = 0,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

#[derive(Debug)]
enum Scale {
    Diatonic(Mode),
}
```

We'll hardcode the C major sequence as the base:

```txt
whole, whole, half, whole, whole, whole, half
  2  +  2   +  1  +   2   +  2  +   2  +  1
```

```rust
impl Mode {
    fn base_intervals() -> &'static [Interval] {
        use Interval::*;
        &[Maj2, Maj2, Min2, Maj2, Maj2, Maj2, Min2]
    }
}
```

We've seen something like this somewhere before:

```bash
split("0,2,4,5,7,9,11,12",a,",");
```

What if we represent this octave as a series of semitone offsets from 440Hz:

```txt
whole, whole, half, whole, whole, whole, half
  2  +  2   +  1  +   2   +  2  +   2  +  1
0    2     4      5      7      9     11     12
Un. Min2  Maj2  Perf4  Perf5   Maj6 Maj7   Octave
A4    B4   C4    D4     E4      F4   G4      A5
```

Aha!  It's was an A major scale over one octave this whole time.

// TODO show generated major scale of intervals

The fact that Ionian Mode/C Major is Offset 0 is actually somewhat arbitrary - though definitely not completely.  There's a reason C major is so commonly found in music - it sounds good.  I did a [bare-minimum](https://lmgtfy.com/?q=why+does+it+start+from+C+not+A) amount of research and found it's an ["unfortunate historical accident"](https://music.stackexchange.com/questions/893/why-is-c-the-base-note-of-standard-notation-and-keys), which in retrospect is completely obvious and I should have seen coming.  The letters were originally numbered from A, of course, but got mapped to frequencies well before the modern modes we use now were honed and refined from previous systems.  The system eventually came to be based around the [C major scale](https://en.wikipedia.org/wiki/C_major), not A major.  By then the fact that what's now Middle C was 261.626Hz was long done and over with.

TL;DR the concept of "mode" in an equally tempered system predates the modern scales and `C == 0` is a historical artifact.

##### Non Heptatonic Scales

*[top](#table-of-contents)*

Okay, Ben.  Ben, okay.  Okay, Ben.  We've arrived at the version from the blog post, great.  This whole time, though, the line from the meme image has had something different:

```bash
split("4,5,7,11",a,",");
```

The diatonic scales we've been working with are a subset of the [heptatonic scales](https://en.wikipedia.org/wiki/Heptatonic_scale), with seven notes each.  These tones are naturally further apart than we've been using.  Let's add a couple others scale lengths to play with:

```rust
#[derive(Debug, Clone, Copy)]
enum ScaleLength {
    Tetratonic = 4,
    Pentatonic = 5,
    Heptatonic = 7,
    Dodecatonic = 12,
}
```

Interestingly, the scale shown is [tetratonic](https://en.wikipedia.org/wiki/Tetratonic_scale), given as octave-less notes, intervals from base, and offsets from A440:

```txt
[C#, D, E, G#]
[Min2, Maj2, Maj3]
[4, 5, 7, 11]
```

which is primarily associated with pre-historic music.  Maybe they spoke `AWK`?I also don't understand how that snippet works, because it's still indexed with `a[$1 % 8]`, but I'm too lazy to find out why.

A more common variant is the [pentatonic scale](https://en.wikipedia.org/wiki/Pentatonic_scale), with 5 tones per octave.   There are a number of ways to construct a pentatonic scale, see the link for more, I'll just define one here:

```txt
[E♭, G♭, A♭, B♭, D♭]
[Min3, Min2, Min2, Min3]
[6, 9, 11, 13, 16]
```

This one is fun because it's what you get when you start at `E♭` and only play the *black* keys.  Like the major scales, this type of scale also has modes, one for each black key:

```rust
// TODO PENTATONIC MODES
```

The only dodecatonic scale is the [chromatic scale](https://en.wikipedia.org/wiki/Chromatic_scale) is just all the notes:

```txt
[A, A#, B, C, C#, D, D#, E, F, F#, G, G#]
[Min2, Min2, Min2, Min2, Min2, Min2, Min2, Min2, Min2, Min2, Min2]
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
```

Who needs key signatures anyhow, that's a waste of all these other keys!  This one throws 'em all in the mix.

This could be a potential natural application of [dependent types](https://en.wikipedia.org/wiki/Dependent_type), a programming language feature that Rust does not support.  Few languages do, one example is the [Haskell](https://en.wikipedia.org/wiki/Haskell_(programming_language))-alike [Idris](https://en.wikipedia.org/wiki/Idris_(programming_language)#Dependent_types).  wherein we codify in the type system some restraint further than a type.  The linked example describes a function that appends a list of `m` elements to a list `n` which specifies as part of the return type that the returned list has length `n + m`.  A caller can then trust this fact implicitly, because the compiler won't build a binary if it's not true.  I think this could be applied here to verify that a scale's intervals method returns an octave, regardless of length.  That can be tested for now, but not encoded in the type directly.

##### Key

*[top](#table-of-contents)*

We're finally ready to pull this all together.  For context, once again here's the original line we're dealing with:

```bash
split("0,2,4,5,7,9,11,12",a,",");
```

We've now discovered that that list represents the list of semitone offsets from C4 that represent a C major scale.  The random notes that get produced will all be frequencies that correspond to these offsets from 440Hz.

We way, way overshot this in the process of modelling the domain.  We can now automatically generate sequences of `StandardPitch` structs that correspond to keys on an 88-key piano to select from: `[C4 D4 E4 F4 G4 A5 B5 C5]`, and each note already knows how to calculate it's frequency in Hertz.  If we want a different scale, we can just ask.

Two identical notes are called a [unison](https://en.wikipedia.org/wiki/Unison), with 0 cents.  These intervals are defined within a single octave, so any of them apply across octaves as well - A4 and A5 are in unison just like A4 and another A4, and C4 and A5 is still a major sixth.  The terms "major", "minor", and "perfect" are not arbitrary, but that discussion is outside the scope of this post.  I will note that the [tritone](https://en.wikipedia.org/wiki/Tritone), representing 3 whole tones or 6 semitones like `F-B`, is the only one that's none of the three.

If interested, I recommend [harmony](https://en.wikipedia.org/wiki/Harmony) for your next rabbit hole.  The tritone takes a leading role in [dissonance](https://en.wikipedia.org/wiki/Consonance_and_dissonance), and to hear it in action you should check out what the [Locrian mode](https://en.wikipedia.org/wiki/Locrian_mode) we defined sounds like with this program.  The C major scale has a perfect fifth, 5 semitones at the [dominant](https://en.wikipedia.org/wiki/Dominant_(music)) scale [degree](https://en.wikipedia.org/wiki/Degree_(music)) - and the Locrian mode has a tritone which is one extra semitone.

We don't necessarily want to stick within a single octave, though. We want to use the full 108 keys from C0 to B8 (even larger than the standard piano from the diagram), but only use notes in the key.

// TODO

Alright.  Back to the bytes.

#### Back To The Bytes

*[top](#table-of-contents)*

![human music](https://thepracticaldev.s3.amazonaws.com/i/92xyu0xcenfmpvrf6kbq.gif)

### Listen To Your Files

*[top](#table-of-contents)*

You know what else is a stream of bytes?  Literally everything else.  Who needs `bash`!

TODO maybe?  maybe not?  

### Write Your Own Tunes

*[top](#table-of-contents)*

## Challenges

*[top](#table-of-contents)*

- A [`WAV`](https://en.wikipedia.org/wiki/WAV) file is an uncompressed audio stream.  Write out the digitized waveform you've defined with [`hound`](https://github.com/ruuda/hound).
- Implement `Chord`.
- Add more scales.
- Support [Helmholtz pitch notation](https://en.wikipedia.org/wiki/Helmholtz_pitch_notation).
- Port this program to another language.

This has been Public Service Announcement on the dangers of online encyclopedias.  Thank you for your time.

*Cover image: [reddit](https://www.reddit.com/r/linuxmasterrace/comments/dyqri7/like_god_would_have_wanted/)*
