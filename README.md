# Exercises in Programming Style - Rust Version [![Build Status](https://travis-ci.org/shybyte/exercises-in-programming-style-rust.svg?branch=master)](https://travis-ci.org/shybyte/exercises-in-programming-style-rust)
 
Comprehensive collection of well-known, and not so well-known, programming styles using a simple computational task, *term frequency*.
This is the Rust Version. For the original python version and the companion book see
[https://github.com/crista/exercises-in-programming-style](https://github.com/crista/exercises-in-programming-style).

## List of Exercises

### Part 1 - Historical

Feel the pain of programming in 1970. 

* Good old times - Minimal memory and no identifiers
* Go Forth - Working on a stack

### Part 2 - Basic Styles

It happens every day. You have seen it already.

* [Monolithic](src/bin/03-monolith.md) - One big program with no named abstractions and no clever library usage
* [Cookbook](src/bin/04-cookbook.md) - Procedural code with global state
* [Pipeline](src/bin/05-pipeline.md) - (Impure) Nested Functions
* [Code Golf](src/bin/06-code-golf.md) - Short code by clever std library usage 

### Part 3 - Function Composition

Combine functions in interesting ways.

* Infinite Mirror - Recursively, call yourself.
* Kick Forward - Callback style like in good old javascript
* [The One](src/bin/09-the-one.md) - Kind of a Monad, Functional composition


### Part 4 - Objects and Object Interaction

Stateful encapsulated things talk with other encapsulated stateful things. 

* [Things](src/bin/10-things.md) - Object-oriented 
* Letterbox - Communication by untyped messages between objects
* Closed Maps - An Object is just a Map of functions
* Abstract Things - Objects implement Interfaces (Traits)
* Hollywood - Objects need to register at a fancy framework, which calls the objects
* Bulletin Board - Objects communicate via an Eventbus

### Part 5 - Reflection and Metaprogramming

When code knows about itself and hacks itself. 

* Introspective - Code know about itself
* Reflective - Code can modify itself
* Aspects - Add general aspects like logging to existing code, without modifying the source code
* Plugins - Load external code without recompiling 

### Part 6 - Adversity 

How to prevent errors or deal with it.

* Constructivist - Check Arguments. If something goes wrong, use fallbacks and defaults
* Tantrum - Check Arguments. Handle problems when the occur by (logging and) forwarding the error to the caller
* Passive Aggressive - Check Arguments. Forward problems to the caller as default (Exceptions)
* Declared Intentions - Declare and enforce types
* Quarantine - Handle IO differently (Kind of IO monad)

### Part 7 - Data-Centric

It's about relations between data.

* Persistent Table - Put data in a (relational) database and query it
* Spreadsheet - Like dependent cells in a excel sheet
* Lazy Rivers - Streams of data instead of loading everything into memory at once

### Part 8 - Concurrency

* Actors
* Dataspaces
* Map Reduce
* Double Map Reduce

### Part 9 - Interactivity

* Trinity
* Restful

### Part 10 - Rusty

* [Rusty](src/bin/42-rusty.md) - Idiomatic efficient Rust solution with complete error handling




## Run one variant

```
  cargo run --bin 06-code-golf test-data/input/simple.txt
```


## Run and test everything


```
  ./test.sh
```


## License

MIT

## Copyright

Copyright (c) 2017 Marco Stahl
