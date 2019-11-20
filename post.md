# Everything Is Music

We're going to teach our computers to sing using [Rust](https://www.rust-lang.org/), along with a little light physics and music theory.

Before we get started, take a few minutes to watch Icelandic musical tour de force [Björk](https://en.wikipedia.org/wiki/Bj%C3%B6rk) dismantle her televesion:

{% youtube 75WFTHpOw8Y %}

TODO make sure this is actually the right clip

As she so succinctly and charmingly notes: "everything is music".  Let's extract some music from our computers - it's been sitting right there.

## Motivation

This post was inpsired by this `bash` one-liner:

```bash
cat /dev/urandom | hexdump -v -e '/1 "%u\n"' | awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }' | xxd -r -p | aplay -c 2 -f S32_LE -r 16000
```

Check out a blog post about how that line specifically works [here](https://blog.robertelder.org/bash-one-liner-compose-music/).  This post explores the same logic but in Rust.  If you're anything like me, though, you kinda glaze over when first looking at token-soup like that.  The fact that it's so succint, though, should tell you we're actually up against a relatively straightforward operation.  Here's a friendlier walktrhough that doesn't require a Linux administration background to understand:

1. Get a random stream of binary data
1. Convert binary to 8-bit integers (0-255)
1. Map integers to notes

## Implementation

### Random input data

// Get random number stream
// Map random data to u8 stream

### Mapping Bytes To Notes

This is the meat of the program - turning our numeric data into something we can hear.  To get from random numbers to sounds we can hear, we need to map each data point to an amplitude.  The relevant section of the `bash` again:

```bash
awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }'
```

No, just mashing your keyboard will likely not yield similar results.  I tried it myself so you don't have to.  Tools like `awk` are terse, but this is merely a `for` loop with some math in the body.

#### A Little Physics

A sound at a specific pitch is a consequence of that sound's frequency.  You may know that sound travels as a "wave":

TODO Sound Graph



#### A Little Music Theory

Sound is a continuous spectrum of frequency, but when we make music deliberatly we tend to use scales to pick which frequency to use.

// semitones : 0,2,4,5,7,9,11,12

Whole, whole, half, whole, whole, whole, half

Think of a super cool way to abstract this concept

### Play The Sound

### Define Non-Random Inputs

//  the frequency in Hertz of a musical note with equal temperament: 440 * 2^(semitone distance / 12).
// 440 being A4

// inside a for loop (i = 0; i < 1; i += 0.0001)
// printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i))

// this prints formatted 4-byte hex representing amplitute of the sound wave
// all is multiplied by 100 (scalar for volume control) - TODO structopt
// 1382 is ABOUT 440 * Pi - use RUST for this - constexpr??
// The bash verison uses 2^x = e^(x*ln(2)), we can just use 2^x
// 100 * sin((440*Pi) * (pick a random semitone / 12) * i)

// THEN instead of xxd, convert back into binary
// Use aplay to play actual sound - I bet there's a Pure Rust way to do this

// Minor scale

### Listen To Any Arbitrary File

### Music Authoring

TODO Rick & Morty "Human Music" gif

## Challenge

* Port this to your favorite programming language (second favorite if that's already Rust)
* Write your favorite melody
