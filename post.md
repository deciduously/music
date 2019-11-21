---
title: Teach Your Computer To Sing
published: false
description: 
tags: beginners, rust, tutorial, music
---

# Everything Is Music

We're going to teach our computers to sing using [Rust](https://www.rust-lang.org/), along with a little light physics and music theory.  This is (hopefully) a beginner-level post.  It's not necessarily Rust-specific, but the code is potentially a little Rust-idiosyncratic for the totally uninitiated.

We'll start, as any worthwhile tutorial should, with a quote from one of SNL's *Celebrity Jeopardy!* sketches in which Winona Ryder is channeling Icelandic music icon [Björk](https://en.wikipedia.org/wiki/Bj%C3%B6rk):

> Alex Trebek: Björk, this is the only thing that becomes toast.
Björk: Everything is music. When I go home, I throw knickers in the oven and it's music. Crash, boom, bang! (buzz)
Alex Trebek: Wow. The answer, of course, was bread.

Here's a [YouTube link](https://youtu.be/R3V94ZtmdbQ?t=190) to this moment.  Let's channel that energy - we'll throw some random numbers into the Rust compiler, and extract some music!

## Table of Contents

TODO maybe??

## The Linux One-liner

### The Meme

This post was inpsired by this meme:

![the meme](https://i.redd.it/uirqnamnjpz31.jpg)

Here's a slightly modified version of the `bash` one-liner at the bottom:

```bash
cat /dev/urandom | hexdump -v -e '/1 "%u\n"' | awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }' | xxd -r -p | aplay -c 2 -f S32_LE -r 16000
```

No, just mashing your keyboard will (likely) not yield similar results.  I tried it myself so you don't have to.  Here's a step-by-step video demonstration:

{% youtube uLhQQSKhTok %}

### The Code

There's a blog post about how that line specifically works [here](https://blog.robertelder.org/bash-one-liner-compose-music/).  Here's a friendlier breeze-through of the `bash`:

1. `cat /dev/urandom`: Get a stream of random binary data
1. `hexdump -v -e '/1 "%u\n"'`: Convert binary to 8-bit base-10 integers (0-255)
1. `awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }'`: Map integers to pitches, as 8-byte hexadecimal values
1. `xxd -r -p`: Convert hex numbers back to binary
1. `aplay -c 2 -f S32_LE -r 16000`: Play back binary data as sound

This, however, is not a post about these commands, nor is it a post about that series of steps exactly.  However, the underlying idea (really, step 3) is identical, and this gives us a roadmap.  ¡Vámonos!

## The Rust

As always, ensure you have at least the default stable Rust toolchain [installed](https://www.rust-lang.org/tools/install).  Then, spin up a new project:

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

This crate is [quite featureful](https://docs.rs/rand/0.7.2/rand/), but we're keeping it simple.  Add an import to the top of `src/main.rs`:

```rust
use rand::random;
```

#### Iterators

We can skip the whole conversion from binary - this crate can give us randum 8-bit integers out of the box.  We can implement a similar result to the first two steps, or `cat /dev/urandom | hexdump -v -e '/1 "%u\n"'` by manually implementing an [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html):

```rust
#[derive(Default)]
struct RandomInput;

impl RandomInput {
    fn new() -> Self {
        Self::default()
    }
}

impl Iterator for RandomInput {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        Some(random::<Self::Item>())
    }
}
```

The struct itself doesn't need to store any state - it will just always produce the next value by calling `rand::random()`, specified with the associated typoe of this iterator.  You can take it for a spin with this driver code:

```rust
fn main() {
    let mut rands = RandomInput::new();
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

Sound is composed physically of vibrations.  These vibrations cause perturbances in some medium, usually air, and propogate as a wave.  This wave can be represented as a [sine wave](https://en.wikipedia.org/wiki/Sine_wave):

![sine waves](https://upload.wikimedia.org/wikipedia/commons/6/6d/Sine_waves_different_frequencies.svg)

*image: [wikimedia commons](https://en.wikipedia.org/wiki/File:Sine_waves_different_frequencies.svg)*

If the X axis is time, a sine wave represents a recurring action with an analog (or smooth) oscillation between their maximal *amplitudes*, or distances in either direction from 0.  The *frequency* is how close together these peaks are, or how frequently this thing occurs.

It makes sense that a vibration propogating through a medium could be represented as such a wave - picture a string vibrating on a guitar.  It wobbles back and forth rapidly, just like this wave's shape.

In simple cases, a sound at a specific pitch is a result of that sound's frequency.  The higher the frequency, or closer together the peaks.

The standard unit for frequency is the [Hertz](https://en.wikipedia.org/wiki/Hertz), abbreviated `Hz`, which measures the *number of cycles per second*.  One cycle here is the distance (or time) between two peaks on the graph:

![cycle gif](https://media.giphy.com/media/F5rQlfTXqCJ8c/giphy.gif)

Sound is a continuous spectrum of frequency, but when we make music we tend to prefer *notes* at set frequencies.  To start, though, we need some sort of standard, and some of the world has settled on [440Hz](https://en.m.wikipedia.org/wiki/A440_(pitch_standard)) - it's [ISO 16](https://www.iso.org/standard/3601.html), at least.

It's also apparently called "The Stuttgart Pitch", which is funny.

![stuttgart](https://i.imgflip.com/3h0y3g.jpg)

*image: I made this on [imgflip.com](https://imgflip.com/) but have no proof of that*

Go ahead and toss that into your source file:

```rust
type Hertz = u32;
const STANDARD_PITCH: Hertz = 440;
```

Let's see if we can produce this tone.

I tend to name *all the things*.

#### A Little Music Theory

A440 is the A above Middle C on a piano:

![piano](https://upload.wikimedia.org/wikipedia/commons/thumb/2/2e/Piano_Frequencies.svg/2560px-Piano_Frequencies.svg.png)

*image: [wikimedia commons](https://commons.wikimedia.org/wiki/File:Piano_Frequencies.svg)*

The cyan key is Middle C, and A440 is highlighted in yellow.

If you're a musician you may own a tuner that marks 440Hz specifically.  This pitch is used for calibrating musical instruments and tuning a group, and we'll use it as a baseline constant for calculating frequencies.

A [scale](https://en.wikipedia.org/wiki/Scale_(music)) is a series of notes (frequencies) defined in terms of successive intervals from a base note.

The smallest of these intervals is called a [semitone](https://en.wikipedia.org/wiki/Semitone), also called a minor second.  Here I'll refer to it as a "half" step.  A major scale, also known as [Ionian mode](https://en.m.wikipedia.org/wiki/Mode_(music)), falls into a category called [diatonic scales](https://en.wikipedia.org/wiki/Diatonic_scale), where the full range of an octave consists of five *whole* steps, which is *two* semitones or a [major second](https://en.wikipedia.org/wiki/Major_second), and two half steps:

```txt
whole, whole, half, whole, whole, whole, half
```

TODO embed sound

There are a few variations of minor scale, but for this application I'll define the [natural minor scale](https://en.m.wikipedia.org/wiki/Minor_scale#Natural_minor_scale) (a.k.a. Aeolian mode):

```txt
whole, half, whole, whole, half, whole, whole
```

TODO embed sound

Note that this scale is still diatonic - there are the same number of whole and half intervals, they're just distributed differently. Actually, if you start the major scale at the sixth note you get a corresponding minor scale - when you reach the end, you've gone an octave and wrap back up to the beginning.  Try it yourself, counting on the examples above.

To madel this, we'll create another `Iterator`, but this time implemented for an `enum`:

```rust
ENUM
```

Then, we can define each variant's sequence.  I'm taking advantage of the face that it's actually the same sa

// major : 0,2,4,5,7,9,11,12

// minor : 0,2,3,5,7,8,10,12

Think of a super cool way to abstract this concept

### Play The Sound

### Listen To Any Arbitrary File

### Music Authoring

TODO - maybe??

TODO Rick & Morty "Human Music" gif

## Challenge

* Port this to your favorite programming language (second favorite if that's already Rust)
* Write your favorite melody

To learn more about asynchronous programming in Rust, I recommend the aptly named [Async Book](https://rust-lang.github.io/async-book/).