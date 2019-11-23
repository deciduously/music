---
title: Teaching Numbers To Sing
published: false
description: Learn how to procedurally generate melodies in Rust.
cover_image: https://thepracticaldev.s3.amazonaws.com/i/iuakwwcexql5u0th7gtm.jpg
tags: beginners, rust, tutorial, music
---

# Everything Is Music

TODO TESTS THROUGHOUT

> Everything is music. When I go home, I throw knickers in the oven and it's music. Crash, boom, bang!

*- [Winona Ryder](https://en.wikipedia.org/wiki/Winona_Ryder) as [Björk](https://en.wikipedia.org/wiki/Bj%C3%B6rk) on [SNL](https://en.wikipedia.org/wiki/Saturday_Night_Live)'s [Celebrity Rock 'N' Roll Jeopardy!](https://en.wikipedia.org/wiki/Celebrity_Jeopardy!_(Saturday_Night_Live)) - [2002](https://en.wikipedia.org/wiki/2002) - [YouTube](https://youtu.be/R3V94ZtmdbQ?t=190)*

In this post, we'll [throw](https://en.wikipedia.org/wiki/Throwing) something [random](https://en.wikipedia.org/wiki/Random_number_generation) into, [well](https://en.wikipedia.org/wiki/Well), a [math](https://en.wikipedia.org/wiki/Mathematics)-[oven](https://en.wikipedia.org/wiki/Subroutine) and [*viola*](https://en.wikipedia.org/wiki/Viola), [music](https://en.wikipedia.org/wiki/Music)!  We'll just skip the [crash](https://en.wikipedia.org/wiki/Crash_(computing)).

In other words, we're going to teach our [computers](https://en.wikipedia.org/wiki/Personal_computer) to ["sing"](https://en.wikipedia.org/wiki/Singing) using [Rust](https://en.wikipedia.org/wiki/Rust_(programming_language)), backed by a little light [physics](https://en.wikipedia.org/wiki/Physics) and [music theory](https://en.wikipedia.org/wiki/Music_theory).

The [one-liner](https://en.wikipedia.org/wiki/One-liner_program) in the cover image [procedurally generates](https://en.wikipedia.org/wiki/Procedural_generation) a [melody](https://en.wikipedia.org/wiki/Melody) using [tools assumed to be present](https://en.wikipedia.org/wiki/Unix_philosophy) on a standard [desktop](https://en.wikipedia.org/wiki/Desktop_computer) [Linux](https://en.wikipedia.org/wiki/Linux) [distribution](https://en.wikipedia.org/wiki/Linux_distribution) like [Ubuntu](https://en.wikipedia.org/wiki/Ubuntu).  The melody produced will be composed of notes along a single [octave](https://en.wikipedia.org/wiki/Octave) in a hardcoded [key](https://en.wikipedia.org/wiki/Key_(music)):

{% youtube uLhQQSKhTok %}

By the end of this post we'll have written a program that can procedurally generate music in number of different kinds of musical scale spanning up and down a whole [keyboard](https://en.wikipedia.org/wiki/Musical_keyboard), as well play [hand-authored](https://en.wikipedia.org/wiki/Musical_composition) songs created with a rudimentary [notation system](https://en.wikipedia.org/wiki/Musical_notation) that compiles and runs on [Windows](https://en.wikipedia.org/wiki/Microsoft_Windows), [MacOS](https://en.wikipedia.org/wiki/MacOS), or Linux with no code modification.

[¡Vámonos!](https://en.wikipedia.org/wiki/Party)

## Table of Contents

- [Preamble](#preamble)
- [The Meme](#the-meme)
- [The Program](#the-program)
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
      * [Diatonic Modes](#diatonic-modes)
      * [Other Scales](#other-scales)
    - [Back To The Bytes](#back-to-the-bytes)
  * [Listen To Your Files](#listen-to-your-files)
  * [Write Your Own Tunes](#write-your-own-tunes)
- [Challenges](#challenges)

## Preamble

*[top](#table-of-contents)*

This tutorial is aimed at [beginners](https://en.wikipedia.org/wiki/Novice) (and up) who are comfortable solving problems with at least one [imperative language](https://en.wikipedia.org/wiki/Imperative_programming).  It does not matter if that's [JavaScript](https://en.wikipedia.org/wiki/JavaScript) or [Python](https://en.wikipedia.org/wiki/Python_(programming_language)) or [Object Pascal](https://en.wikipedia.org/wiki/Object_Pascal), I just assume you know the [basic](https://en.wikipedia.org/wiki/Syntax_(programming_languages)) [building](https://en.wikipedia.org/wiki/Semantics_(computer_science)) [blocks](https://en.wikipedia.org/wiki/Standard_library) of [creating a program](https://en.wikipedia.org/wiki/Computer_programming).  You do not need any prior knowledge of physics or music theory, but there will be a tiny smattering of [elementary algebra](https://en.wikipedia.org/wiki/Elementary_algebra).  I promise it's quick.

There's a bunch of fairly [idiomatic](https://en.wikipedia.org/wiki/Programming_idiom) [Rust](https://www.rust-lang.org/) throughout this write-up, but don't worry if that's not what you're here for.  You can choose to skip all the code snippets entirely and still come out knowing how it all works.  It could also be useful for translating to your (second) favorite programming language.  Rust tends to get verbose, but one positive side-effect of that [verbosity](https://en.wikipedia.org/wiki/Verbosity) is that at least the core of what this code does should be easy to follow even without knowing the language.

I have two disclaimers:

1. [There are](https://en.wikipedia.org/wiki/Existence) [145](https://en.wikipedia.org/wiki/145_(number)) [Wikipedia](https://en.wikipedia.org/wiki/Main_Page) [links](https://en.wikipedia.org/wiki/Hyperlink) [here](https://en.wikipedia.org/wiki/Blog).  [If](https://en.wikipedia.org/wiki/Conditional_(computer_programming)) [you're](https://en.wikipedia.org/wiki/You) [that](https://en.wikipedia.org/wiki/Autodidacticism) [kind](https://en.wikipedia.org/wiki/Impulsivity) [of](https://en.wikipedia.org/wiki/Preposition_and_postposition) [person](https://en.wikipedia.org/wiki/Person), [set](https://en.wikipedia.org/wiki/Innovation) [rules](https://en.wikipedia.org/wiki/Law).
1. Further to Point 1, most of this I learned myself on Wikipedia.  The rest is what I remember from [high school](https://en.wikipedia.org/wiki/High_school_(North_America)) as a [band geek](https://en.wikipedia.org/wiki/Euphonium), which was over [ten years](https://en.wikipedia.org/wiki/Decade) [ago](https://en.wikipedia.org/wiki/Past).  I do believe it's generally on the mark, but I am making no claims of authority.  If you see something, [say something](https://en.wikipedia.org/wiki/Allen_Kay#Advertisements).

## The Meme

*[top](#table-of-contents)*

This post was inspired by [this](https://www.reddit.com/r/linuxmasterrace/comments/dyqri7/like_god_would_have_wanted/) [meme](https://en.wikipedia.org/wiki/Internet_meme) I saw when I was *attempting* to casually browse [Reddit](https://en.wikipedia.org/wiki/Reddit):

![the meme](https://i.redd.it/uirqnamnjpz31.jpg)

I (evidently) couldn't let myself just scroll past that one.  Here's a version of the [`bash`](https://en.wikipedia.org/wiki/Bash_(Unix_shell)) [pipeline](https://en.wikipedia.org/wiki/Pipeline_(Unix)) at the bottom with slightly different hard-coded values, taken from [this blog post](https://blog.robertelder.org/bash-one-liner-compose-music/) by [Robert Elder](https://www.robertelder.org/) that explores it:

```bash
cat /dev/urandom | hexdump -v -e '/1 "%u\n"' | awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }' | xxd -r -p | aplay -c 2 -f S32_LE -r 16000
```

The linked blogpost is considerably more brief and assumes a greater degree of background knowledge than this one, but that's not to discredit it at all as a great source of information.  That write-up and Wikipedia were all I needed to complete this translation, and I had absolutely not a clue how this whole thing worked going in.  Reading that post and writing this program taught me a lot of the concepts I'm about to walk through for the first time.

I've gotta be honest - I didn't even try the `bash` and immediately dove into the pure Rust solution.  Nevertheless, it serves as a solid [30,000ft](https://en.wikipedia.org/wiki/Flight_level) [roadmap](https://en.wikipedia.org/wiki/Plan):

1. `cat /dev/urandom`: Get a stream of random binary data.
1. `hexdump -v -e '/1 "%u\n"'`: Convert binary to 8-bit base-10 integers (0-255).
1. `awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }'`: Map integers to pitches, as 8-byte hexadecimal values.
1. `xxd -r -p`: Convert hex numbers back to binary.
1. `aplay -c 2 -f S32_LE -r 16000`: Play back binary data as sound.

Don't worry, you don't need to have a clue how any of it works yet if some or all of this is incomprehensible.  I sure didn't.  I'm not going to do what that [code](https://en.wikipedia.org/wiki/Source_code) does exactly in this post, and I'm not going to elaborate much on what any of the specific commands in the pipeline mean (read the linked post for that).   By the time we're done, you'll be able to pick apart the whole thing yourself anyway.

If you'd like the challenge of implementing this yourself from scratch, **stop right here**.  There's something about extensively modelling a problem space that kinda takes the mystery out, you know?  Try to build the program I described yourself in the language of your choice.  If you get stuck, this should all apply to whatever you've got going unless you've gone real funky with it - in which case, it sounds cool and you should show me.

[¡Vámonos!](https://en.wikipedia.org/wiki/Party)

## The Program

*[top](#table-of-contents)*

As always, ensure you have at least the default stable Rust toolchain [installed](https://www.rust-lang.org/tools/install).  This code was written with `rustc` [version 1.39](https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html) for [Rust 2018](https://doc.rust-lang.org/nightly/edition-guide/rust-2018/edition-changes.html).  

Then, spin up a new project:

```txt
$ cargo new music
```

Open that directory in the environment of your choice.  We'll use three crates to replace the functionality not (quickly) found in the Rust standard library:

* [`rand`](https://docs.rs/rand/0.7.2/rand/) - [Random number generation](https://en.wikipedia.org/wiki/Random_number_generation)
* [`hound`](https://github.com/ruuda/hound) - [WAV stream creation](https://en.wikipedia.org/wiki/WAV)
* [`rodio`](https://docs.rs/rodio/0.10.0/rodio/) - [WAV stream playback](https://en.wikipedia.org/wiki/Audio_signal)

`rand` is in place of [`cat /dev/urandom`](https://en.wikipedia.org/wiki//dev/random), and `hound`/`rodio` will cover [`aplay`](https://linux.die.net/man/1/aplay).  We also use two other crates for quality-of-life Rust stuff:

* [`lazy_static`](https://github.com/rust-lang-nursery/lazy-static.rs) - [Static](https://en.wikipedia.org/wiki/Static_variable) values with [runtime](https://en.wikipedia.org/wiki/Runtime_(program_lifecycle_phase)) [initialization](https://en.wikipedia.org/wiki/Initialization_(programming))
* [`regex`](https://docs.rs/regex/1.1.0/regex/) - [Regular expressions](https://en.wikipedia.org/wiki/Regular_expression)

In `Cargo.toml`:

```toml
[dependencies]

hound = "3.4"
lazy_static = "1.4"
rand = "0.7"
regex = "1.3"
rodio = "0.10"
```

### Random Bytes

*[top](#table-of-contents)*

When I sat down to tackle this whole thing I wrote this struct first.  The first part of the one-liner is  `cat /dev/urandom | hexdump -v -e '/1 "%u\n"'`, which gets a source of random bytes (8-bit binary values) and shows them to the user formatted as base-10 integers.  The `rand` crate can give us random 8-bit integers out of the box by ["turbofish"](https://docs.serde.rs/syn/struct.Turbofish.html)ing a type: `random::<u8>()` will produce a random [unsigned](https://en.wikipedia.org/wiki/Signedness) [8 bit](https://en.wikipedia.org/wiki/8-bit) integer ([`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) with the default generator settings.  The following snippet does the same thing as `cat /dev/urandom | hexdump -v -e '/1 "%u\n"'`:

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

Give that a go with `cargo run` - whee.  There it is.  Random integers 0-255 until you kill the process.  Now delete the whole thing, top to bottom, we're not going to use any of that.  Sorry.  That was a little mean, I know.  We're going to use this crate to introduce randomness later on, don't worry, but first we have to get some fundamentals out of the way if we're gonna get this thing done right.  At least you spent a little less time than I did in this particular rabbit hole, and now we see we don't need no Linux [userland](https://en.wikipedia.org/wiki/User_space) tools.

I promise that's the only [red herring](https://en.wikipedia.org/wiki/Red_herring), the rest of the code you should actually add to your file.

### Mapping Bytes To Notes

*[top](#table-of-contents)*

Let's take a closer look at step 3 of the pipeline.  Of all the steps, that code most closely resembles what we ultimately end up with:

```bash
split("0,2,4,5,7,9,11,12",a,",");
for (i = 0; i < 1; i += 0.0001)
    printf("%08X\n",
           100 * sin(1382 * exp((a[$1 % 8] / 12) * log(2)) * i))
```

This is probably still not too helpful for most - there's [magic numbers](https://en.wikipedia.org/wiki/Magic_number_(programming)) and [sines](https://en.wikipedia.org/wiki/Sine) and [logarithms](https://en.wikipedia.org/wiki/Logarithm) (oh, my) - and its written in freakin' [`AWK`](https://en.wikipedia.org/wiki/AWK).  Don't despair if this still doesn't mean much (or literally anything) to you.  We're going to model this problem from the ground up in [Rust](https://en.wikipedia.org/wiki/Rust_(programming_language)).  For a sneak peek, this is our fully abstracted equivalent:

```rust
TODO
```

We can glean a bit of information at a glance, though, and depending on your current comfort with this domain you may be able to kind of understand the general idea here.  It looks like we're going to tick up floating point values by ten-thousandths from zero to one (`for (i = 0; i < 1; i += 0.0001)`), and do... I don't know, some math and stuff on each value based on the list `[0,2,4,5,7,9,11,12]`: `100 * sin(1382 * exp((a[$1 % 8] / 12) * log(2)) * i))` .  After we do the math, we're going to print it out as an 8-digit hex number: `printf("%08X\n",math())` - this [`printf`](https://en.wikipedia.org/wiki/Printf_format_string) formatter means we want a [0-padded](https://en.wikipedia.org/wiki/Npm_(software)#Notable_breakages) number that's 8 digits long in [upper-case](https://en.wikipedia.org/wiki/Letter_case) [hexadecimal](https://en.wikipedia.org/wiki/Hexadecimal).  The [base 10](https://en.wikipedia.org/wiki/Decimal) integer [`42`](https://en.wikipedia.org/wiki/Phrases_from_The_Hitchhiker%27s_Guide_to_the_Galaxy#Answer_to_the_Ultimate_Question_of_Life,_the_Universe,_and_Everything_(42)) would be printed as `0000002A`.

#### A Little Physics

*[top](#table-of-contents)*

[Sound](https://en.wikipedia.org/wiki/Sound) is composed physically of [vibrations](https://en.wikipedia.org/wiki/Vibration).  These vibrations cause perturbations in some [medium](https://en.wikipedia.org/wiki/Transmission_medium), and those perturbations are what we experience as sound.  When we're talking about [hearing](https://en.wikipedia.org/wiki/Hearing) a sound with our [ears](https://en.wikipedia.org/wiki/Ear), the medium is usually [air](https://en.wikipedia.org/wiki/Atmosphere_of_Earth).

##### Sine Waves

*[top](#table-of-contents)*

Sound propagates as a [wave](https://en.wikipedia.org/wiki/Wave).  In [reality](https://en.wikipedia.org/wiki/Reality) a sound contains many components but for this program we can talk about a super-simplified version that can be represented as a single [sine wave](https://en.wikipedia.org/wiki/Sine_wave):

![sine waves](https://upload.wikimedia.org/wikipedia/commons/6/6d/Sine_waves_different_frequencies.svg)

If you're thinking *but Ben, you CAN mix component frequencies to represent sound waves as sine waves in fact we all do that all the time*, you're [correct in ways I don't personally fully understand](https://en.wikipedia.org/wiki/Signal_processing).  That's really cool stuff and a lot more complicated than what happens in this post.  If that was either turning you {off|on} to this, you can {start|stop} breathing normally.  There will be no signals processed here, just a single frequency [scalar](https://en.wikipedia.org/wiki/Variable_(computer_science)) we modulate.

If the X axis is time, a sine wave represents a recurring action with an analog (or smooth) oscillation.  There are two interesting properties: the amplitude, which measures the deviation from the 0 axis at the peaks (how high the peaks are), and the frequency, which is how close together these peaks are, or how frequently this recurring thing happens.

##### Pitch

*[top](#table-of-contents)*

The standard unit for frequency is the [Hertz](https://en.wikipedia.org/wiki/Hertz), abbreviated `Hz`, which measures the *number of cycles per second*.  One cycle here is the distance (or time) between two peaks on the graph, or the time it takes to go all the way around the circle once:

![cycle gif](https://media.giphy.com/media/F5rQlfTXqCJ8c/giphy.gif)

Recall above that we saw we're going to run a loop like this:  `for (i = 0; i < 1; i += 0.0001)`.  If one were to, say, calculate a bunch of points along a single cycle of a sine wave like this one, it sure seems like this loop could get the job done.

In simple cases, a sound at a specific pitch is a result of that sound's frequency.  The higher the frequency, or closer together the peaks, the higher the pitch.

![frequency](https://upload.wikimedia.org/wikipedia/commons/e/ea/Wave_frequency.gif)

Sound is a continuous spectrum of frequency, but when we make music we tend to prefer [notes](https://en.wikipedia.org/wiki/Musical_note) at set frequencies, or pitches.  I'm using  [frequency](https://en.wikipedia.org/wiki/Fundamental_frequency) and [pitch](https://en.wikipedia.org/wiki/Pitch_(music)) interchangeably, because for this application specifically they are, but go Wiki-diving if you want to learn about the distinction and nuance at play here.  The nature of sound is super cool but super complex and outside of the scope of this post - we just want to hear some numbers sing, we don't need to hear a full orchestra.

One of the super cool things about it is the [octave](https://en.wikipedia.org/wiki/Octave).  Octaves just sound related, you know?

// TODO embed octave sound

It turns out the relationship is physical - to increase any pitch by an octave, you double the frequency.  It turns out, though, that this fixed ratio actually holds for any arbitrary smaller or larger interval as well.  This system is called ["equal temperament"](https://en.wikipedia.org/wiki/Equal_temperament) - every pair of adjacent notes has the same ratio, regardless of how you define "adjacent".  To get halfway to the next octave, you multiply by 1.5 instead of 2.

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

Knowing what frequency to use to produce a given pitch is all well and good, but we need to actually make the sound.

TODO produce the flat tone - I think it's just gonna be 440*i*Pi

#### A Little Music Theory

*[top](#table-of-contents)*

A440 is the A above Middle C on a piano:

![piano](https://upload.wikimedia.org/wikipedia/commons/thumb/2/2e/Piano_Frequencies.svg/2560px-Piano_Frequencies.svg.png)

The cyan key is Middle C, and A440 is highlighted in yellow.  The octaves on an 88-key piano are numbered as shown, so often A440 is simply denoted "A4" especially when dealing with a keyboard specifically.  You may own a tuner that marks 440Hz/A4 specifically if you're a musician.  This pitch is used for calibrating musical instruments and tuning a group, as well as a baseline constant for calculating frequencies.

Note how each octave starts at C, not A, so A4 is actually higher in pitch than C4.  Octaves are "C-indexed" and base 8: `C D E F G A B C`.

##### Scales

*[top](#table-of-contents)*

A [scale](https://en.wikipedia.org/wiki/Scale_(music)) is a series of notes (frequencies) defined in terms of successive intervals from a base note.  The smallest of these intervals on a piano (and most of Western music) is called a [semitone](https://en.wikipedia.org/wiki/Semitone), also called a minor second or half step.  Take a look back at that piano diagram above - one semitone is the distance between an adjacent white key and black key.  A *whole* step, or a [major second](https://en.wikipedia.org/wiki/Major_second), is equal to two of semitones, or two adjacent white keys that pass over a black key.  There's a name for [each interval](https://en.wikipedia.org/wiki/Interval_(music)#Main_intervals) of semitones in an octave:

```rust
#[derive(Debug, Clone, Copy)]
enum Interval {
    Unison,
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

Throughout this post I will tend towards over-specifying types like this - we only need `Min2` (half tone) and `Maj2` (whole tone) for now, but at least this way we have a full toolkit to work with should the need arise.

Clearly, there isn't a black key between every white key.  The piano is designed to play notes from a category of scales called [diatonic scales](https://en.wikipedia.org/wiki/Diatonic_scale), where the full range of an octave consists of five whole steps and two half steps.  We can see this visually on the keyboard - it has the same 8-length whole/half step pattern for the whole length.

A [major scale](https://en.wikipedia.org/wiki/Major_scale) is the baseline scale.  Start at Middle C, the one highlighted in cyan above, and count up to the next C key, eight white keys to the left.  THis is our baseline C-indexed exampled from above, `C D E F G A B C`.  Each time you skip a black key is a whole step and if the two white keys are adjacent it's a half step.  These are the steps you get counting up to the next C, when the pattern repeats.  This totals 12 semitones per octave:

```txt
whole, whole, half, whole, whole, whole, half
  2  +  2   +  1  +   2   +  2  +   2  +  1   =  12  
```

TODO embed sound

There are a few variations of *minor* scale, but for now I'll define the [natural minor scale](https://en.wikipedia.org/wiki/Minor_scale#Natural_minor_scale).  This is what you get if you start at our good old friend A4 and march on up the white keys to A5:

```txt
whole, half, whole, whole, half, whole, whole
```

TODO embed sound

There are the same number of whole and half intervals, they're just distributed differently.  You can play a corresponding minor scale using only the white keys by simply starting at the sixth note.  Try counting it out yourself!

##### Cents

*[top](#table-of-contents)*

Beyond the twelve 12 semitones in an octave, each semitone is divided into 100 [cents](https://en.wikipedia.org/wiki/Cent_(music)).  This means a full octave, representing a 2:1 ratio in frequency, spans 1200 cents.  Go ahead and set up some types:

```rust
struct Cents(f64);
struct Semitones(i8);
```

I didn't just assign aliases as with `type Hertz = f64`, because I need to re-define how to convert to and from these with the [`From`](https://doc.rust-lang.org/std/convert/trait.From.html) [trait](https://doc.rust-lang.org/book/ch10-02-traits.html).  For that, I need my very own type, not just an alias of a primitive that already can convert to and from other primitives with the standard logic.  `Semitones` to `Cents` is not the same thing as `i8` to `f64`, we have a conversion factor.   The [tuple struct](https://doc.rust-lang.org/1.37.0/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types) syntax is perfect for that.  Hertz really is a more general unit of frequency, so it made sense to me to separate that concept from a `Pitch` that can be modulated by cents.

Now, bear with me - we're going to do a little plumbing to let ourselves work at a higher level of abstraction.  We can give ourselves some conversions to the inner primitive:

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

However, now we can convert between them:

```rust
const SEMITONE_CENTS: Cents = Cents(100.0);

impl From<Semitones> for Cents {
    fn from(semitones: Semitones) -> Self {
        Cents(i8::from(semitones) as f64 * f64::from(SEMITONE_CENTS))
    }
}
```

We can also map our `Interval` variants to `Semitones`:

```rust
impl From<Interval> for Semitones {
    fn from(i: Interval) -> Self {
        use Interval::*;
        let x = match i {
            Unison => 0,
            Min2 => 1,
            Maj2 => 2,
            Min3 => 3,
            Maj3 => 4,
            Perfect4 => 5,
            Tritone => 6,
            Perfect5 => 7,
            Min6 => 8,
            Maj6 => 9,
            Min7 => 10,
            Maj7 => 11,
            Octave => 12,
        };
        Semitones(x)
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

Phew!  Lots of code, but now we can operate directly in terms of intervals.

There's one more step to get to frequencies though.  Remember how Middle C was some crazy fraction, 261.626?  This is because cents are a [logarithmic](https://en.wikipedia.org/wiki/Logarithmic_scale) unit, standardized around the point 440.0.  Because of equal temperament, this 2:1 ratio holds for arbitrarily smaller intervals than octaves as well, where the math isn't always so clean.  Doubling this will get 880.0Hz, every time, but how would we add a semitone?  It's 100 cents, nice and neat, and there are 12 semitones - so we'd need to increase by a 12th of what doubling the number would do: `440 * 2^(1/12)`.  Looks innocuous enough, but my calculator gives me 466.164, Rust gives me 466.1637615180899 - not enough to perceptually matter, but enough that it's important that the standard is the interval ratio and not the specific amount of Hertz to add or subtract.  Those amounts will only be precise in floating point decimal representations at exact octaves from the base note, because that's integral factor after multiplying by 1 in either direction, 2 or 1/2.

Otherwise stated, the ratio between frequencies separated by a single cent is the 1200th root of 2, or 2^(1/1200).    In decimal, it's about 1.0005777895.  You wouldn't be able to hear a distinction between two tones a single cent apart.  The [just-noticeable difference](https://en.wikipedia.org/wiki/Just-noticeable_difference) is about 5 or 6 cents, or 5*2^(1/1200).  Using this math, it works out to just shy of 4 cents to cause an increase of 1Hz, more precisely around 3.9302 for a base frequency of 440.0.

Logarithmic units are helpful when the range of the y axis, in our case frequency, increases exponentially.  We know the graph of frequency to pitch does because to jump by any single octave, we double what we have - we're multiplying at each step, not adding (which results in a linear graph).  If A4 is 440Hz, A5 is 880Hz, and by A6 we're already at 1,760Hz.  The graph `f(x) = x^2` looks like this:

![x_squared](https://thepracticaldev.s3.amazonaws.com/i/mkh095mgcasg1soygrb7.png)

A [logarithm](https://en.wikipedia.org/wiki/Logarithm) is the inverse of an [exponent](https://en.wikipedia.org/wiki/Exponentiation).  Our ratio had an exponent that was "1 divided by n", which is the inverse of raising something to the power of "n divided by 1", such as squaring it (n=2).  This is otherwise written as an "nth root", in the case of a cent *n* being 1,200.  This counteracts the rapid growing curve we get by constantly squaring the frequency into a more linear scaled subdivision between octaves:

![cent graph](https://upload.wikimedia.org/wikipedia/commons/thumb/3/3f/Music_intervals_frequency_ratio_equal_tempered_pythagorean_comparison.svg/550px-Music_intervals_frequency_ratio_equal_tempered_pythagorean_comparison.svg.png)

This is a much better way to deal with intervals than by frequency deltas.  Knowing all this we can translate back to the frequency in Hertz of a desired pitch if we know both a base frequency and the number of cents to increase by:

![cents formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/920411bb22d357b13f69a76fa33557c707f7cb57)

Here, *a* is the initial frequency in Hertz, *b* is the target frequency, and *n* is the number of cents by which to increase *a*.

Time for the plumbing.  It looks like we're going to need to divide some `Cents`:

```rust
use std::ops::Div;

impl Div for Cents {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Cents(f64::from(self) / f64::from(rhs))
    }
}
```

We now know enough to manipulate a `Pitch`:

```rust
use std::ops::AddAssign

impl AddAssign<Cents> for Pitch {
    fn add_assign(&mut self, cents: Cents) {
        self.frequency *= 2.0f64.powf((cents / Cents::from(Interval::Octave)).into())
    }
}
```

The [`AddAssign`](https://doc.rust-lang.org/std/ops/trait.AddAssign.html) trait gets us the `+=` operator.  Lets try to increase by a single Hertz using the value above:

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
```

That's a lot easier to work with:

```rust
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

Armed with this knowledge, we can start manipulating pitches in terms of [Scientific Pitch Notation](https://en.wikipedia.org/wiki/Scientific_pitch_notation), another fancy name for a simple concept.  The piano keybaord above was labelled according to this standard, and it's where we get "A4" from.  A standard pitch is composed of three components: a note from A-G, an optional "accidental", and a 0-indexed octave.

```rust
#[derive(Debug, Default, Clone, Copy)]
struct StandardPitch
 {
    accidental: Option<Accidental>,
    note: Note,
    octave: u8,
}
```

The notes are C-indexed, for better or for worse, so `Note::default()` should return that variant.  I did a bare-minimum amount of research and found two [fuzzy](https://ibreathemusic.com/forums/showthread.php?18520-Why-did-C-become-the-middle-note-Why-not-middle-A) [sources](http://ars-nova.com/Theory%20Q&A/Q65.html) that both boil down to "unfortunate historical accident".  The letters were originally numbered from A, of course, but got mapped to frequencies well before the modern modes we use now were honed and refined from previous systems.  We ended up somewhat arbitrarily with this system based around what ended up being C, not A.:

```rust
#[derive(Debug, Clone, Copy)]
enum Note {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Default for Note {
    fn default() -> Self {
        Note::C
    }
}
```

There's optionally a `♭` or `#` modifier which lower or raise the note by one semitone, respectively.  These are called [accidentals](https://en.wikipedia.org/wiki/Accidental_(music)):

```rust
#[derive(Debug, Clone, Copy)]
enum Accidental {
    Flat,
    Sharp,
}
```

There is third accidental called "natural", `♮`, which cancels these out.  To represent a pitch in data we don't need it - that's a string-parsing concern.  The natural symbol is generally used for overriding a [key signature](https://en.wikipedia.org/wiki/Key_signature), which defines the default accidental for all the notes within a scale on [sheet music](https://en.wikipedia.org/wiki/Staff_(music)).  There are a series of accidentals on the margin of the staff that apply to all notes, which is how we ensure we play notes within a single given scale, or [key](https://en.wikipedia.org/wiki/Key_(music)).  However, you may choose to compose a melody that contains a note outside this key.  To cancel it for one written note,  you can write `F♮`.  Our data representation would just store an F in this case, though.

The `Default` implementation that the compiler derives from this code corresponds to the official base pitch of this system, C0.  We can use `StandardPitch::default()` to procure one - here's a [playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=dca4808334d51474c03a993bc1f97c03):

```rust
println!("{:?}", StandardPitch::default()); // StandardPitch { accidental: None, note: C, octave: 0 }
```

It's defined at a set frequency:

```rust
const C_ZERO: Hertz = Hertz(16.352);
```

This is super low - most humans bottom out around 20Hz.  The 88-key piano's lowest note is up at A0, a 9-semitone [`major sixth`](https://en.wikipedia.org/wiki/Major_sixth) higher.  Note how even though this is a different abstraction for working with pitches, the frequencies baked in to the standard are still pinned to the A440 scale.  Do a quick sanity check before abstracting further:

// TODO this should start with StandardPitch::default()

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

Luckily, even being off by a full Hertz at 440 (~4 cents) is less than the just-noticeable difference of ~5-6 cents, so within the ranges we're working with that's not wrong enough to care.

We can get them from strings with `std::str::FromStr`.  We should reduce notation like `E#` to `F` as well - there's no such thing as `E#`, in our diatonic scale `E` and `F` are only separated by a semitone.

```rust

```

Next, we need a way to convert this `StandardPitch` struct to a `Pitch`:

```rust

```

Scales are usually defined as an octave of intervals:

```rust
type Octave = [Interval; 7];
```

Seven hops gets you to eight pitches including the first and last.  Adding a set of intervals to a pitch should increase it by one octave:

```rust

```

// TODO From/Into
// TODO FromStr for Pitch that just calls StandardPitch::FromStr
// TODO Traits for adding different types to pitches
// TODO Scales that return an iterator of StandardPitch notes one octave long - THIS is where I'll talk about iterators - it should take a base note in StandardPitch and a length, return an Octave that's just a `Vec` or something with iter() on it?  and a definite end?
// TODO Go back and compare with the original AWK/one-liner
// TODO Authoring - Pest? - just take separated characters - accept either Unicode flat OR some other character - you can use 'b', because only the first character is matching a note.  For sharp, ASCII 35 '#' is fine to demand.  Add a character for 
// TODO dependent types could verify scale intervals
// TODO after moving all helper code and edit code and stuff, see if this file can be literate?  

##### Diatonic Modes

*[top](#table-of-contents)*

Now we can start defining scales.  If you count up to one octave higher from any given key using just successive white keys, that's a diatonic scale no matter which note you start from.  These scales are called [`Modes`](https://en.wikipedia.org/wiki/Mode_(music)#Modern_modes).

The first scale I laid out, the major scale, is also known as the [`Ionian mode`](https://en.wikipedia.org/wiki/Ionian_mode).  This is the base mode, each other is some offset from this scale.  The natural minor scale, where we started at A4, is called the [`Aeolian mode`].  There's an absurdly fancy name for each offset.  This means we get our first seven `Scale` variants for free:

```rust
#[derive(Debug, Clone, Copy)]
enum Mode {
    Ionian = 0,
    Dorian = 1,
    Phrygian = 2,
    Lydian = 3,
    Mixolydian = 4,
    Aeolian = 5,
    Locrian = 6,
}

#[derive(Debug)]
enum Scale {
    Diatonic(Mode),
}
```

The C mode, Ionian, is the base, so we'll hardcode that sequence:

```txt
whole, whole, half, whole, whole, whole, half
  2  +  2   +  1  +   2   +  2  +   2  +  1
```

```rust
impl Mode {
    fn base_intervals() -> Octave {
        use Interval::*;
        [Maj2, Maj2, Min2, Maj2, Maj2, Maj2, Min2]
    }
}
```

We've seen something like this somewhere before:

```bash
split("0,2,4,5,7,9,11,12",a,",");
```

What if we represent this octave as a series of offsets:

```txt
whole, whole, half, whole, whole, whole, half
  2  +  2   +  1  +   2   +  2  +   2  +  1
0    2     4      5      7       9    11     12
```

Aha!  It's was a major scale over one octave this whole time.

##### Other Scales

*[top](#table-of-contents)*

Okay, Ben.  Ben, okay.  Okay, Ben.  That's the version from the blog post, great.  The line from the meme image has something different:

```bash
split("4,5,7,11",a,",");
```

This is called a [pentatonic scale](https://en.wikipedia.org/wiki/Pentatonic_scale), as it only has five tones per octave defined by four intervals.  The diatonic scales we've been working with are a subset of the [heptatonic scales](https://en.wikipedia.org/wiki/Heptatonic_scale), with seven notes each.  These tones are naturally further apart than we've been using - we're going to need some more intervals - I'm just going to go ahead and toss in the [full set](https://en.wikipedia.org/wiki/Interval_(music)#Main_intervals):

```rust

```

Two identical notes are called a [unison](https://en.wikipedia.org/wiki/Unison), with 0 cents.  These intervals are defined within a single octave, so any of them apply across octaves as well - A4 and A5 are in unison just like A4 and another A4, and C4 and A5 is still a major sixth.  The terms "major", "minor", and "perfect" are not arbitrary, but that discussion is outside the scope of this post.  I will note that the [tritone](https://en.wikipedia.org/wiki/Tritone), representing 3 whole tones or 6 semitones like `F-B`, is the only one that's none of the three.  If you're into the mathy, music theory pattern parts of this exploration, I recommend [harmony](https://en.wikipedia.org/wiki/Harmony) for your next rabbit hole.  The tritone takes a leading role in [dissonance](https://en.wikipedia.org/wiki/Consonance_and_dissonance), and to hear it in action you should check out what the [Locrian mode](https://en.wikipedia.org/wiki/Locrian_mode) we defined sounds like with this program.  The C major scale has a perfect fifth, 5 semitones at the [dominant](https://en.wikipedia.org/wiki/Dominant_(music)) scale [degree](https://en.wikipedia.org/wiki/Degree_(music)) - and the Locrian mode has a tritone which is one extra semitone.

This scale actually corresponds to playing just the black keys on a piano, skipping all the white keys.

Alright.  Back to the bytes.

##### Back To The Bytes

*[top](#table-of-contents)*

### Listen To Your Files

*[top](#table-of-contents)*

You know what else is a stream of bytes?  Literally everything else.  Who needs `bash`!

TODO maybe?  maybe not?  

TODO Rick & Morty "Human Music" gif

### Write Your Own Tunes

*[top](#table-of-contents)*

## Challenges

*[top](#table-of-contents)*

* Port this to your favorite programming language (second favorite if that's already Rust).
* Add more scales.
* Support [Helmholtz pitch notation](https://en.wikipedia.org/wiki/Helmholtz_pitch_notation)
* Support authoring note sequences with variable durations

*Cover image: [reddit](https://www.reddit.com/r/linuxmasterrace/comments/dyqri7/like_god_would_have_wanted/)*