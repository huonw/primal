# slow_primes

[![Build Status](https://travis-ci.org/huonw/slow_primes.png)](https://travis-ci.org/huonw/slow_primes)

Extremely simplistic and relatively unoptimised handling of basic
tasks around primes:

- checking for primality
- enumerating primes
- factorising numbers
- estimating upper and lower bounds for π(*n*) (the number of primes
  below *n*) and *p<sub>k</sub>* (the <i>k</i>th prime)

This uses a basic Sieve of Eratosthenes to enumerate the primes up to
some fixed bound (in a relatively memory efficient manner), and then
allows this cached information to be used for things like enumerating
the primes, and factorisation via trial division.

(Despite the name, it can sieve the primes up to 10<sup>9</sup> in
about 5 seconds.)

[**Documentation**](http://huonw.github.io/slow_primes/slow_primes/)
