---
author: Richard Snider (Trongle)
---
# Crab Trap: An Introduction to Rust
Welcome to my presentation!
Today's discussion will include:
* What is systems programming?
* What is memory safety?
* What is Rust?
* How can Rust help me write better code?
* What are some of Rust's cool features?

---
# About This Presentation
* Oh, and this presentation is open source!
* So is the software that I'm using to present.
* So are my other presentations for this club.
* Slideshow: https://github.com/CodeTriangle/fslc-presentations
* Software: https://github.com/maaslalani/slides

---
# Systems Programming
* Systems programming works with the OS.
* Needs to be performant.
* Languages like C and C++.
* Generally harder to work with than interpreted languages.
    * Take more planning to do the same task.
    * Require more knowledge of memory.
* But, very fast compared to other languages.

---
# Rust
* Rust is a new(er) systems programming language.
* Created in 2010 by employees at Mozilla Research.
* Notably used in Firefox Servo browser.
* "Most Loved" language for the past six years.
* Focus on "safety".
    * All variables are immutable by default.
    * Rust provides memory safety.
    * Rust has atomic types for safe concurrency.

---
# Why is Rust ~~Better~~ Different?
* Programming is fundamentally annoying.
* We are very good at finding ways to make it less annoying.
* But there are still some things that are just hard to do.
* How does Rust help programmers do these things?
* We'll discuss a few of them:
    * Memory management
    * Null pointers
    * Data races
    * Concurrency
    * Error Handling

---
# Memory Management
* In a program, data are stored either on the stack or the heap.
* Limitations of the stack:
    * Memory in the stack is local to the function.
    * Hard to access variables outside of the function we're in.
    * Passing variables in means copying the whole contents.
* Limitations of the heap:
    * Memory on the heap can be requested (allocated).
    * All heap memory must be returned (freed).
* Some common errors with freeing memory:
    * **Memory Leak**: used memory is never freed.
    * **Dangling Pointers**: memory is freed while references still exist.
    * **Double free**: memory is freed twice.

---
# Manual Memory Management
* Programmer allocates memory on the heap explicitly.
    * OS expects you to return it.
* C is the canonical example:
    * Use `malloc()` to allocate.
    * Use `free()` to free.
    * Easy, right?

---
# Manual Memory Management: Example

```cpp
~~~xargs cat
manual.c
~~~
```

---
# Manual Memory Management: I lied about C
* Admittedly, that last example is a bit contrived.
* A function that returns a newly allocated array is a code smell.
* I just needed a way to show the syntax.
* So let's look at an example that you might actually encounter.
* Open your manual to section 3, page `regex`.

---
# Manual Memory Management: C Redux (Cdux)
```cpp
~~~xargs cat
regex.c
~~~
```

---
# Manual Memory Management: Pros and Cons

## Pros
* Incredible speed.
* Gives the programmer the most control.

## Cons
* Don't forget a single `free()`
* Programmer does all the work.
* Do you really trust yourself to remember that?
* Susceptible to **all** of the previously-discussed errors.

---
# Garbage Collection
* All structures are stored on the heap.
* The runtime handles allocation and freeing for us.
* The garbage collector runs intermittently.
    * Checks if a bit of memory is still being used.
    * If not, frees it.

---
# Garbage Collection: Example
```java
~~~xargs cat
Gc.java
~~~
```
* See, much easier!

---
# Garbage Collection: Pros and Cons

## Pros
* Very easy on the programmer.
* Prevents all the memory errors previously discussed.

## Cons
* Sloooooooooooow.
* Adds tons of overhead:
    * Can add up to be a significant portion of runtime.
* Can be completely unnecessary for some tasks.

---
# Garbage Collection: A Defense Thereof
* I give GC a hard time, but it's not all bad.
* It truly is effortless.
* There's a reason that garbage collected languages are:
    * often recommended as first languages.
    * used to create a prototype of an idea.
    * great for scripting.
* For a lot of use cases, the overhead is justified.

## However
* Sometimes GC cannot be tolerated.
* Questioning preconcieved notions is how we move forward.

---
# RAII
* Stands for "Resource Acquisiton is Initialization"
* A newer pattern, popularized by C++.
* No manual memory management, but no additional overhead.

## How It Works
* Objects have a constructor **and** a destructor.
    * Constructor may allocate memory, managed by the object.
    * Destructor `free()`s all memory allocated by constructor.
* Compiler finds "lifetime" of structures at compile time.
* When the structure goes out of scope, the destructor is run.
* Combats memory problems previously mentioned.
* Looks and feels high-level, actually very performant.

---
# RAII: Example in C++
```cpp
~~~xargs cat
raii.cpp
~~~
```
* Look, no `free()`!
* Sorry it looks bad, it's C++.

---
# RAII: Example in Rust
```rust
~~~xargs cat
raii.rs
~~~
```
* This is our first rust code, so let's look into it a bit.

---
# Ownership
* RAII in Rust is tightly coupled to another concept: Ownership.
    * C++ also kind of has it, but it's complicated.
* Every resource is owned by one function.
    * Destructor is called when resource goes out of scope where it is owned.
* When you pass resource R in:
    * Ownership is transferred to callee.
    * Caller cannot access R after call.
    * This prevents data races as well.
* When you pass reference to R:
    * Ownership is not transferred.
    * Callee "borrows" R while it is running.
    * Caller never loses ownership of R and can access R after call.

---
# Ownership: Example
```rust
~~~xargs cat
borrow.rs
~~~
```

---
# Nullity
* Null pointers are a massive pain.
* A common pattern is to return a null pointer or reference on failed operation.
* This is an extremely easy step to forget.
* According to Sir Tony Hoare, credited with the creation of ALGOL's type system:
> [The null reference] has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.
* The answer turns out to be pretty easy.
* Just have references that can't be null!
* Rust has no nullable references.

---
# Nullity: How Rust Fixes The Billion Dollar Mistake
* The `Option<T>` type.
* `Option<T>` stores either:
    * `Some(T)`: the result (stored as the argument of T) exists.
    * `None`: the result doesn't exist.
* Optional types exist elsewhere, but Rust emphasizes them.

```rust
~~~xargs cat
nullity.rs
~~~
```

---
# Error Handling
* No one likes error handling.
* C is possibly the worst.
    * Errors are denoted by return values only.
    * Return values can often be non-obvious.
    * They're also very easy to ignore, which is bad.
* `try`-`catch` sucks too.
    * Sorry to say, but this is not a good pattern.
    * Encourages overly broad `try` blocks that handle too much.

## The Rusty Way
* Rust's philosophy is as follows:
    * Each function that **can** fail should be checked individually.
    * If the function errors, we shouldn't have access to a returned object.

---
# Error Handling: Example
* The `Result<T, E>` type, similar to `Option<T>`, is Rust's solution.
* `Result<T, E>` stores either:
    * `Ok(T)`: the call succeeded and the return value is contained within.
    * `Err(E)`: the call failed and the error is contained within.

```rust
~~~xargs cat
errors.rs
~~~
```

---
# Cool Features
* Now is my favorite part of the presentation.
* Here I just talk about more things Rust does really well.
* Strap in, this may go a bit quick.

---
# Cool Features: Pattern Matching
* Pattern matching is a newer concept.
* Deconstruct a type, matching aspects of its structure.

```rust
~~~xargs cat
patmat.rs
~~~
```

---
# Cool Features: Iterators
* Iterators are not a new concept.
* Elegant way to traverse ordered data.
* In Rust, iterators enable performant functional programming.

```rust
~~~xargs cat
iter.rs
~~~
```

---
# Really Cool Features
* Reference-counted types
    * Do borrow checking at runtime.
* Enum types
    * May encapsulate one of several types.
    * Keeps a flag for what types exist.
* Cargo
    * Rust's build system.
    * Automatically grab packages from crates.io.
    * Easy to set up a project.
    * Comes with a testing framework.
* LSP support
    * Write Rust in your favorite editor with completion.
* And more!
