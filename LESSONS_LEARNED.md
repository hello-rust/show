### Lessons learned the hard way

- Always double check the final rendered video for hickups before uploading.
- Check the final upload as well. The [video speed controller chrome
  plugin](https://chrome.google.com/webstore/detail/video-speed-controller/nffaoalbilbmmfgbnbgppjihopabppdk)
  helps a lot with screening.
- Video assets are huuuge. A typical episode takes hundreds of gigabytes of
  render files. To edit efficiently, I have to squeeze out every byte from my
  tiny disk. In the long run, I will have to invest in better hardware, which I
  haven't done in a while because Computers are reasonably fast for most task
  today - not so for recording.
- After a video is shot, you can not change the narrative anymore. You'll wake
  up in the morning and think: "oh my God, I totally forgot THAT". But the only
  you can do is add a comment or cut stuff, not add things to the recording.
  Therefore, a script is vital in which I keep track of the things I absolutely
  have to say during the episode, but no word-by-word instructions.
- Every programming mistake during the recording is very expensive. In post, you
  have to decide whether to cut it out or leave it in there. Cutting it out can
  be tricky, as you might only discover your mistake minutes later and leaving
  it in might confuse or even frustrate your viewers ("why do you waste my
  time???").
- If I have a long block of uncut material, it's probably not because there are
  no hickups, but because I forgot to edit that.
- Never let the recording equipment get in the way of your ideas. In other
  words: shoot with what you have. I wanted a second camera but didn't want to
  spend a lot of money on it. So I just use an old webcam for now. The quality
  might not be perfect, but it's still better than not having it. I'm also
  shooting alone and sometimes I would love to have somebody zoom into a scene
  when I explain things. For now I'm simply emulating this effect in my Video
  editor using crops or a Ken Burns effect. This will dilute the recording
  quality a bit, but it adds so much depth and liveness.

### Things I did right

- Adding features as I go: for example I had to manually format the show notes.
  Seeing that this took so much time I wrote a small CLI tool to render it from
  a yaml file. Over time I added more output formats for the same content:
  YouTube, Twitter, the `README` and so on. The important part is that I did
  that piece by piece of many episodes instead of doing lots of work up-front.


### Consistency

- Doing a single episode takes me *ages*. I would never have thought it takes so
  much time. Here's a rough timetable:
  * Topic preparation (2-5 hours)
  * Audio/Video setup (30min-1h)
  * Recording (1h-5h)
  * Writing show notes (30min)
  * Editing (10-20h)
  * Publication (1h).
  * Creating a title image (15min)
  * Updating the website (15min)
  * Sharing on social media (15min)
  * Feedback and questions (15min)

- It's pretty easy to find excuses for skipping an episode: live gets in the
  way, there's more important stuff to do, the content doesn't feel just right
  yet, there was a technical problem while recording,... don't fall into that
  trap. I try to avoid skipping the schedule at all costs. Consistency is king
  and viewers reward it. I treat "Hello Rust!" like a TV show: Even if I know
  that no episode will ever be perfect, I still publish at the scheduled time
  (which can be pretty stressful).

### Resources

- There's a ton of royality free music out there, but it's hard to find because
  it's not properly categorized or the preview is cumbersome. Right now I use
  [freemusicarchive](freemusicarchive.org/), although it's not the most
  convenient to use.


### Simple tips 

- Metadata is quite important to people: one person suggested to add keywords
  to the end of the title to improve discovery on YouTube. That was a great
  idea! People rarely search for the show directly, but instead they find it
  through keywords.
- Similarly, people were frustrated about the flow of the show: some people said
  I was rushing over some topics, others thought that it was dragging along.
  What I did was adding a lot more links to entry-level topics for onboarding
  beginners and adding markers to the videos so that people could jump to the
  interesting bits right away. Similarly, I suggest to watch the videos at 1.5x
  the speed if you feel bored. That's a nice variation to live-coding where
  you're kind of stuck with one pace.

### Finding your voice

- Bootstrapping a YouTube channel reminds me a lot of bootstrapping a band: you
  learn as you go and your first shows are most likely awful. This soul-seeking
  takes a while until you find your "voice" and your audience. 
- It's tempting to arrange the show like your viewers presumably want it to be,
  but it's very important that you stay true to yourself. If you want to revamp
  the show, just do it. Not everybody will like it, but it's your personal
  project and your freetime after all.
- In the beginning, it was hard to let my personality show through. I'm usually
  a very enthusiastic person, especially when talking about technology, but
  in the videos I sometimes look dull. Some people even commented that it was
  "painful to watch". It took me a while to figure out what was the reason. What
  helped me was looking at other people to learn their tricks on how to loosen
  things up.

  #### B-roll

  [Peter McKinnon](https://www.youtube.com/user/petermckinnon24) is a
    charismatic storyteller. That's the level of enthusiasm I'd like to show.
    He's using a ton of b-roll to support his narrative.

  #### Background music

  The background music in [Hot Ones](https://www.youtube.com/user/FirstWeFeast)
  is top-notch. It really creates a thick atmosphere.

  #### Editing

  * The audio and video editing of Masterclass is second to none. Just watch a
    trailer of [Chris Hadfield's masterclass on space
    exploration](https://www.masterclass.com/classes/chris-hadfield-teaches-space-exploration)
    to get a glimpse of that.

  What I learned from them is that music is pretty important (pizzicanto works
  good for my clumsy style), b-roll is quite cool (e.g. showing the website of a
  project while I talk about it), camera movements and zooms also work well.
  Sometimes I wish I had a dedicated camera-person to make that part easier. A
  second camera might be nice to record additional footage.

  I found that not many coding channels think about delivery. Many have
  great content but they fall short on the execution part which makes the videos
  less immersive and entertaining.


### Technique

- Like everything else, video editing is a rabbit hole. From multi-camera setups
  to LUTs, there's sooo much to learn and experiment with. But it's also a lot
  of fun! It's important, that the main focus is always on the content, though.
- [Lighting](https://www.youtube.com/watch?v=eZ5hpcn6tIM) is also a rabbit hole
  and so is audio.

### Gear

- Invest in good gear. If you're serious about this, you will spend a LOT of
  time in front of the screen editing text / cutting videos. It's wise to spend
  some money on a good screen, mouse, keyboard and so on. Unfortunately, this
  ends in a never-ending race to find the perfect gear. And it can get quite
  expensive.
- Rendering is still a time-sink in 2018. Plan in some time for that. I usually
  aim to render the master video over night.
