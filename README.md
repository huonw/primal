# primal

[![crates.io](https://img.shields.io/crates/v/primal.svg)](https://crates.io/crates/primal)
[![documentation](https://docs.rs/primal/badge.svg)](https://docs.rs/primal)
![minimum rustc 1.36](https://img.shields.io/badge/rustc-1.36+-red.svg)

`primal` puts raw power into prime numbers.

This crates includes

- optimised prime sieves
- checking for primality
- enumerating primes
- factorising numbers
- estimating upper and lower bounds for π(*n*) (the number of primes
  below *n*) and *p<sub>k</sub>* (the <i>k</i>th prime)

This uses a state-of-the-art cache-friendly Sieve of Eratosthenes
to enumerate the primes up to some fixed bound (in a memory
efficient manner), and then allows this cached information to be
used for things like enumerating and counting primes.

`primal` takes around 2.8 seconds and less than 3MB of RAM to
count the exact number of primes below 10<sup>10</sup> (455052511)
on the author's laptop (i7-3517U).

[**Documentation**](http://docs.rs/primal/)

### no_std
This crate can be built for no_std environments by enabling the no-std feature.
