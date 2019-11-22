---
title: Teaching Numbers To Sing
published: false
description: Learn how to generate sound from numeric data in Rust.
cover_image: https://thepracticaldev.s3.amazonaws.com/i/iuakwwcexql5u0th7gtm.jpg
tags: beginners, rust, tutorial, music
---

# Everything Is Music

TODO TESTS THROUGHOUT

> Everything is music. When I go home, I throw knickers in the oven and it's music. Crash, boom, bang!

*- [Winona Ryder](https://en.wikipedia.org/wiki/Winona_Ryder) as [Björk](https://en.wikipedia.org/wiki/Bj%C3%B6rk) on [SNL](https://en.wikipedia.org/wiki/Saturday_Night_Live)'s [Celebrity Rock 'N' Roll Jeopardy!](https://en.wikipedia.org/wiki/Celebrity_Jeopardy!_(Saturday_Night_Live)) - [2002](https://en.wikipedia.org/wiki/2002) - [YouTube](https://youtu.be/R3V94ZtmdbQ?t=190)*

Let's channel that wacky energy.  In this post, we'll throw something [random](https://en.wikipedia.org/wiki/Random_number_generation) into, well, a [math-oven](https://en.wikipedia.org/wiki/Subroutine) and [*viola*](https://en.wikipedia.org/wiki/Viola), [music](https://en.wikipedia.org/wiki/Music)!

In other words, we're going to teach our [computers](https://en.wikipedia.org/wiki/Personal_computer) to ["sing"](https://en.wikipedia.org/wiki/Singing) using [Rust](https://www.rust-lang.org/), backed by a little light [physics](https://en.wikipedia.org/wiki/Physics) and [music theory](https://en.wikipedia.org/wiki/Music_theory).

[¡Vámonos!](https://en.wikipedia.org/wiki/Party)

## Table of Contents

- [Preamble](#preamble)
- [The Meme](#the-meme)
- [The Program](#the-program)
  * [Random input data](#random-input-data)
    + [`Iterator`](#iterator)
  * [Mapping Bytes To Notes](#mapping-bytes-to-notes)
    + [A Little Physics](#a-little-physics)
      - [Sine Waves](#sine-waves)
      - [Pitch](#pitch)
    + [A Little Music Theory](#a-little-music-theory)
      - [Scales](#scales)
      - [Cents](#cents)
      - [Piano Keys](#piano-keys)
      - [Modes](#modes)
      - [Other Scales](#other-scales)
    + [Back To The Bytes](#back-to-the-bytes)
  * [Listen To Your Files](#listen-to-your-files)
- [Challenges](#challenges)

## Preamble

I have two disclaimers:

1. [There are](https://en.wikipedia.org/wiki/Existence) [too many](https://en.wikipedia.org/wiki/Saturated_model) [Wikipedia](https://en.wikipedia.org/wiki/Main_Page) [links](https://en.wikipedia.org/wiki/Hyperlink) [here](https://en.wikipedia.org/wiki/Blog).  [If](https://en.wikipedia.org/wiki/Conditional_(computer_programming)) [you're](https://en.wikipedia.org/wiki/You) [that](https://en.wikipedia.org/wiki/Autodidacticism) [kind](https://en.wikipedia.org/wiki/Impulsivity) [of](https://en.wikipedia.org/wiki/Preposition_and_postposition) [person](https://en.wikipedia.org/wiki/Person), [set](https://en.wikipedia.org/wiki/Innovation) [rules](https://en.wikipedia.org/wiki/Law).
1. Further to Point 1, most of this I learned myself on Wikipedia.  The rest is what I remember from [high school](https://en.wikipedia.org/wiki/High_school_(North_America)) as a [band geek](https://en.wikipedia.org/wiki/Euphonium), which was over [ten years](https://en.wikipedia.org/wiki/Decade) [ago](https://en.wikipedia.org/wiki/Past).  I do believe it's generally on the mark, but I am making no claims of authority.  If you see something, [say something](https://en.wikipedia.org/wiki/Say_Something_(Justin_Timberlake_song)).


This is (hopefully) a [beginner](https://en.wikipedia.org/wiki/Novice)-level post.  It's not necessarily specific to Rust but also not shy about Rust idioms or added [verbosity](https://en.wikipedia.org/wiki/Verbosity).  Even so, or perhaps because of, it should be pretty readable even if you don't speak Rust - that's the whole point!  I promise I'll (mostly) stop the whole parenthesis thing, too.

## The Meme

This post was inspired by this meme:

![the meme](https://i.redd.it/uirqnamnjpz31.jpg)

Here's a slightly modified version of the [`bash`](https://en.wikipedia.org/wiki/Bash_(Unix_shell)) one-liner at the bottom, taken from [this blog post](https://blog.robertelder.org/bash-one-liner-compose-music/) by [Robert Elder](https://www.robertelder.org/) that explores it:

```bash
cat /dev/urandom | hexdump -v -e '/1 "%u\n"' | awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }' | xxd -r -p | aplay -c 2 -f S32_LE -r 16000
```

The linked blogpost is considerably more brief and assumes a greater degree of background knowledge than this particular adventure, but that's not to discredit it at all - that write-up and Wikipedia were all I needed to complete this translation with absolutely not a clue how this whole thing worked going in.  If you'd like the challenge of implementing this yourself blind, stop right here. Read just that, try it yourself in the language of your choice, come back when you get stuck.  The stuff here should apply to whatever you've got unless you've gone real funky.  This post does extend that functionality, so, you know, maybe still come back anyway.

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

We can glean a bit of information at a glance, though, and depending on your current comfort with this domain you may be able to kind of understand the general idea here.  It looks like we're going to tick up floating point values by ten-thousandths from zero to one (`for (i = 0; i < 1; i += 0.0001)`), and do... I don't know, some math and stuff on each value based on the list `[0,2,4,5,7,9,11,12]`: `100 * sin(1382 * exp((a[$1 % 8] / 12) * log(2)) * i))` .

Even if you're not a math person, this alone may have triggered [something](https://en.wikipedia.org/wiki/Unit_circle) from deep within your teenage math textbooks.  If it did, don't sweat it, I'm not going to be drilling you on the [radian](https://en.wikipedia.org/wiki/Radian) unit circle values, but we are going to be working with a sine wave.  If it didn't, also don't sweat it, I'm gonna walk us through the whole thing and it's really not painful.  After we do the math, we're going to print it out as an 8-digit hex number: `printf("%08X\n",math())` - this [`printf`](https://en.wikipedia.org/wiki/Printf_format_string) formatter means we want a [0-padded](https://en.wikipedia.org/wiki/Npm_(software)#Notable_breakages) number that's 8 digits long in [upper-case](https://en.wikipedia.org/wiki/Letter_case) [hexadecimal](https://en.wikipedia.org/wiki/Hexadecimal).  Te [base 10] integer [`42`](https://en.wikipedia.org/wiki/Phrases_from_The_Hitchhiker%27s_Guide_to_the_Galaxy#Answer_to_the_Ultimate_Question_of_Life,_the_Universe,_and_Everything_(42)) would be printed as `0000002A`.

This code goes a little further than the one-liner can - thank gosh, we hit XXX lines here.  ¡Vámonos!

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

```diff
  [dependencies]

+ hound = "3.4"
+ rand = "0.7"
+ rodio = "0.10"
```

### Random input data

This crate is quite featureful, but we're keeping it simple.  Add an import to the top of `src/main.rs`:

```rust
use rand::random;
```

#### Iterator

We can skip the conversion from binary.   This crate can give us random 8-bit integers out of the box by ["turbofish"](https://docs.serde.rs/syn/struct.Turbofish.html)ing a type: `random::<u8>()` will produce a random [unsigned](https://en.wikipedia.org/wiki/Signedness) [8 bit](https://en.wikipedia.org/wiki/8-bit) integer ([`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) with the default generator settings.  See the crate docs for all the various ways to tune this.

We can implement a similar result to the first two steps, or `cat /dev/urandom | hexdump -v -e '/1 "%u\n"'` by manually implementing an [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html).  This trait is the standard way to represent, well, things that we iterate over, and this will easily let us represent what's essentially an infinite list.  It's easy to implement manually if a standard collection isn't right.  There's a [rich library](https://doc.rust-lang.org/std/iter/trait.Iterator.html) for types that implement this trait that you can take advantage of quickly.   There's only the one method:

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

This struct itself doesn't need to store any [state](https://en.wikipedia.org/wiki/State_(computer_science)).  We just always want to produce the next value by calling `rand::random()`, specified with the associated type of this iterator.  I set `Item` to `u8`, so calling `Random::Input::next()` will always return a `random::<u8>()` - there's no `None` branch, just `Some(x)`.  That means `unwrap()` is always safe to call on this iterator, it won't panic.  You can take it for a spin with this driver code:

```rust
fn main() {
    let mut rands = RandomBytes::new();
    loop {
        println!("{}", rands.next().unwrap());
    }
}
```

This should print an endless loop of random numbers between 0 and 255 inclusive until you kill the process.  Be careful messing with this particular `Iterator` - a lot of those library functions we mentioned are really assuming your list is _NOT_ infinite.  This is not [lazily-evaluated](https://en.wikipedia.org/wiki/Lazy_evaluation), so something like `RandomBytes::last()` will sit there and call `next()` until your struct tells it it needs to stop somehow (which it does not ever do).  It does not bode well for anything else you had planned for the rest of this process.

### Mapping Bytes To Notes

This is the meat of the program - turning our numeric data into something we can hear.  To get from random numbers to sounds we can hear, we need to map each data point to an amplitude.  The relevant section of the `bash` again:

```bash
awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }'
```

Tools like `awk` are terse, but this is merely a `for` loop with some math in the body.

#### A Little Physics

[Sound](https://en.wikipedia.org/wiki/Sound) is composed physically of [vibrations](https://en.wikipedia.org/wiki/Vibration).  These vibrations cause perturbations in some [medium](https://en.wikipedia.org/wiki/Transmission_medium), and those perturbations are what we experience as sound.  When we're talking about [hearing](https://en.wikipedia.org/wiki/Hearing) a sound with our [ears](https://en.wikipedia.org/wiki/Ear), the medium is usually [air](https://en.wikipedia.org/wiki/Atmosphere_of_Earth).

##### Sine Waves

Sound propagates as a [wave](https://en.wikipedia.org/wiki/Wave).  In [reality](https://en.wikipedia.org/wiki/Reality) a sound contains many components but for this program we can talk about a super-simplified version that can be represented as a single [sine wave](https://en.wikipedia.org/wiki/Sine_wave):

![sine waves](https://upload.wikimedia.org/wikipedia/commons/6/6d/Sine_waves_different_frequencies.svg)

If you're thinking *but Ben, you CAN mix component frequencies to represent sound waves as sine waves in fact we all do that all the time*, you're [correct in ways I don't personally fully understand](https://en.wikipedia.org/wiki/Digital_signal_processing).  That's really cool stuff and a lot more complicated than what happens in this post.  If that was either turning you {on|off} to this, you can {stop|start} breathing normally.  There will be no signals processed here, just a single frequency [scalar](https://en.wikipedia.org/wiki/Variable_(computer_science)) we modulate.

If the X axis is time, a sine wave represents a recurring action with an analog (or smooth) oscillation between their maximal amplitudes, or distances in either direction from 0.  The frequency is how close together these peaks are, or how frequently this thing occurs.  In simple cases, a sound at a specific pitch is a result of that sound's frequency.  The higher the frequency, or closer together the peaks, the higher the pitch.  The amplitude reflects the volume.

The standard unit for frequency is the [Hertz](https://en.wikipedia.org/wiki/Hertz), abbreviated `Hz`, which measures the *number of cycles per second*.  One cycle here is the distance (or time) between two peaks on the graph:

![cycle gif](https://media.giphy.com/media/F5rQlfTXqCJ8c/giphy.gif)

##### Pitch

Sound is a continuous spectrum of frequency, but when we make music we tend to prefer [notes](https://en.wikipedia.org/wiki/Musical_note) at set frequencies, or pitches.  I'm using  [frequency](https://en.wikipedia.org/wiki/Fundamental_frequency) and [pitch](https://en.wikipedia.org/wiki/Pitch_(music)) interchangeably, because for this application specifically they are, but go Wiki-diving if you want to learn about the distinction and nuance at lay here.  The nature of sound is super cool but super complex and outside of the scope of this post - we just want to hear some numbers sing, we don't need to hear a full orchestra.

To start working with something concrete, we need some sort of standard.   Some of the world has settled on [440Hz](https://en.m.wikipedia.org/wiki/A440_(pitch_standard)) - it's [ISO](https://en.wikipedia.org/wiki/International_Organization_for_Standardization) [16](https://www.iso.org/standard/3601.html), at least.  It's also apparently called "The Stuttgart Pitch", which is funny.

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

With this code we can use `Pitch::default()` to get our A440 pitch, or pass an arbitrary frequency: `Pitch::new(440.0)`.

Let's see if we can produce this tone.

TODO produce the flat tone - I think it's just gonna be 440*i*Pi

#### A Little Music Theory

A440 is the A above Middle C on a piano:

![piano](https://upload.wikimedia.org/wikipedia/commons/thumb/2/2e/Piano_Frequencies.svg/2560px-Piano_Frequencies.svg.png)

The cyan key is Middle C, and A440 is highlighted in yellow.  The octaves on an 88-key piano are numbered as shown, so often A440 is simply denoted "A4" especially when dealing with a keyboard specifically.  You may own a tuner that marks 440Hz/A4 specifically if you're a musician.  This pitch is used for calibrating musical instruments and tuning a group, as well as a baseline constant for calculating frequencies.

##### Scales

A [scale](https://en.wikipedia.org/wiki/Scale_(music)) is a series of notes (frequencies) defined in terms of successive intervals from a base note.  The smallest of these intervals on a piano (and most of Western music) is called a [semitone](https://en.wikipedia.org/wiki/Semitone), also called a minor second or half step.  Take a look back at that piano diagram above - one semitone is the distance between an adjacent white key and black key.  A *whole* step, or a [major second](https://en.wikipedia.org/wiki/Major_second), is equal to two of semitones, or two adjacent white keys that pass over a black key.  For now these are the only intervals we'll need:

```rust
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Interval {
    Min2 = 1,
    Maj2 = 2,
}
```

Each variant of this [`enum`](https://en.wikipedia.org/wiki/Tagged_union#2010s) also carries the number of semitones it represents.

Clearly, there isn't a black key between every white key.  The piano is designed to play notes from a catagory of scales called [diatonic scales](https://en.wikipedia.org/wiki/Diatonic_scale), where the full range of an octave consists of five whole steps and two half steps.  We can see this visually on the keyboard - it has the same 8-length whole/half step pattern for the whole length.

A major scale is the baseline scale.  Start at Middle C, the one highlighted in cyan above, and count up to the next C key, eight white keys to the left.  Each time you skip a black key is a whole step and if the two white keys are adjacent it's a half step.  These are the steps you get counting up to the next C, when the pattern repeats:

```txt
whole, whole, half, whole, whole, whole, half
```

TODO embed sound

There are a few variations of *minor* scale, but for now I'll define the [natural minor scale](https://en.m.wikipedia.org/wiki/Minor_scale#Natural_minor_scale).  This is what you get if you start at our good old friend A4 and march on up the white keys to A5:

```txt
whole, half, whole, whole, half, whole, whole
```

TODO embed sound

There are the same number of whole and half intervals, they're just distributed differently.  You can play a corresponding minor scale using only the white keys by simply starting at the sixth note.  Try counting it out yourself!

##### Cents

The reason an octave is where the pattern restarts is that we're working in a tuning system called [equal temperment](https://en.wikipedia.org/wiki/Equal_temperament).  This means any two adjacent notes in this system have the same ratio.  There's a reason diatonic scales move up eight keys on a piano, or twelve semitones: the frequency ratio over an octave is 2:1.  Middle C is C4, A5 is thwi

To calculate the value needed in Hertz, we need a more precise way to describe an interval.  A full octave has a frequency ratio of 2:1, meaning a note one octave higher has double the frequency of the lower.  This results in an exponential curve if you were to graph frequencies as they grow.  When working with such a curve there's often a corresponding logarithmic unit that turns that curve into a line.  For musical intervals this unit called a [cent](https://en.wikipedia.org/wiki/Cent_(music)) to represent the ratio between two frequencies.  We've already seen how each octave is divided into 12 semitones:

```txt
whole, whole, half, whole, whole, whole, half
  2  +  2   +  1  +   2   +  2  +   2  +  1   =  12  
```

Each semitone is defined as 100 cents meaning a full octave spans 1200 cents.  Go ahead and set up some constants:

```rust
type Cents = f64;
const SEMITONE_CENTS: Cents = 100.0;
const OCTAVE_SEMITONES: u32 = 12;
const OCTAVE_CENTS: Cents = SEMITONE_CENTS * OCTAVE_SEMITONES as f64;
```

The ratio between frequencies separated by a *single* cent is the 1200th root of 2, or 2^1/1200.  You wouldn't be able to hear a distinction between two tones a single cent apart.  The [just-noticeable difference](https://en.wikipedia.org/wiki/Just-noticeable_difference) is about 5 or 6 cents.

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

##### Piano Keys

Let's do one better and just use piano keys:

```rust
#[derive(Debug, Clone, Copy)]
struct PianoKey {
    note: u8,
    octave: u8,
}

impl PianoKey {
    fn new(note: u8, octave: u8) -> Self {
        PianoKey { note, octave }
    }
}
```

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

##### Modes

Now we can start defining scales.  We actually get seven of these for free - one for each of the white keys in an octave.  If you count up to one octave higher from any given key, that's a diatonic scale.  These scales are called [`Modes`](https://en.wikipedia.org/wiki/Mode_(music)).

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

*Body images are wikimedia commons unless otherwise specified*
*Cover image from some subreddit, I don't remember*