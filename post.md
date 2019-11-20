# Everything Is Music

We're going to teach our computers to sing using [Rust](https://www.rust-lang.org/), along with a little music theory and a little physics.

Before we get started, take a few minutes to watch Icelandic musical tour de force [Björk](https://en.wikipedia.org/wiki/Bj%C3%B6rk) dismantle her televesion:

{% youtube 75WFTHpOw8Y %}

TODO make sure this is actually the right clip

As she so succinctly and charmingly notes: "everything is music".  Let's extract some music from our computers - it's been sitting right there.

## Motivation

This post was inpsired by this `bash` one-liner:

```bash
cat /dev/urandom | hexdump -v -e '/1 "%u\n"' | awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }' | xxd -r -p | aplay -c 2 -f S32_LE -r 16000
```

Check out a blog post about how that line specifically works [here](https://blog.robertelder.org/bash-one-liner-compose-music/).  This post explores the same logic but in Rust.

## Implementation

### Random input data

// Get random number stream
// Map random data to u8 stream

// semitones : 0,2,4,5,7,9,11,12

### Mapping Bytes To Notes

#### A Little Music Theory

#### A Little Physics

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

// THEN - author your own!

TODO Rick & Morty "Human Music" gif

## Challenge

* Port this to your favorite programming language
* Write your favorite melody
