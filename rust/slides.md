# Rust
Welcome to my presentation!
Today's discussion will include:
* What is systems programming?
* What is memory safety?
* What is Rust?
* How can Rust help me write better code?
* What are some of Rust's cool features?

---
# About this presentation
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

---
# Memory Leaks
* Variables are stored either on the stack or the heap.
* Limitations of the stack:
    * Memory in the stack is local to the function.
    * Hard to access variables outside of the function we're in.
    * Passing variables in means copying the whole contents.
* Limitations of the heap:
    * Memory on the heap can be requested (allocated).
    * Storing one copy of a large structure on the heap is easy.
    * All heap memory must be returned (freed).
    * If we don't free the memory we use, that's a memory leak.

---
# C: Manual Memory Management
* You allocate memory on the heap explicitly.
    * OS expects you need to return it.
* Use `malloc()` to allocate.
* Use `free()` to free.

---
# Manual Memory Management Example
```
~~~cat manual.c
* Easy, right?
~~~
```

---
# I lied about C
* Admittedly, that last example is a bit contrived.
* A function that returns a newly allocated array is a code smell.
* I just needed a way to show the syntax.
* So let's look at a real world example.
* Open your manual to section 3, page `regex`.

---
# C Redux (Cdux)
```cpp
#include <stdio.h>
#include <regex.h>

int main(int argc, char *argv[]) {
    
}

```

---
# Manual Memory Management: Pros and Cons
* Incredible speed.
* Gives the programmer the most control.
* But, don't forget a single `free()`
* Programmer does all the work.
* Do you really trust yourself to remember that?

---
# Garbage Collection
* All structures are stored on the heap.
* The runtime handles allocation and freeing.
* Counts how many places still need each piece of data.
* Once the data is unused, deallocates for you.

---
```java
public class Gc {
    public static void main(String[] args) {
        int[] numbers = new int[15];

        numbers[0] = 1;
        numbers[1] = 1;

        for (int i = 2; i < numbers.length; i++) {
            numbers[i] = numbers[i-1] + numbers[i-2];
        }

        for (int i = 0; i < numbers.length; i++) {
            System.out.println(numbers[i]);
        }
    }
}
```
* See, much easier!

---
# Garbage Collection: Pros and Cons

## Pros
* Very easy on the programmer.
* Prevents memory leaks.

## Cons
* Sloooooooooooow.
* Tons of overhead.

---
# A Third Method?
* In most cases, we do not need GC.
* But we also don't want to go full manual.

---
# RAII
* Find lifetime of every variable at compile time.
* Compiler fills in the blanks.
* Memory safety with less overhead than GC.
* Popularized by C++.
$nvim raii.cpp
$./raiicpp
* Look, no `free()`!
$nvim raii.rs
$./raii
* Looks high-level, actually very performant.

---
# Null Pointers
* Null pointers are a massive pain point.
> I call it my billion-dollar mistake. It was the invention of the null
> reference. At that time, I was designing the first comprehensive type system
> for references in an object oriented language. My goal was to ensure that all
> use of references should be absolutely safe... but I couldn't resist the
> temptation to put in a null reference. This has led to innumerable errors,
> vulnerabilities, and system crashes, which have probably caused a billion
> dollars of pain and damage in the last forty years.
> - Sir Tony Hoare

---
# Null Pointers (cont.)
* The answer turns out to be pretty easy.
* Just have references that can't be null!
* Rust has no nullable references.

---
# Data Races
* Imagine the following:
    * Resource R is initialized.
    * Thread A gets reference to R.
    * Thread B also gets a reference.
    * Thread A begins an operation on R.
    * Thread B requests information from R.
* This can lead to unexpected results.
* Happens in single-threaded programs as well.

---
# Data Race Mitigation
* Mutexes help for multi-threaded.
* C++ invalidates iterators.
* C makes you figure it out by yourself.
* Rust's solution: Ownership.

---
# Ownership
* Every resource is owned by one function.
* When you pass resource R in:
    * Ownership is transferred to callee.
    * Caller cannot access R after call.
* When you pass reference to R:
    * Ownership is not transferred.
    * Callee borrows R.
    * Caller can access R after call.

---
# Ownership in Action
$nvim move.rs
$rustc move.rs
* This code doesn't compile
$nvim borrow.rs
$./borrow
* This code works!

---
# Really Cool Features
* Iterators:
    * Performant functional programming.
* Reference-counted types
    * Do borrow checking at runtime.
* Enum types
    * May encapsulate one of several types.
    * Keeps a flag for what types exist.
* Option<T>
    * Type that can be Some or None
    * Safe version of nullable references.
* Result<T, E>
    * Rust's way to handle errors.
    * Functions that may have an error return Result.
    * Result is either Ok(T) or Err(E).
    * Handle things differently depending on return.
* Cargo
    * Rust's build system.
    * Automatically grab packages from crates.io.
    * Easy to set up a project.
    * Comes with a testing framework.
* LSP support
    * Write Rust in your favorite editor with completion.
* And more!
