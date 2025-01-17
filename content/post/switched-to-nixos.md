---
title: I switched to NixOS from Arch Linux and I'm never going back
date: 2024-10-02
readTime: true
---

10 months ago I tried Linux for the first time. First it was Ubuntu, then I switched to Debian, then Arch Linux.

<!--more-->

I tested them all for a few months and was mostly happy with my Arch Linux for about 6 months. But then I tried NixOS...

## First Encounter with NixOS

That was until 1 week ago where I decided "everyone seems to be talking about nix, idk what that is, I won't do any research, just going to boot it on my computer and see what happens"

For about a whole day I was transferring my [dotfiles configuration](https://github.com/nikitarevenco/dotfiles) to NixOS. First I started doing it the primordial way, with configuration.nix and zero flakes.

### Initial Resistance to Flakes

I didn't want to use flakes because I didn't see a point, I just saw it as extra added complexity. After a few days I was really done tinkering with my setup and things worked fine now.

### A Comparison to Arch Linux

My other laptop had arch on it, and after an update I couldn't boot into my system, so I decided to try install NixOS on it, again

Wow, the entire installation literally took about 10 minutes. I didn't have to follow the Arch wiki for the 10th time then running a bunch of symlink commands and editing random /etc files and running random commands to get my system working.

It was just as simple as git clone, `sudo nixos-rebuild switch`. I'm impressed, seriously impressed.

## Switching to Helix Editor

During the setup, I decided to ditch my bloated Neovim configuration which I've kept adding on and on to and it was becoming ugly.

I wanted something that has everything built-in, is lightweight, fast, and made with Rust.

So I decided to use Helix editor instead, because i just grew tired of having to manage my LSPs with Neovim. Helix is awesome at what it does, and I was mostly happy with it.

### Hard Times

But I was about to switch back, because the only thing Helix is missing is a way to properly navigate files. However, while looking at the pull requests for Helix I noticed that it has a [pull request](https://github.com/helix-editor/helix/pull/11285) which adds a basic file navigator that really covers all my needs.

So I was excited. I cloned, checked out and built it. But it was painful. Helix binary wasn't aware of the runtime directory that it usually knows about, which contains tools for syntax highlighting (treesitter) and it didn't recognize my configuration either. There were a few other pull requests I really wanted to use, but it started feeling like it was too much pain.

### Discovering the Power of Flakes

Then I saw that they have a flake.nix file, so I researched flakes. I previously didn't want to use flakes because I didn't see the point, I have all my packages from nixpkgs. Why should I use them?

But, as it turns out -- flakes make the process I described above _comically easy_. I converted my setup to flakes, and then I pointed one of the inputs at the helix repository. And it built from `master`. I was astonished, because I could see the potential that this can have...

I cloned helix repo, added the pull requests I wanted merged _now_ rather having to wait a few months for the next release. Merged them into my own repo, then pointed my helix input at my own helix fork on my personal branch rather than helix/main. Holy crap! It successfully built the whole thing, and now I have a file browser. It took like 2 minutes.

Doing this kind of thing on any other linux distribution would be far more painful. I made a post here one week ago saying that I think Nix is the best distribution, but now I think that 10x.

I was changed as a person and I'll never be the same again.
