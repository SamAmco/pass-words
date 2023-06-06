# pass-words

Who says you can't use lowercase words for passwords!

pass-words is a simple command line application I threw together in rust that generates passwords using random short English words.

First clone the repo and then run `make && make install`. You will need rust and cargo installed. 

## User guide 

Just run: 

`pass-words <num_lines> [num_iterations]`

e.g.

```
pass-words 4 3
chyak bugle brugh unson chyakbuglebrughunson
libra tilda sate atony  libratildasateatony
vinas campo dard kappa  vinascampodardkappa
```

Granted I've never heard of most of the words. They are all the words from https://github.com/dwyl/english-words that are 5 characters or less and containing characters a-z only.

## Why use words?

Words are convenient to remember for a short period of time. If I need to share a password with someone, it's not a pain to read it out to them. If I want to remember it for later in the day, I usually can.

While using long, random sequences of letters, numbers, and symbols may offer more permutations, it often leads to copying and pasting them (leaving them in your clipboard), texting them to yourself, or writing them in plain text somewhere. Alternatively, you might rely on a remote password manager like LastPass and entrust them with all your passwords (no thanks). Introducing such friction into your login process inevitably leads to moments of weakness and creates new security problems.

## Is this a secure password generator?

Meh sorta. If you want a truly secure password use something like [pwgen](https://linux.die.net/man/1/pwgen).

There are 37315 possible words in the file. If you use a 5 word password that's 37315^5 possible passwords. 

Let's assume your adversary can brute force 100 trillion passwords per second it would take 
(((37315 ^ 5)) / 100000000000000) / (60 * 60 * 24 * 365) years to iterate all possible combinations which is 11.45 years on average. 

It's unlikely anyone would direct that much computing power at cracking your password but it's good practice to change your passwords every few years as well anyway. 

Really though use this at your own risk DYOR and all that.
