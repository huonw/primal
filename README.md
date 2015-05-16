# primal

[![Build Status](https://travis-ci.org/huonw/primal.png)](https://travis-ci.org/huonw/primal) [![Coverage Status](https://coveralls.io/repos/huonw/primal/badge.svg?branch=update)](https://coveralls.io/r/huonw/primal?branch=update) [![Crates.io](https://img.shields.io/crates/v/primal.svg)](https://crates.io/crates/primal)

Simplistic and relatively unoptimised handling of basic tasks around
primes:

- checking for primality
- enumerating primes
- factorising numbers
- estimating upper and lower bounds for π(*n*) (the number of primes
  below *n*) and *p<sub>k</sub>* (the <i>k</i>th prime)

This uses a basic Sieve of Eratosthenes to enumerate the primes up to
some fixed bound (in a relatively memory efficient manner), and then
allows this cached information to be used for things like enumerating
the primes, and factorisation via trial division.

(It can sieve the primes up to 10<sup>9</sup> in about 5 seconds.)

[**Documentation**](http://huonw.github.io/primal/primal/)
