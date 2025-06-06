---
title: Quest to understand the idea of "Everything is a file" in Linux
---

I had been using Windows for most of my life. I didn't know much about files -- I thought that files are just what programs use to store their data, such as Notepad using `.txt` extension and Photoshop with its `.psd`.

<!--more-->

I assumed that a file's purpose is _defined by its extension_. And this conclusion made sense to me at the time, since for example I couldn't open a `.mp4` file format in Photoshop.

## The Experiment

When I started using Linux, I made an experiment. For example if I have just a single JavaScript `file.js` like this:

```js
console.log("Hello World!");
```

Then running it with `node file.js` would output `Hello World!` in the terminal. But what was surprising to me, was that when I tried to remove the extension completely:

```
console.log("Hello World!");
```

Running `node file` would still output the same string. So this was insightful to me, at this point I realised that the purpose of files is **not** defined by their extension -- rather, its the _program itself_ that decides how to read a specific file.

It seems that file extensions are only for semantic purposes. If I open a `file.js` in an IDE like [Visual Studio Code](https://code.visualstudio.com/), it'll know which highlighter to use, and that for example certain keywords like `import{:js}`, `const{:js}` should be highlighted. But the file extension (`.js`) isn't strictly required.

### Partitions

Okay, so that's interesting. But when I switched from Ubuntu to Arch Linux, I had to manually set up my partitions during the installation. For example a Windows `C:\` drive may be the equivalent to `/dev/sda` in Linux. And this `/dev/sda` may be divided into separate chunks called "partitions", for example if we were to divide it into 3 partitions, we would have `/dev/sda1`, `/dev/sda2` and `/dev/sda3`.

Each of these partitions like `/dev/sda2` holds _very important information_, such as the file system to use (e.g. [`ext4`](https://en.wikipedia.org/wiki/Ext4)) and **all of your data**.

And at this point, I took notice at how `/dev/sda2` and others look like file paths. So I went exploring to see if that's actually the case...

```sh
cd /dev

ls
```

There were a bunch of files listed, and to my surprise, I also noticed the three of my partitions, **_as files!_**

```
sda1
sda2
sda3
```

I tried opening it with `nvim sda2`, thinking that I'll see a bunch of random data. But actually, it's empty?!

```sda2

```

This was certainly odd, but I just guess the actual information is not being stored as text content and rather is embedded within the metadata, or something like that. Similar to how files don't contain the time they were modified as plain text, etc.

### Everything is a File

I was amazed to find out that so many things in Linux are files. I searched online and found out that it's in the unix philosophy that ["everything is a file"](https://en.wikipedia.org/wiki/Everything_is_a_file), but I didn't _really_ believe it at first.

If that was true, if everything was a file -- then to run those files we would need some sort of interpreter that can read files. But how would we have this interpreter, when the interpreter itself would have to be a file, so it would have to be an interpreter and...? This really sounds like a chicken-and-egg problem.

So this didn't really make much sense to me, I figured that the only way this could work is if we had a chain of interpreters which grow increasingly more complex. The 1st interpreter would have to be so simple that it can be directly read by the CPU, on "raw metal", then we would gradually build up to something that can understand C to boot the kernel.

## Research

As it turns out, files are simply abstractions of how data can be represented. In order for data to be stored, there has to be some sort of medium where two distinct states can be _read_ and _manipulated_ at will. For example:

- [Hard Disks](https://simple.wikipedia.org/wiki/Hard_disk) polarize each region with either north or south magnetization.
- [CDs](https://simple.wikipedia.org/wiki/Compact_disc) use lasers that point at a surface, and the laser will either scatter or be reflected back.
- [SSDs](https://simple.wikipedia.org/wiki/Solid-state_drive) store and release electrons from a tiny chamber.
- Even DNA can store data by numbers to combinations of [nucleotide bases](https://simple.wikipedia.org/wiki/DNA#Nucleotides).

### File Systems

Storing data is cool and all, but storing is in a way that makes sense is equally important. This is where [filesystems](https://simple.wikipedia.org/wiki/File_system) come in to play.

Filesystems use bits of the storage media to hold data that doesn't belong to a specific file, but rather is there to organize them. Tables of where files start and end, metadata of files such as their names, permissions, etc.

At this point, the Operating System (<abbr>OS</abbr>) reads all that information and abstracts it away into an easy-to-digest format as folders and icons, it presents everything as if it was a file. Some things we see on the filesystems aren't files, they can be devices, network interfaces, etc. With this approach, we can access them by the same means as we would with an actual file.

### Bootstrapping

When the computer is first booted, there are no files. The CPU has no idea what a file is, it can only read/write data from RAM and execute primitive instructions like adding two numbers and branching -- jumping from one place to another based on some condition.

Once the CPU turns on it will run the instructions in some memory address, which contains the firmware (for example, [UEFI](https://simple.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface)).

The firmware instructs the CPU to load data from a given partition (like `/dev/sda2`), which is where our OS is stored.

At this point, Linux will copy the contents of a file which is the Initial RAM FileSystem ([`initramfs`](https://en.wikipedia.org/wiki/Initial_ramdisk)) -- that contains an entire disk image of a bare-bones primitive OS. This "primitive" OS is capable of stuff like reading filesystems and executing programs, but not much more.

We use this basic OS to load our actual system that we've installed on our disk. When our system is loaded, the primitive OS we first loaded gives up its control to our actual system and then _unloads itself from RAM_.

So we basically have:

1. Computer turns on.
1. Firmware is turned on.
1. Data from boot partition is loaded and executed.
1. Primitive OS that can only perform basic tasks is loaded.
1. Our actual system is loaded by the primitive OS.

Of course, we are really simplifying here -- but it's nice to get a high level overview before [delving deeper](https://en.wikipedia.org/wiki/Booting_process_of_Linux) if you are interested.

### Opening Binary Files

When I tried to open a binary file like `/bin/ls` (which, yes, even a command like `ls` is represented as a file) -- all I would see is a bunch of gibberish:

```/bin/ls showLineNumbers
ELF          >    0Q      @       À         @ 
  @       @       @       Ø 
                       
   1I     1I            
               h                 h 
```

I was honestly expecting to see something like `001100110` -- the raw binary data. But I don't... why is that?

Well, text editors may read each byte and translate them using [ASCII](https://en.wikipedia.org/wiki/ASCII) into letters. But an image viewer may read groups of three bytes and convert each into a number between 0 and 255 indicating how much <span style="color: var(--red)">Red</span>, <span style="color: var(--green)">Green</span> and <span style="color: var(--blue)">Blue</span> for each pixel.

As we've discovered earlier, purpose of files is defined by the reader, not the file extension! Let's say we had a pure binary file that contains the following binary data:

```
01001000 01100101 01101100 01101100
01101111 00100000 01010111 01101111
01110010 01101100 01100100
```

If we were to convert this to ascii, we would get `Hello World`, but if we were to interpret it as an image, we'd convert this to hex first:

```
Row 1: [72, 101, 108, 108]
Row 2: [111, 32,  87, 111]
Row 3: [114, 108, 100, 0]
```

Converting it to an image where each each number represents the lightness value gives us:

<div
  style="display: flex; flex-direction: column; align-items: center;"
  role="img"
  aria-label="hex values converted to a 4x3 image with each pixel representing lightness value from 0 to 255"
>
  <div style="display: flex;">
    <span style="width: 16px; height: 16px; background-color: rgb(72, 72, 72);"></span>
    <span style="width: 16px; height: 16px; background-color: rgb(101, 101, 101);"></span>
    <span style="width: 16px; height: 16px; background-color: rgb(108, 108, 108);"></span>
    <span style="width: 16px; height: 16px; background-color: rgb(108, 108, 108);"></span>
  </div>
  <div style="display: flex;">
    <span style="width: 16px; height: 16px; background-color: rgb(111, 111, 111);"></span>
    <span style="width: 16px; height: 16px; background-color: rgb(32, 32, 32);"></span>
    <span style="width: 16px; height: 16px; background-color: rgb(87, 87, 87);"></span>
    <span style="width: 16px; height: 16px; background-color: rgb(111, 111, 111);"></span>
  </div>
  <div style="display: flex;">
    <span style="width: 16px; height: 16px; background-color: rgb(114, 114, 114);"></span>
    <span style="width: 16px; height: 16px; background-color: rgb(108, 108, 108);"></span>
    <span style="width: 16px; height: 16px; background-color: rgb(100, 100, 100);"></span>
    <span style="width: 16px; height: 16px; background-color: rgb(0, 0, 0);"></span>
  </div>
</div>

Interestingly enough though, we _can_ actually see the raw bits of files with a tool like [`xxd`](https://manpages.org/xxd)! If we open up the `ls` command with `xxd /bin/ls` we will get over 8000+ lines of code, with some of it even marking sense! _maybe_...

```/bin/ls
00021250: 0000 0000 0000 0000 0000  ................
00021260: 0000 0000 0200 0000 0000  ........@4......
00021270: 0200 0000 433a 2028 474e  `"......GCC: (GN
00021280: 2031 342e 3230 3234 3038  U) 14.2.1 202408
00021290: 0000 6c73 7567 0000 0000  05..ls.debug....
000212a0: 8112 002e 7274 6162 002e  .z....shstrtab..
000212b0: 7465 7270 7465 2e67 6e75  interp..note.gnu
000212c0: 726f 7065 2e6e 6f74 652e  .property..note.
```

### Magic Numbers

While we found out that file extensions aren't really important, I also learned that many programs will use the first couple of bytes within a file to determine the file type and decide e.g. which highlighting engine to use.

Here's just some of those magic numbers and their corresponding file formats that I got from the [list of file signatures](https://en.wikipedia.org/wiki/List_of_file_signatures):

| Magic Number              | File Format |
| ------------------------- | ----------- |
| `89 50 4E 47 0D 0A 1A 0A` | `.png`      |
| `66 74 79 70 69 73 6F 6D` | `.mp4`      |
| `25 50 44 46 2D`          | `.pdf`      |

### References

- [Magic Numbers](https://en.wikipedia.org/wiki/File_format#Magic_number)
- [List of Magic Number File Signatures](https://en.wikipedia.org/wiki/List_of_file_signatures)
- [Everything is a File](https://en.wikipedia.org/wiki/Everything_is_a_file)
