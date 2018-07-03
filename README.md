 ![Hello Rust Show logo](logo.png)

[▶️ YouTube](https://www.youtube.com/channel/UCZ_EWaQZCZuGGfnuqUoHujw) | [🏠 Homepage](https://hello-rust.show) | [❤️ Patreon](https://www.patreon.com/hellorust) | [💭 Twitter](https://twitter.com/hellorustshow) | [📝 Show notes](https://github.com/hello-rust/show/tree/master/episode)

This is the main repository of *Hello Rust*, a live-coding show by [Matthias Endler](@mre) about the Rust programming language.  
It is targeted towards intermediate Rust programmers, who have already read [the book](https://doc.rust-lang.org/book/) and want to learn advanced patterns and tricks as well as how to write ergonomic code in Rust.

In this repository, you find the list of previous episodes, the planned shows and the show notes.

### How to support me

[**Become a patron**](https://www.patreon.com/bePatron?c=1568097) and earn a special place in my heart &mdash; forever. ❤️  
If you have an idea for a future show, [don't hesitate to create a new issue or upvote an exiting one](/issues).

### List of episodes

* [#0: **Humble Beginnings**](/episode/0) - An introduction on what this show is about. [[Video](https://www.youtube.com/watch?v=jMJRTjnh_jo)]
* [#1: **Hello Universe**](/episode/1) - Builder Pattern, Enums, standard traits. [[Video](https://www.youtube.com/watch?v=STWuPMcwwbw)]
* [#2: **Snakes and Gears**](/episode/2) - Entry API, list compreshensions, iter, filter, map, and reduce. [[Video](https://www.youtube.com/watch?v=bS5rtxWd2yQ)]
* [#3: **A Code Review**](/episode/3) - Option, Result, Error handling, URL parsing, external crates [[Video](https://www.youtube.com/watch?v=a6KWRvAPsmo)]
* [#4: **Touch Typing Tutor**](/episode/4) - touch typing, application, ggez, event handling, game state, iterators, hacking, live-coding [[Video](https://youtu.be/S0Vubd-C5-o)]
* [#5: **Coding Challenge - Balanced Brackets**](/episode/5) - Into trait, Pattern matching, HashMap, Stack, unreachable! macro, coding puzzle, competitive programming [[Video](https://youtu.be/XcuLHO8z_RA)]
* [#6: **Parameterized Tests, Macros, And Refactoring**](/episode/6) - Test data providers, Parameterized tests, AsRef trait, Macros [[Video](https://youtu.be/XJPci7GI-qg)]
* [#7: **Parsing Dates Using Proptest And Tdd**](/episode/7) - Property testing, Quickcheck, Unit testing, Fuzzy testing, TDD [[Video](https://youtu.be/zb7SD0Jco6g)]
* [#8: **Let's Write A Python Module!**](/episode/8) - Tutorial, FFI, pyo3, Module, Extension, Python [[Video](https://youtu.be/D9r__qxtRMQ)]

### Inspiration

These YouTube channels inspired me to make this:

* [Ferris Streams Stuff](https://www.youtube.com/channel/UC4mpLlHn0FOekNg05yCnkzQ)
* [Fun Fun Function](https://www.youtube.com/channel/UCO1cgjhGzsSYb1rsB4bFe4Q)
* [Just for func](https://github.com/campoy/justforfunc)

### Creating an episode

The following notes are helpful for me when recording a new episode.  
Also, I thought it might be interesting to others, who also consider to run their own show.  

#### Getting started

* Pick an issue from the [issues page](https://github.com/hello-rust/show/issues) or create a new one.
* Prefix the issue title with the next show number.

#### Shooting the video

* Cleanup SD card of digital camera
* Prepare lighting
* Set-up microphone
* Prepare primary and secondary camera
* Test recording and audio
* Close all unrelated apps
* Hide Bookmarks Toolbar in Firefox (right-click next to address bar and remove tick)
* Prepare IDE settings
  - Set slightly bigger font (VS Code: `editor.fontSize": 15`)
  - ~~Enter "zen mode"~~ (doesn't show the terminal)
* Set Mac to *do not disturb mode*
* Deactivate night shift
* Start recording and get cracking!

### Shooting the intro

* Create a teaser intro that shows exactly what to expect as an outcome (show some code as b-roll, maybe).

### Postproduction

1. Cut video
2. Transitions
3. Normalize sound
4. Add music and sound effects
5. Color Grading

#### Before publication

* Create nice title images for the video and the website.

### Upload

When uploading the video, I first set it to "unlisted".
That gives me another opportunity to check the quality and the editing of the final cut.
At this phase, I also add show notes to the video description, adjust the title
and add time markers to the video description.

#### Publication

* Upload the code to Github.
* Set YouTube video to "public".
* Announce publication on Twitter and Reddit.

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
