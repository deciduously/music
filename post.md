We're going to teach our computers to sing using a little [Rust](https://www.rust-lang.org/, a little music theory, and a little physics.

Before we get started, take a few minutes to watch Icelandic musical tour de force [Björk](https://en.wikipedia.org/wiki/Bj%C3%B6rk) dismantle her televesion:

{% youtube 75WFTHpOw8Y %}

TODO make sure this is actually the right clip

As she so succinctly and charmingly notes: "everything is music".  Let's extract some music from our computers - it's been sitting right there.

This post was inpsired by this `bash` one-liner:

```bash
cat /dev/urandom | hexdump -v -e '/1 "%u\n"' | awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }' | xxd -r -p | aplay -c 2 -f S32_LE -r 16000
```

Check out a blog post about how that line specifically works [here](https://blog.robertelder.org/bash-one-liner-compose-music/).  This post explores the same logic but in Rust.

TODO Rick & Morty "Human Music" gif

## Challenge

* Port this to your favorite programming language
* Write your favorite melody
