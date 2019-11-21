---
title: Teaching Numbers To Sing
published: false
description: Learn how to generate sound from numeric data in Rust.
tags: beginners, rust, tutorial, music
---

# Everything Is Music

> Everything is music. When I go home, I throw knickers in the oven and it's music. Crash, boom, bang!

*- [Winona Ryder](https://en.wikipedia.org/wiki/Winona_Ryder) as [Björk](https://en.wikipedia.org/wiki/Bj%C3%B6rk) on [SNL's Celebrity Jeopardy!](https://youtu.be/R3V94ZtmdbQ?t=190) - 2002*

Let's channel that wacky energy.  In this post, we'll throw something random into, well, a [math-oven](https://en.wikipedia.org/wiki/Subroutine) and *viola*, music!  We're going to teach our [computers](https://en.wikipedia.org/wiki/Personal_computer) to ["sing"](https://en.wikipedia.org/wiki/Singing) using [Rust](https://www.rust-lang.org/), along with a juuust a little light [physics](https://en.wikipedia.org/wiki/Physics) and [music theory](https://en.wikipedia.org/wiki/Music_theory).  ¡Vámonos!

## Table of Contents

- [Preamble](#preamble)
- [The Meme](#the-meme)
- [The Program](#the-program)
  * [Random input data](#random-input-data)
    + [`Iterator`](#iterator)
  * [Mapping Bytes To Notes](#mapping-bytes-to-notes)
    + [A Little Physics](#a-little-physics)
      - [Sine Waves](#sine-waves)
      - [Notes](#notes)
    + [A Little Music Theory](#a-little-music-theory)
      - [Scales](#scales)
      - [Cents](#cents)
      - [Modes](#modes)
      - [Other Scales](#other-scales)
  * [Listen To Any Arbitrary File](#listen-to-any-arbitrary-file)
- [Challenges](#challenges)

## Preamble

I have two disclaimers:

1. There are a lot of [Wikipedia](https://en.wikipedia.org/wiki/Main_Page) links here.  If you're *that* kind of [person](https://en.wikipedia.org/wiki/Person), there's a potential for problematic [rabbit](https://en.wikipedia.org/wiki/Rabbit) holes.  Set [rules](https://en.wikipedia.org/wiki/Law).  Most of them will be on topic from here on, though.
1. Further to Point 1, most of this I learned *myself* on Wikipedia.  The rest was [high school](https://en.wikipedia.org/wiki/High_school_(North_America)), which was like ten years ago.  I do believe it's generally on the mark, but I am making *no* claims of authority.  If you see something, say something.

This is (hopefully) a beginner-level post.  It's not necessarily specific to Rust but also not shy about Rust idioms.  Even so, or perhaps because of, it should be pretty readable even if you don't speak Rust (yet) - that's the whole point!  I promise I'll (mostly) stop the whole parenthesis thing, too.

## The Meme

This post was inpsired by this meme:

![the meme](https://i.redd.it/uirqnamnjpz31.jpg)

Here's a slightly modified version of the `bash` one-liner at the bottom, taken from [this blog post](https://blog.robertelder.org/bash-one-liner-compose-music/) that explores it:

```bash
cat /dev/urandom | hexdump -v -e '/1 "%u\n"' | awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }' | xxd -r -p | aplay -c 2 -f S32_LE -r 16000
```

No, just mashing your keyboard will (likely) not yield similar results.  I tried myself so you don't have to.  Here's a step-by-step video demonstration:

{% youtube uLhQQSKhTok %}

We're not going to do what that code does exactly, and I'm not going to elaborate on what any of these specific snippets mean, but it does serve as a solid roadmap.  Each like calls out to some other GNU/Linux/(ALSA) tool:

1. `cat /dev/urandom`: Get a stream of random binary data.
1. `hexdump -v -e '/1 "%u\n"'`: Convert binary to 8-bit base-10 integers (0-255).
1. `awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }'`: Map integers to pitches, as 8-byte hexadecimal values.
1. `xxd -r -p`: Convert hex numbers back to binary.
1. `aplay -c 2 -f S32_LE -r 16000`: Play back binary data as sound.

Of this, only step three ends up being pretty much what happens here too - here's what it looks like as spread apart as I could:

```bash
split("0,2,4,5,7,9,11,12",a,",");
for (i = 0; i < 1; i+= 0.0001)
    printf("%08X\n",
           100 * sin(1382 * exp((a[$1 % 8]/12) * log(2)) * i))
```

Don't worry, we're gonna Rust up the joint and it'll all be clear.  We'll actually be able to make it even cooler with minimal effort thanks to aforementional total Rusting.  ¡Vámonos!

## The Program

As always, ensure you have at least the default stable Rust toolchain [installed](https://www.rust-lang.org/tools/install).  This code was written with `rustc` [version 1.39](https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html) for [Rust 2018](https://doc.rust-lang.org/nightly/edition-guide/rust-2018/edition-changes.html), and uses only the [`rand`](https://docs.rs/rand/0.7.2/rand/) crate.

Then, spin up a new project:

```txt
$ cargo new music
```

Open that directory in the environment of your choice.

### Random input data

First off, we need to grab the Rust crate used for generating random numbers.  Add the `rand` dependency to `Cargo.toml`:

```diff
  [dependencies]

+ rand = "0.7"
```

This crate is quite featureful, but we're keeping it simple.  Add an import to the top of `src/main.rs`:

```rust
use rand::random;
```

#### Iterator

We can skip the conversion from binary.   This crate can give us random 8-bit integers out of the box.  We can implement a similar result to the first two steps, or `cat /dev/urandom | hexdump -v -e '/1 "%u\n"'` by manually implementing an [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html):

```rust
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
```

The struct itself doesn't need to store any state.  We just always want to produce the next value by calling `rand::random()`, specified with the associated type of this iterator.  I set `Item` to `u8`, so calling `Random::Input::next()` will always return a `random::<u8>()` - there's no `None` branch, just `Some(x)`.  That means `unwrap()` is always safe to call on this iterator, it won't panic.  You can take it for a spin with this driver code:

```rust
fn main() {
    let mut rands = RandomBytes::new();
    loop {
        println!("{}", rands.next().unwrap());
    }
}
```

This should print an endless loop of random numbers between 0 and 255 inclusive until you kill the process.

### Mapping Bytes To Notes

This is the meat of the program - turning our numeric data into something we can hear.  To get from random numbers to sounds we can hear, we need to map each data point to an amplitude.  The relevant section of the `bash` again:

```bash
awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }'
```

Tools like `awk` are terse, but this is merely a `for` loop with some math in the body.

#### A Little Physics

Before diving in - I'm using things like [*frequency*](https://en.wikipedia.org/wiki/Fundamental_frequency) and [*pitch*](https://en.wikipedia.org/wiki/Pitch_(music)) interchangeably, because for this application specifically they are, and this is a huge oversimplification of what makes up sound.

[Sound](https://en.wikipedia.org/wiki/Sound) is composed physically of [vibrations](https://en.wikipedia.org/wiki/Vibration).  These vibrations cause perturbances in some [medium](https://en.wikipedia.org/wiki/Transmission_medium), and those perturbations are what we experience as sound.  When we're talking about hearing a sound with our ears, the medium is usually air.

##### Sine Waves

Sound propogates as a [wave](https://en.wikipedia.org/wiki/Wave).  In [reality](https://en.wikipedia.org/wiki/Reality) a sound contains many components but we can talk about a simplified version that can be represented as a single [*sine wave*](https://en.wikipedia.org/wiki/Sine_wave):

![sine waves](https://upload.wikimedia.org/wikipedia/commons/6/6d/Sine_waves_different_frequencies.svg)

*image: [wikimedia commons](https://en.wikipedia.org/wiki/File:Sine_waves_different_frequencies.svg)*

If you're thinking *but Ben, you CAN mix component frequencies to represent sound waves as sine waves we all do that all the time*, you're correct (and probably smarter than me).  This is much simpler math-wise.  If that was either turning you on or off to this post, you can {start/stop} breathing.  No signal processing here, just a single frequency we modulate.

If the X axis is time, a sine wave represents a recurring action with an analog (or smooth) oscillation between their maximal *amplitudes*, or distances in either direction from 0.  The *frequency* is how close together these peaks are, or how frequently this thing occurs.

It makes sense that a vibration propogating through a medium could be represented as such a wave - picture a string vibrating on a guitar.  It wobbles back and forth rapidly, just like this wave's shape.  It stands to reason that the waves generated from this action would correspond to this osciallation at a given point.

In simple cases, a sound at a specific pitch is a result of that sound's frequency.  The higher the frequency, or closer together the peaks, the higher the pitch.

The standard unit for frequency is the [Hertz](https://en.wikipedia.org/wiki/Hertz), abbreviated `Hz`, which measures the *number of cycles per second*.  One cycle here is the distance (or time) between two peaks on the graph:

![cycle gif](https://media.giphy.com/media/F5rQlfTXqCJ8c/giphy.gif)

##### Notes

Sound is a continuous spectrum of frequency, but when we make music we tend to prefer [*notes*](https://en.wikipedia.org/wiki/Musical_note) at set frequencies.  To start, though, we need some sort of standard, and some of the world has settled on [440Hz](https://en.m.wikipedia.org/wiki/A440_(pitch_standard)) - it's [ISO 16](https://www.iso.org/standard/3601.html), at least.  It's also apparently called "The Stuttgart Pitch", which is funny.

![stuttgart](https://i.imgflip.com/3h0y3g.jpg)

*image: I made this on [imgflip.com](https://imgflip.com/) but have no proof of that*

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

With this code we can use `Pitch::default()` to get our A440 pitch, or pass an abitrary frequency: `Pitch::new(440.0)`.

Let's see if we can produce this tone.

TODO produce the flat tone - I think it's just gonna be 440*i*Pi

#### A Little Music Theory

A440 is the A above Middle C on a piano:

![piano](https://upload.wikimedia.org/wikipedia/commons/thumb/2/2e/Piano_Frequencies.svg/2560px-Piano_Frequencies.svg.png)

*image: [wikimedia commons](https://commons.wikimedia.org/wiki/File:Piano_Frequencies.svg)*

The cyan key is Middle C, and A440 is highlighted in yellow.

If you're a musician you may own a tuner that marks 440Hz specifically.  This pitch is used for calibrating musical instruments and tuning a group, and we'll use it as a baseline constant for calculating frequencies.

##### Scales

A [scale](https://en.wikipedia.org/wiki/Scale_(music)) is a series of notes (frequencies) defined in terms of successive intervals from a base note.  The smallest of these intervals is called a [semitone](https://en.wikipedia.org/wiki/Semitone), also called a minor second.  Here I'll refer to it as a "half" step.  Take a look back at that piano diagram above - one semitone is the distance between an adjacacent white key and black key.  A *whole* step, or a [major second](https://en.wikipedia.org/wiki/Major_second), is equal to two of these, or two adjacant white keys skipping a black.

Clearly, though, there isn't a black key between every white key.  The piano is designed to play notes from a catagory called [diatonic scales](https://en.wikipedia.org/wiki/Diatonic_scale), where the full range of an octave consists of five whole steps and two half steps.  We can see this visually on the keyboard - an octave is 8 notes, and between any two keys that are eight apart there will be the same number of whole and half steps.

A major scale, also known as [Ionian mode](https://en.m.wikipedia.org/wiki/Mode_(music)), is the baseline scale.  Start at Middle C, the one highlight in cyan above, and count up to the next C key, eight white keys to the left.  Each time you skip a black key is a whole step and if the two white keys are adjacent it's a half step.  These are the steps you get:

```txt
whole, whole, half, whole, whole, whole, half
```

TODO embed sound

There are a few variations of *minor* scale, but for now I'll define the [natural minor scale](https://en.m.wikipedia.org/wiki/Minor_scale#Natural_minor_scale) (a.k.a. Aeolian mode):

```txt
whole, half, whole, whole, half, whole, whole
```

TODO embed sound

There are the same number of whole and half intervals, they're just distributed differently.  You can play a corresponding minor scale using only the white keys by simply starting at the sixth note.  Try counting it out yourself!

##### Cents

To calculate the value needed in Hertz, we need a more precise way to describe an interval.  A full octave has a frequency ratio of 2:1, meaning a note one octave higher has double the frequency of the lower.  This results in an exponential curve if you were to graph frequencies as they grow.  When working with such a curve there's often a corresponding logarithmic unit that turns that curve into a line.  For musical intervals this unit called a [cent](https://en.wikipedia.org/wiki/Cent_(music)) to represent the ratio between two frequencies.  We've already seen how each octave is divided into 12 semitones:

```txt
whole, whole, half, whole, whole, whole, half
  2  +  2   +  1  +   2   +  2  +   2  +  1   =  12  
```

Each semitone is defined as 100 cents, meaing that a full octave spans 1200 cents.  Go ahead and set up some Rust constants:

```rust
type Cents = f64;
const SEMITONE_CENTS: Cents = 100.0;
const OCTAVE_SEMITONES: u32 = 12;
const OCTAVE_CENTS: Cents = SEMITONE_CENTS * OCTAVE_SEMITONES as f64;
```

The ratio between frequencies separated by a *single* cent is the 1200th root of 2, or 2^1/1200.  You wouldn't be able to hear a distinction between two tones a single cent apart.  The [Just-noticable difference](https://en.wikipedia.org/wiki/Just-noticeable_difference) is about 5 or 6 cents.

Knowing all this we can calculate the frequency in Hertz of a desired pitch if we know both a base frequency and the number of cents to increase by:

![cents formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/920411bb22d357b13f69a76fa33557c707f7cb57)

Here, *a* is the initial frequency in Hertz, `b` is the target frequency, and *n* is the number of cents by which to increase `a`.

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

This works out to just shy of 4 cents to cause an increase of 1Hz, more precisely around 3.9302 for a base frequency of 440:


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
+     fn add_semitones(&mut self, semitones: u32) {
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

##### Modes

Now we can start defining scales.

// major : 0,2,4,5,7,9,11,12

// minor : 0,2,3,5,7,8,10,12

##### Other Scales

Okay, Ben.  Ben, okay.  Okay, Ben.  But what about the pentatonic scale:

```txt

```

This corresponds to playing just the black keys on a piano, starting from 

Alright.  

### Listen To Any Arbitrary File

TODO maybe?  maybe not?  

TODO Rick & Morty "Human Music" gif

## Challenges

* Port this to your favorite programming language (second favorite if that's already Rust).
* Add some more scales.
* Extend beyond one octave.
* Signal processing.
    - Write a post to teach me signal processing