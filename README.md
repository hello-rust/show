 ![Hello Rust Show logo](logo.png)

[🏠 Homepage](https://hello-rust.show) | [▶️ YouTube](https://www.youtube.com/hellorust) | [❤️ Patreon](https://www.patreon.com/hellorust) | [💭 Twitter](https://twitter.com/hellorustshow) | [📝 Show notes](https://github.com/hello-rust/show/tree/master/episode)

This is the main repository of *Hello Rust*, a live-coding show by [Matthias
Endler](@mre) about the Rust programming language (https://www.youtube.com/hellorust).  
It is targeted towards intermediate Rust programmers, who have already read [the book](https://doc.rust-lang.org/book/) and want to learn advanced patterns and tricks as well as how to write ergonomic code in Rust.

In this repository, you find the list of previous episodes, the planned shows and the show notes.

### How to support me

As you know, producing content takes a lot of time and effort. On top of that, running a YouTube channel requires getting a lot of expensive hardware (like recording and editing equipment) to achieve somewhat acceptable quality.
This show is free for everybody to watch. If you want it to stay this way, consider donating.  
[**Become a patron**](https://www.patreon.com/bePatron?c=1568097) and earn a special place in my heart &mdash; forever. ❤️  

### List of episodes

* [#0: **Humble Beginnings**](/episode/0) - An introduction on what this show is about. [[Video](https://youtu.be/jMJRTjnh_jo)]
* [#1: **Hello Universe**](/episode/1) - default trait, debug trait, builder pattern [[Video](https://youtu.be/STWuPMcwwbw)]
* [#2: **Snakes And Gears**](/episode/2) - iterators, map, filter, EntryAPI, List comprehensions [[Video](https://youtu.be/bS5rtxWd2yQ)]
* [#3: **A Code Review**](/episode/3) - Option, Result, Error handling, URL parsing, external crates [[Video](https://youtu.be/a6KWRvAPsmo)]
* [#4: **Touch Typing Tutor**](/episode/4) - touch typing, application, ggez, event handling, game state, iterators, hacking, live-coding [[Video](https://youtu.be/S0Vubd-C5-o)]
* [#5: **Coding Challenge - Balanced Brackets**](/episode/5) - Into trait, Pattern matching, HashMap, Stack, unreachable! macro, coding puzzle, competitive programming [[Video](https://youtu.be/XcuLHO8z_RA)]
* [#6: **Parameterized Tests, Macros, And Refactoring**](/episode/6) - Test data providers, Parameterized tests, AsRef trait, Macros [[Video](https://youtu.be/XJPci7GI-qg)]
* [#7: **Parsing Dates Using Proptest And Tdd**](/episode/7) - Property testing, Quickcheck, Unit testing, Fuzzy testing, TDD [[Video](https://youtu.be/zb7SD0Jco6g)]
* [#8: **Let'S Write A Python Module!**](/episode/8) - Tutorial, FFI, pyo3, Module, Extension, Python [[Video](https://youtu.be/D9r__qxtRMQ)]
* [#9: **Go Vs Rust - Concurrency And Race Conditions**](/episode/9) - race-conditions, ownership, mutex, concurrency, rayon, golang, rustlang [[Video](https://youtu.be/B5xYBrxVSiE)]

If you have an idea for a future show, [don't hesitate to create a new issue or upvote an exiting one](/issues).

### Inspiration

These YouTube channels inspired me to make this. I highly appreciate the work that went into these channels.

* [Ferris Streams Stuff](https://www.youtube.com/channel/UC4mpLlHn0FOekNg05yCnkzQ)
* [Fun Fun Function](https://www.youtube.com/channel/UCO1cgjhGzsSYb1rsB4bFe4Q)
* [Just for func](https://github.com/campoy/justforfunc)

### FAQ

**Q: What development environment do you use?**    
**A:** Right now, I use *VSCode* and the *Rust* plugin plus the *Rust Language Server*.  
I've heard good things about the IntelliJ Rust plugin, though.  
Therefore, I might try this setup in the future.  

**Q: What audio/video equipment do you use?**
**A:** Video: Canon 700D. Audio: Rhode NT USB, Takstar SGC 598.

**Q: What is your post-processing routine?**    
**A:** Quite simple. I solely use ~~iMovie~~ (Davinci Resolve) right now. There are better programs out there, but so far it does the job. I'm mostly annoyed by the long rendering times on my MacBook. (5 hours for 30 minutes of video)

**Q: What is the name of your color theme?**  
**A:** Usually I use 1337, but that can vary per show. I will try to mention it in the show notes, if it's something else.

**Q: What font are you using?**  
**A:** Operator Sans Mono

**Q: How long does it take to create one episode?**  
**A:** Around 30 hours. Here's a breakdown of the process:
* Preparation time: 5 hours
* Creating a prototype for the code I'd like to show: 2-5 hours
* Prepare office for recording: 30min
* Recording: 2-4 hours
* Editing: 12-16 hours
* Publication: 2 hours

My hope is to get faster with more practice. Especially the editing is still
painful, although I've gotten much better already. Publication could also mostly
be done automatically and I wrote some tooling to help me with that (see `ci`
folder).

### Credits

* Gears from the show intro [designed by Freepik](http://www.freepik.com).
* Ornament from the show intro [designed by Freepik](http://www.freepik.com).
