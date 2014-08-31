# slow_primes

[![Build Status](https://travis-ci.org/huonw/slow_primes.png)](https://travis-ci.org/huonw/slow_primes)

Extremely simplistic and relatively unoptimised handling of basic
tasks around primes:

- checking for primality
- enumerating primes
- factorising numbers

This uses a basic Sieve of Eratosthenes to enumerate the primes upto
some fixed bound (in a relatively memory efficient manner), and then
allows this cached information to be used for factorisation via trail
division and just enumerating the numbers.

(Despite the name, it can sieve the primes up to 10<sup>9</sup> in
about 5 seconds.)

[**Documentation**](http://www.rust-ci.org/huonw/slow_primes/doc/slow_primes/)
