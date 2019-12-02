# music

Random music!

* Run tests - `cargo test`
* Run program - `cargo run`

```txt
$ cargo run -- -h
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/music -h`
music 0.1.0
music is a procedural single-tone melody generator

USAGE:
    music [FLAGS] [OPTIONS]

FLAGS:
    -h, --help          Prints help information
    -p, --pitch-mode    Single-pitch mode
    -V, --version       Prints version information

OPTIONS:
    -b, --base-note <base-note>    The base note to calculate the scale from [default: C4]
    -o, --octaves <octaves>        Number of octaves over which to range, anything over 8 gets parsed as 8 [default: 1]
    -s, --scale <scale>            The series of intervals from the base note to use per octave [default: Ionian]
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
