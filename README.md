# music

Random music!

Install `libasound2-dev`, e.g. `sudo apt install libasound2-dev`.

- Run tests - `cargo test`
- Run program - `cargo run`

Usage:

```txt
Usage: music [OPTIONS]

Options:
  -p, --pitch-mode             Single-pitch mode
  -b, --base-note <BASE_NOTE>  The base note to calculate the scale from [default: C4]
  -s, --scale <SCALE>          The series of intervals from the base note to use per octave [default: Ionian]
  -o, --octaves <OCTAVES>      Number of octaves over which to range, anything over 8 gets parsed as 8 [default: 1]
  -h, --help                   Print help
  -V, --version                Print version
```

For example:

```txt
$ cargo run -- -s locrian -b Eb2 -o 3
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/music -s locrian -b Eb2 -o 3`
.: Cool Tunes :.
Generating music from the E♭ Locrian mode
Octaves: 2 - 5
[ E♭ E F# G# A B C# E♭ ]
```
