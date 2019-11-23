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

Let's channel that wacky energy.  In this post, we'll throw something [random](https://en.wikipedia.org/wiki/Random_number_generation) into, well, a [math-oven](https://en.wikipedia.org/wiki/Subroutine) and [*viola*](https://en.wikipedia.org/wiki/Viola), [music](https://en.wikipedia.org/wiki/Music)!

In other words, we're going to teach our [computers](https://en.wikipedia.org/wiki/Personal_computer) to ["sing"](https://en.wikipedia.org/wiki/Singing) using [Rust](https://www.rust-lang.org/), backed by a little light [physics](https://en.wikipedia.org/wiki/Physics) and [music theory](https://en.wikipedia.org/wiki/Music_theory).

While the [one-liner](https://en.wikipedia.org/wiki/One-liner_program) in the cover image [procedurally generates](https://en.wikipedia.org/wiki/Procedural_generation) music from random input along a hard-coded single scale, we'll end up with a program that can either procedurally generate music from a large, extensible set of scales or play hand-authored songs created with a rudimentary notation system.

[¡Vámonos!](https://en.wikipedia.org/wiki/Party)

## Table of Contents

- [Preamble](#preamble)
- [The Meme](#the-meme)
- [The Program](#the-program)
  * [Mapping Bytes To Notes](#mapping-bytes-to-notes)
    + [Random Bytes](#random-bytes)
    + [A Little Physics](#a-little-physics)
      - [Sine Waves](#sine-waves)
      - [Pitch](#pitch)
    + [A Little Music Theory](#a-little-music-theory)
      - [Scales](#scales)
      - [Cents](#cents)
      - [Scientific Pitch Notation](#scientific-pitch-notation)
      - [Diatonic Modes](#diatonic-modes)
      - [Other Scales](#other-scales)
    + [Back To The Bytes](#back-to-the-bytes)
  * [Listen To Your Files](#listen-to-your-files)
- [Challenges](#challenges)

## Preamble

I have two disclaimers:

1. [There are](https://en.wikipedia.org/wiki/Existence) [too many](https://en.wikipedia.org/wiki/Saturated_model) [Wikipedia](https://en.wikipedia.org/wiki/Main_Page) [links](https://en.wikipedia.org/wiki/Hyperlink) [here](https://en.wikipedia.org/wiki/Blog).  [If](https://en.wikipedia.org/wiki/Conditional_(computer_programming)) [you're](https://en.wikipedia.org/wiki/You) [that](https://en.wikipedia.org/wiki/Autodidacticism) [kind](https://en.wikipedia.org/wiki/Impulsivity) [of](https://en.wikipedia.org/wiki/Preposition_and_postposition) [person](https://en.wikipedia.org/wiki/Person), [set](https://en.wikipedia.org/wiki/Innovation) [rules](https://en.wikipedia.org/wiki/Law).
1. Further to Point 1, most of this I learned myself on Wikipedia.  The rest is what I remember from [high school](https://en.wikipedia.org/wiki/High_school_(North_America)) as a [band geek](https://en.wikipedia.org/wiki/Euphonium), which was over [ten years](https://en.wikipedia.org/wiki/Decade) [ago](https://en.wikipedia.org/wiki/Past).  I do believe it's generally on the mark, but I am making no claims of authority.  If you see something, [say something](hhttps://en.wikipedia.org/wiki/Allen_Kay#Advertisements).


This is (hopefully) a [beginner](https://en.wikipedia.org/wiki/Novice)-level post.  It's not necessarily specific to Rust but also not shy about Rust idioms or added [verbosity](https://en.wikipedia.org/wiki/Verbosity).  Even so, or perhaps because of, it should be pretty readable even if you don't speak Rust - that's the whole point!  I promise I'll (mostly) stop the whole parenthesis thing, too.

## The Meme

This post was inspired by this meme:

![the meme](https://i.redd.it/uirqnamnjpz31.jpg)

Here's a slightly modified version of the [`bash`](https://en.wikipedia.org/wiki/Bash_(Unix_shell)) one-liner at the bottom, taken from [this blog post](https://blog.robertelder.org/bash-one-liner-compose-music/) by [Robert Elder](https://www.robertelder.org/) that explores it:

```bash
cat /dev/urandom | hexdump -v -e '/1 "%u\n"' | awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }' | xxd -r -p | aplay -c 2 -f S32_LE -r 16000
```

The linked blogpost is considerably more brief and assumes a greater degree of background knowledge than this one, but that's not to discredit it at all.  That write-up and Wikipedia were all I needed to complete this translation with absolutely not a clue how this whole thing worked going in.  If you'd like the challenge of implementing this yourself blind, _stop right here_. Read just that post and try to build this yourself in the language of your choice.  Come back here when you get stuck.  This should apply to whatever you've got going on by then unless you've gone real funky with it.  This post does extend the functionality of the one-liner (you'd hope, at XXX lines), so, you know, maybe still come back anyway.

Here's a step-by-step video demonstration of that [pipeline](https://en.wikipedia.org/wiki/Pipeline_%28Unix%29) in sequence:

{% youtube uLhQQSKhTok %}

I've gotta be honest - I didn't even try it myself.  I'm not going to do what that [code](https://en.wikipedia.org/wiki/Source_code) does in this post, and I'm not going to elaborate on what any of these specific steps of the pipeline mean.  We're immediately diving into a pure Rust solution.  Nevertheless, it serves as a solid [roadmap](https://en.wikipedia.org/wiki/Plan).  Each command of this pipeline calls out to some other tool present on a standard [desktop](https://en.wikipedia.org/wiki/Desktop_computer) [Linux](https://en.wikipedia.org/wiki/Linux) [distribution](https://en.wikipedia.org/wiki/Linux_distribution) like [Ubuntu](https://en.wikipedia.org/wiki/Ubuntu) to perform a series of operations on some continuous incoming data:

1. `cat /dev/urandom`: Get a stream of random binary data.
1. `hexdump -v -e '/1 "%u\n"'`: Convert binary to 8-bit base-10 integers (0-255).
1. `awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }'`: Map integers to pitches, as 8-byte hexadecimal values.
1. `xxd -r -p`: Convert hex numbers back to binary.
1. `aplay -c 2 -f S32_LE -r 16000`: Play back binary data as sound.

Of this, only step 3 ends up being pretty much what happens here too - here's what it looks like spread apart:

```bash
split("0,2,4,5,7,9,11,12",a,",");
for (i = 0; i < 1; i += 0.0001)
    printf("%08X\n",
           100 * sin(1382 * exp((a[$1 % 8] / 12) * log(2)) * i))
```

This is probably still not too helpful for most - there's [magic numbers](https://en.wikipedia.org/wiki/Magic_number_(programming)) and [sines](https://en.wikipedia.org/wiki/Sine) and [logarithms](https://en.wikipedia.org/wiki/Logarithm) (oh, my) - and its written in freakin' [`AWK`](https://en.wikipedia.org/wiki/AWK).  Don't despair if this still doesn't mean much (or literally anything) to you.  We're going to model this problem from the ground up in [Rust](https://en.wikipedia.org/wiki/Rust_(programming_language)).  As a result, this logic will become crystal clear, and we'll be able to extend a lot further with minimal effort.

We can glean a bit of information at a glance, though, and depending on your current comfort with this domain you may be able to kind of understand the general idea here.  It looks like we're going to tick up floating point values by ten-thousandths from zero to one (`for (i = 0; i < 1; i += 0.0001)`), and do... I don't know, some math and stuff on each value based on the list `[0,2,4,5,7,9,11,12]`: `100 * sin(1382 * exp((a[$1 % 8] / 12) * log(2)) * i))` .  After we do the math, we're going to print it out as an 8-digit hex number: `printf("%08X\n",math())` - this [`printf`](https://en.wikipedia.org/wiki/Printf_format_string) formatter means we want a [0-padded](https://en.wikipedia.org/wiki/Npm_(software)#Notable_breakages) number that's 8 digits long in [upper-case](https://en.wikipedia.org/wiki/Letter_case) [hexadecimal](https://en.wikipedia.org/wiki/Hexadecimal).  The [base 10](https://en.wikipedia.org/wiki/Decimal) integer [`42`](https://en.wikipedia.org/wiki/Phrases_from_The_Hitchhiker%27s_Guide_to_the_Galaxy#Answer_to_the_Ultimate_Question_of_Life,_the_Universe,_and_Everything_(42)) would be printed as `0000002A`.

¡Vámonos!

## The Program

As always, ensure you have at least the default stable Rust toolchain [installed](https://www.rust-lang.org/tools/install).  This code was written with `rustc` [version 1.39](https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html) for [Rust 2018](https://doc.rust-lang.org/nightly/edition-guide/rust-2018/edition-changes.html).  

Then, spin up a new project:

```txt
$ cargo new music
```

Open that directory in the environment of your choice.  We'll use three crates:

* [`rand`](https://docs.rs/rand/0.7.2/rand/) - [RNG](https://en.wikipedia.org/wiki/Random_number_generation)
* [`hound`](https://github.com/ruuda/hound) - [WAV](https://en.wikipedia.org/wiki/WAV)
* [`rodio`](https://docs.rs/rodio/0.10.0/rodio/) - [OUT](https://en.wikipedia.org/wiki/Audio_signal)

We'll use `rand` in place of [`cat /dev/urandom`](https://en.wikipedia.org/wiki//dev/random), and `hound`/`rodio` will cover [`aplay`](https://linux.die.net/man/1/aplay).  In `Cargo.toml`:

```toml
  [dependencies]

+ hound = "3.4"
+ rand = "0.7"
+ rodio = "0.10"
```



### Mapping Bytes To Notes

#### Random Bytes

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

Give that a go with `cargo run` - whee.  There it is.  Random integers 0-255 until you kill the process.  Now delete the whole thing, top to bottom, we're not going to use any of that.  We're going to introduce randomness later on, don't worry, but first we have to get some fundamentals out of the way if we're gonna get this thing done right.  I'm only a little sorry, but hey - at least you spent less time in this particular rabbit hole.  I promise that's the last [red herring](https://en.wikipedia.org/wiki/Red_herring), the rest of the code you should actually add to your file.

#### A Little Physics

[Sound](https://en.wikipedia.org/wiki/Sound) is composed physically of [vibrations](https://en.wikipedia.org/wiki/Vibration).  These vibrations cause perturbations in some [medium](https://en.wikipedia.org/wiki/Transmission_medium), and those perturbations are what we experience as sound.  When we're talking about [hearing](https://en.wikipedia.org/wiki/Hearing) a sound with our [ears](https://en.wikipedia.org/wiki/Ear), the medium is usually [air](https://en.wikipedia.org/wiki/Atmosphere_of_Earth).

##### Sine Waves

Sound propagates as a [wave](https://en.wikipedia.org/wiki/Wave).  In [reality](https://en.wikipedia.org/wiki/Reality) a sound contains many components but for this program we can talk about a super-simplified version that can be represented as a single [sine wave](https://en.wikipedia.org/wiki/Sine_wave):

![sine waves](https://upload.wikimedia.org/wikipedia/commons/6/6d/Sine_waves_different_frequencies.svg)

If you're thinking *but Ben, you CAN mix component frequencies to represent sound waves as sine waves in fact we all do that all the time*, you're [correct in ways I don't personally fully understand](https://en.wikipedia.org/wiki/Signal_processing).  That's really cool stuff and a lot more complicated than what happens in this post.  If that was either turning you {off|on} to this, you can {start|stop} breathing normally.  There will be no signals processed here, just a single frequency [scalar](https://en.wikipedia.org/wiki/Variable_(computer_science)) we modulate.

If the X axis is time, a sine wave represents a recurring action with an analog (or smooth) oscillation.  There are two interesting properties: the amplitude, which measures the deviation from the 0 axis at the peaks (how high the peaks are), and the frequency, which is how close together these peaks are, or how frequently this recurring thing happens.

##### Pitch

The standard unit for frequency is the [Hertz](https://en.wikipedia.org/wiki/Hertz), abbreviated `Hz`, which measures the *number of cycles per second*.  One cycle here is the distance (or time) between two peaks on the graph, or the time it takes to go all the way around the circle once:

![cycle gif](https://media.giphy.com/media/F5rQlfTXqCJ8c/giphy.gif)

In simple cases, a sound at a specific pitch is a result of that sound's frequency.  The higher the frequency, or closer together the peaks, the higher the pitch.

![frequency](https://upload.wikimedia.org/wikipedia/commons/e/ea/Wave_frequency.gif)

Sound is a continuous spectrum of frequency, but when we make music we tend to prefer [notes](https://en.wikipedia.org/wiki/Musical_note) at set frequencies, or pitches.  I'm using  [frequency](https://en.wikipedia.org/wiki/Fundamental_frequency) and [pitch](https://en.wikipedia.org/wiki/Pitch_(music)) interchangeably, because for this application specifically they are, but go Wiki-diving if you want to learn about the distinction and nuance at play here.  The nature of sound is super cool but super complex and outside of the scope of this post - we just want to hear some numbers sing, we don't need to hear a full orchestra.

One of the super cool things about it is the [octave](https://en.wikipedia.org/wiki/Octave).  Octaves just sound related, you know?  It turns out the relationship is physical - to increase any pitch by an octave, you double the frequency.

To start working with something concrete, we need some sort of standard.   Some of the world has settled on [440Hz](https://en.wikipedia.org/wiki/A440_(pitch_standard)) - it's [ISO](https://en.wikipedia.org/wiki/International_Organization_for_Standardization) [16](https://www.iso.org/standard/3601.html), at least.  It's also apparently called "The Stuttgart Pitch", which is funny.

![stuttgart](https://i.imgflip.com/3h0y3g.jpg)

Let's set up a type to represent a pitch:

```rust
type Hertz = f64;
const STANDARD_PITCH: Hertz = 440.0;

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

Let's see if we can produce this tone.

TODO produce the flat tone - I think it's just gonna be 440*i*Pi

#### A Little Music Theory

A440 is the A above Middle C on a piano:

![piano](https://upload.wikimedia.org/wikipedia/commons/thumb/2/2e/Piano_Frequencies.svg/2560px-Piano_Frequencies.svg.png)

The cyan key is Middle C, and A440 is highlighted in yellow.  The octaves on an 88-key piano are numbered as shown, so often A440 is simply denoted "A4" especially when dealing with a keyboard specifically.  You may own a tuner that marks 440Hz/A4 specifically if you're a musician.  This pitch is used for calibrating musical instruments and tuning a group, as well as a baseline constant for calculating frequencies.

Note how each octave starts at C, not A, so A4 is actually higher in pitch than C4.  Octaves are "C-indexed" and base 8: `C D E F G A B C`.

##### Scales

A [scale](https://en.wikipedia.org/wiki/Scale_(music)) is a series of notes (frequencies) defined in terms of successive intervals from a base note.  The smallest of these intervals on a piano (and most of Western music) is called a [semitone](https://en.wikipedia.org/wiki/Semitone), also called a minor second or half step.  Take a look back at that piano diagram above - one semitone is the distance between an adjacent white key and black key.  A *whole* step, or a [major second](https://en.wikipedia.org/wiki/Major_second), is equal to two of semitones, or two adjacent white keys that pass over a black key.  For now these are the only intervals we'll need:

```rust
#[derive(Debug, Clone, Copy)]
enum Interval {
    Min2,
    Maj2,
}

impl Interval {
    fn semitones(&self) -> Semitones {
        use Interval::*;
        match self {
            Min2 => 1,
            Maj2 => 2,
        }
    }
}
```

Each variant of this [`enum`](https://en.wikipedia.org/wiki/Tagged_union#2010s) also carries the number of semitones it represents.

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

Beyond the twelve 12 semitones in an octave, each semitone is divided into 100 [cents](https://en.wikipedia.org/wiki/Cent_(music)).  This means a full octave, representing a 2:1 ratio in frequency, spans 1200 cents.  Go ahead and set up some constants:

```rust
type Cents = f64;
type Semitones = u8;
const SEMITONE_CENTS: Cents = 100.0;
const OCTAVE_SEMITONES: Semitones = 12;
const OCTAVE_CENTS: Cents = SEMITONE_CENTS * OCTAVE_SEMITONES as f64;
```

Remember how Middle C was some crazy fraction, 261.626?  This is because cents are a [logarithmic](https://en.wikipedia.org/wiki/Logarithmic_scale) unit, standardized around the point 440.0.  Because of equal temperament, this 2:1 ratio holds for arbitrarily smaller intervals than octaves as well, where the math isn't always so clean.  Doubling this will get 880.0Hz, every time, but how would we add a semitone?  It's 100 cents, nice and neat, and there are 12 semitones - so we'd need to increase by a 12th of what doubling the number would do: `440 * 2^(1/12)`.  Looks innocuous enough, but my calculator gives me 466.164, Rust gives me 466.1637615180899 - not enough to perceptually matter, but enough that it's important that the standard is the interval ratio and not the specific amount of Hertz to add or subtract.  Those amounts will only be precise in floating point decimal representations at exact octaves from the base note, because that's integral factor after multiplying by 1 in either direction, 2 or 1/2.

Otherwise stated, the ratio between frequencies separated by a single cent is the 1200th root of 2, or 2^(1/1200).    In decimal, it's about 1.0005777895.  You wouldn't be able to hear a distinction between two tones a single cent apart.  The [just-noticeable difference](https://en.wikipedia.org/wiki/Just-noticeable_difference) is about 5 or 6 cents, or 5*2^(1/1200).  Using this math, it works out to just shy of 4 cents to cause an increase of 1Hz, more precisely around 3.9302 for a base frequency of 440.0.

Logarithmic units are helpful when the range of the y axis, in our case frequency, increases exponentially.  We know the graph of frequency to pitch does because to jump by any single octave, we double what we have - we're multiplying at each step, not adding (which results in a linear graph).  If A4 is 440Hz, A5 is 880Hz, and by A6 we're already at 1,760Hz.  The graph `f(x) = x^2` looks like this:

![x_squared](https://thepracticaldev.s3.amazonaws.com/i/mkh095mgcasg1soygrb7.png)

A [logarithm](https://en.wikipedia.org/wiki/Logarithm) is the inverse of an [exponent](https://en.wikipedia.org/wiki/Exponentiation).  Our ratio had an exponent that was "1 divided by n", which is the inverse of raising something to the power of "n divided by 1", such as squaring it (n=2).  This is otherwise written as an "nth root", in the case of a cent *n* being 1,200.  This counteracts the rapid growing curve we get by constantly squaring the frequency into a more linear scaled subdivision between octaves:

![cent graph](https://upload.wikimedia.org/wikipedia/commons/thumb/3/3f/Music_intervals_frequency_ratio_equal_tempered_pythagorean_comparison.svg/550px-Music_intervals_frequency_ratio_equal_tempered_pythagorean_comparison.svg.png)

This is a much better way to deal with intervals than by frequency deltas.  Knowing all this we can translate back to the frequency in Hertz of a desired pitch if we know both a base frequency and the number of cents to increase by:

![cents formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/920411bb22d357b13f69a76fa33557c707f7cb57)

Here, *a* is the initial frequency in Hertz, *b* is the target frequency, and *n* is the number of cents by which to increase *a*.

We can add a method to `Pitch` with this logic:

```diff
  impl Pitch {
      fn new(frequency: Hertz) -> Self {
          Self { frequency }
      }
+     fn add_cents(&mut self, cents: Cents) {
+         self.frequency *= 2.0f64.powf(cents / OCTAVE_CENTS);
+     }
  }
```

Lets try to increase by a single Hertz using the value above:


```rust
fn main() {
    let mut pitch = Pitch::default();
    println!("{:?}", pitch); // Pitch { frequency: 440.0 }
    pitch.add_cents(3.9302); // attempt to add one Hz
    println!("{:?}", pitch); // Pitch { frequency: 441.0000105867894 } - close enough
}
```

Instead of adding single cents at a time, add a helper method that just expects a number of semitones:

```diff
  impl Pitch {
      fn new(frequency: Hertz) -> Self {
          Self { frequency }
      }
      fn add_cents(&mut self, cents: Cents) {
          self.frequency *= 2.0f64.powf(cents / OCTAVE_CENTS);
      }
+     fn add_semitones(&mut self, semitones: i32) {
+         self.add_cents(semitones as f64 * SEMITONE_CENTS);
+     }
  }
```

That's a lot easier to work with:

```rust
fn main() {
    let mut pitch = Pitch::default();
    println!("{:?}", pitch); // Pitch { frequency: 440.0 }
    pitch.add_semitones(OCTAVE_SEMITONES); // add an octave
    println!("{:?}", pitch); // Pitch { frequency: 880.0 } - 2:1 ratio
}
```

##### Scientific Pitch Notation

Armed with this knowledge, we can start manipulating pitches in terms of [Scientific Pitch Notation](https://en.wikipedia.org/wiki/Scientific_pitch_notation), another fancy name for a simple concept.  The piano keybaord above was labelled according to this standard, and it's where we get "A4" from.  A note is composed of three components.  There the note:

```rust
#[derive(Debug)]
enum Note {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}
```

There's optionally one of three modifiers, called `accidentals`:

```rust
#[derive(Debug)]
enum Accidental {
    Flat,
    Natural,
    Sharp,
}
```

A given pitch has one of each:

```rust
#[derive(Debug)]
struct SPN {
    accidental: Accidental,
    note: Note,
    octave: u8,
}
```

The base of this system is defined as C0, with a set frequency:

```rust
const C_ZERO: Hertz = 16.352;

impl Default for SPN {
    fn default() -> Self {
        Self {
            accidental: Accidental::Natural,
            note: Note::C,
            octave: u8::default(),
        }
    }
}
```

This is super low - most humans bottom out around 20Hz.  The 88-key piano doesn't even start until A0Note how even though this is a different abstraction for working with pitches, the frequencies baked in to the standard are still pinned to the A440 scale.  Do a quick sanity check before abstracting further:

```rust
fn main() {
    let mut pitch = Pitch::new(C_ZERO);
    println!("{:?}", pitch); // Pitch { frequency: 16.352 }
    pitch.add_semitones(OCTAVE_SEMITONES * 4); // add 4 octaves - C0 -> C4
    println!("{:?}", pitch); // Pitch { frequency: 261.632 }
    pitch.add_semitones(9); // C4 -> A4
    println!("{:?}", pitch); // Pitch { frequency: 440.010821831319 }
}
```

Oof.  Damn floating point numbers.  Luckily, even being off by a full Hertz at 440 (~4 cents) is less than the just-noticeable difference of ~5-6 cents, so within the ranges we're working with, that's not wrong enough to care.

We can get them from strings with `std::str::FromStr`:

```rust
impl FromStr for PianoKey {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use io::{Error, ErrorKind::*};
        // Verify it's a 2-character string
        if s.len() != 2 {
            return Err(Error::new(InvalidInput, "Must be two characters long"));
        }
        let mut chars = s.chars();
        // Grab the note value - first character of string
        if let Some(note) = chars.next() {
            let char_note = note as char;
            if !char_note.is_uppercase() {
                // Reject anything outside [A-Z]
                Err(Error::new(InvalidData, "First character must be an uppercase letter"))
            } else if let Some(octave) = chars.next() {
                // Grabbed octave value - second and final character of string
                // Turn octave to integer
                let integer_octave = octave as u8 - b'0';
                if integer_octave > 8 {
                    // Reject anything outside [0-8]
                    return Err(Error::new(InvalidData, "Second character must be 0-8"));
                }
                // Turn note to integer
                let integer_note = char_note as u8 - b'A';
                // Make sure its a real note
                if integer_note <= 8 {
                    // Success!!
                    Ok(PianoKey::new(integer_note, integer_octave))
                } else {
                    Err(Error::new(InvalidData, "First character must be A-G"))
                }
            } else {
                Err(Error::new(InvalidInput, "Must be two characters long"))
            }
        } else {
            Err(Error::new(NotFound, "Input cannot be empty"))
        }
    }
}
```

This has some error checking to make sure we get a valid key, doesn't handle the special cases where `0` and `8` are not actually full octaves - for this demonstration I'm sticking to the middle of the keyboard where it doesn't matter, but you'll want to address that correctly if you build out form this example!  Refer to the diagram for specifics. The [`regex`](https://docs.rs/regex/1.3.1/regex/) crate for representing [regular expressions](https://en.wikipedia.org/wiki/Regular_expression) might come in handy.

Next, we need a way to convert a `PianoKey` to a `Pitch`:

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
// TODO FromStr for Pitch that just calls SPN::FromStr
// TODO Traits for adding different types to pitches
// TODO Scales that return an iterator of SPN notes one octave long - THIS is where I'll talk about iterators - it should take a base note in SPN and a length, return an Octave that's just a `Vec` or something with iter() on it?  and a definite end?
// TODO Go back and compare with the original AWK/one-liner
// TODO Authoring - Pest? - just take separated characters - accept either Unicode flat OR some other character - you can use 'b', because only the first character is matching a note.  For sharp, ASCII 35 '#' is fine to demand.  Add a character for 

##### Diatonic Modes

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

##### Other Scales

Okay, Ben.  Ben, okay.  Okay, Ben.  But what about the pentatonic scale:

```txt

```

This corresponds to playing just the black keys on a piano, starting from 

Alright.  Back to the bytes.

##### Back To The Bytes

### Listen To Your Files

You know what else is a stream of bytes?  Literally everything else.  Who needs `bash`!

TODO maybe?  maybe not?  

TODO Rick & Morty "Human Music" gif

## Challenges

* Port this to your favorite programming language (second favorite if that's already Rust).
* Add more scales.
* Parse sequences of keys to author music.
* Support [Helmholtz pitch notation](https://en.wikipedia.org/wiki/Helmholtz_pitch_notation)

*Body images are wikimedia commons unless otherwise specified*
*Cover image from some subreddit, I don't remember*