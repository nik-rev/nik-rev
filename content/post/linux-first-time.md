---
title: I was forced to use Linux. But now I love it.
date: 2024-08-21
---

In December of 2023 I wanted to learn Web development, for fun. I was 6 months away from my A-Level exams and I was really bored.

I found this website called [The Odin Project](https://www.theodinproject.com/), which is a free and open-source curriculum for learning web development. Perfect!

# Forced into Linux

Unfortunately, one of the requirements for the course was to use Linux. Odin Project didn't support Windows, only Mac and Linux at the time. I didn't have a Mac, so...

It was an all-or-nothing situation. Either I use Linux, or I can't complete this course. Mind you, I had absolutely zero idea what a Linux was. I've only ever been using Windows before.

But despite that fact, my curiosity got the best of me and I decided to try it out.

Having zero idea that it will change my life, I followed their installation instructions for Ubuntu. It took me an entire day to install it, but I finally did.

# Descent into Madness

Once I did, I opened the file manager and was terrified -- I had no idea what was going on. Where are all the Program Files? Why is everything empty?

Initially I thought that on Linux you can't access any of the "system files". I thought to myself, hey -- I'll just complete this course and then go back to my familiar Windows.

The Odin Project also had several lessons on the command line, such as creating new files, changing directories, etc. The basics.

It was extremely confusing to me, because I had no idea what was happening. But it was also super interesting and I wanted to learn more.

## Researching Commands

I spent several days researching various Linux commands. Odin Project said that Linux is used everywhere in development, that's why I continued to go through with it.

But to be honest, I didn't really see a point in it. I thought that typing out all of these commands is a waste of time. That was just in the beginning.

As I continued to use Linux full-time as my computer for several weeks, I slowly became more accustomed to it. I found myself more efficient in the command line than with their file explorer.

## Becoming more Efficient

And it all started from there. I found out that I can make aliases in bash, so I aliased `e` to `ls`, and `t` to `cd` and then `ls` in the new directory.

Worked like a charm, it was very much an improvement in efficiency. I found myself always using `ls` when `cd` so might as well combine them.

I started tinkering with my system more and more over time, learning about various configuration options and programs like `cat`.

At some point I needed to create a bunch of files with a number at the end, for example like this:

```sh
touch file1 file2 file3 file4 file5 ... file67
```

Typing out the above is actually so much faster than if I were to use a GUI like a file manager to do the same thing.

But better yet, I dicovered there was a way to do that much more concisely:

```sh
touch file{1..67}
```

And oh wow. That just worked. It worked like a charm. I saved so much time from this. And from there on, I started learning more about Unix commands.

I learned how to use `grep` to search for strings fast, and `find` to dig through the filesystem in order to find a file matching a pattern, which turned out _very_ useful for adjusting to unfamiliar codebases.