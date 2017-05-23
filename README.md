# Exercises in Programming Style - Rust Version [![Build Status](https://travis-ci.org/shybyte/exercises-in-programming-style-rust.svg?branch=master)](https://travis-ci.org/shybyte/exercises-in-programming-style-rust)
 
Comprehensive collection of well-known, and not so well-known, programming styles using a simple computational task, *term frequency*.
This is the Rust Version. For the original python version and the companion book see
[https://github.com/crista/exercises-in-programming-style](https://github.com/crista/exercises-in-programming-style).

## List of Exercises

### Part 1 - Historical

* Good old times
* Go Forth

### Part 2 - Basic Styles

* [Monolithic](src/bin/03-monolith.md) - One big program with no named abstractions and no clever library usage
* [Cookbook](src/bin/04-cookbook.md) - Procedural code with global state
* [Pipeline](src/bin/05-pipeline.md) - (Impure) Nested Functions
* [Code Golf](src/bin/06-code-golf.md) - Short code by clever std library usage 

### Part 3 - Function Composition

* Infinite Mirror
* Kick Forward
* [The One](src/bin/09-the-one.md) - Kind of a Monad, Functional composition


### Part 4 - Objects and Object Interaction

* [Things](src/bin/10-things.md) - Object-oriented 
* Letterbox
* Closed Maps
* Abstract Things
* Hollywood
* Bulletin Board

### Part 5 - Reflection and Metaprogramming

* Introspective
* Reflective
* Aspects
* Plugins

### Part 6 - Adversity 

* Constructivist
* Tantrum
* Passive Aggressive
* Declared Intentions
* Quarantine

### Part 7 - Data-Centric

* Persistent Table
* Spreadsheet
* Lazy Rivers

### Part 8 - Concurrency

* Actors
* Dataspaces
* Map Reduce
* Double Map Reduce

### Part 9 - Interactivity

* Trinity
* Restful

### Part 10 - Rusty

* [Rusty](src/bin/42-rusty.md) - Idiomatic efficient Rust solution




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
