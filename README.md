# pass-words

Who says you can't use lowercase words for passwords!

pass-words is a simple command line application I threw together in rust that generates passwords using random short English words.

## Why use words?

Words are convenient to remember for a short period of time. If I need to share a password with someone, it's not a pain to read it out to them. If I want to remember it for later in the day, I usually can.

While using long, random sequences of letters, numbers, and symbols may offer more permutations, it often leads to copying and pasting them (leaving them in your clipboard), texting them to yourself, or writing them in plain text somewhere. Alternatively, you might rely on a remote password manager like LastPass and entrust them with all your passwords (no thanks). Introducing such friction into your login process inevitably leads to moments of weakness and creates new security problems.

## Is this a secure password generator?

Meh sorta. If you want a truly secure password use something like [pwgen](https://linux.die.net/man/1/pwgen).

There are 10,000 possible words in the in pass-words. If you use a 5 word password that's 10000^5 possible passwords. 

Let's assume your adversary can brute force 1 billion passwords per second it would take (37315^5 / 1000000000) / (60 * 60 * 24 * 365) years to iterate all possible combinations which is about 1,585 years on average. 

It's worth noting that the number of passwords an adversary can check per second could be much greater or lower depending on the password hashing algorithm being used by the service storing your password. Modern password hashing algorithms such as Aargon2id are much more computationally intensive to crack using memory hard problems to avoid the power of large scale GPU/ASIC crackers. Ultimately the security of your password is partly in their hands so if your password really needs to be secure and you don't trust the service to use a good password hashing algorithm then again consider using something like [pwgen](https://linux.die.net/man/1/pwgen).

Always use a different password for every account you create. If one service stores your password badly it is now in-secure for all other accounts using the same password.

Use this at your own risk and DYOR on password management.

## Ok how do I use it?

First clone the repo and then run `make && make install`. You will need rust and cargo installed. 

The makefile will install to `$HOME/.local/bin` If you're using an OS other than Linux may god have mercy on your soul.

Then just run: 

`pass-words <num_lines> [num_iterations]`

e.g.

```
$ pass-words 4 3
finite might ranks wearing	finitemightrankswearing
publication judges oil requests	publicationjudgesoilrequests
harvard fleet tribunal respected	harvardfleettribunalrespected

Selected 4 words out of a list of size 10000
At 1 billion checks per second this password could be cracked in 0.15854895991882292 years
At 1 million checks per second this password could be cracked in 158.54895991882293 years
At 1 thousand checks per second this password could be cracked in 158548.95991882292 years
```
