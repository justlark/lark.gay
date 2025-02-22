+++
title = "Switching to Jujutsu: A git-compatible VCS"
description = "I've been curious about non-git version control systems for a while, but escaping git's ubiquity is hard. Jujutsu is a new VCS that's repo-compatible with git."
date = 2025-02-22

[taxonomies]
tags = ["tech"]

[extra]
uuid = "c11df468-1dda-4cec-a4a5-fb93c34f9458"
+++

I've been curious about non-git version control systems like
[Mercurial](https://www.mercurial-scm.org/),
[Fossil](https://fossil-scm.org/home/doc/trunk/www/index.wiki), and
[Pijul](https://pijul.org/) for a while, but switching away from git is
difficult given its ubiquity, particularly considering the other VCSes have next
to no support among the major hosting platforms.

However, a friend recently clued me in to a relatively new VCS that's
repo-compatible with git. It's called [Jujutsu](https://github.com/jj-vcs/jj).
You can use Jujutsu with git forges like GitHub, and you can collaborate with
people using git--without them even knowing!

After playing around with it for a few weeks, it's almost completely supplanted
git for me; I don't foresee myself going back. The CLI is much more sensible--a
famous shortcoming of git--but it's more than just another git frontend. Its
data model makes rewriting history, rebasing, and conflict resolution *much*
less painful. It took some adjustment and unlearning of git paradigms, but after
just a few weeks I'm already more confident navigating complex situations in
Jujutsu than I am after 10 years of working with git.

My one gripe with Jujutsu is the lack of git LFS support; using Jujutsu with
LFS-enabled repos will implicitly check in those files, which is annoying.

A friend and I collaborated to write a [quick reference and cheat
sheet](https://justinpombrio.net/src/jj-cheat-sheet.pdf) for Jujutsu. You can
find his blog post
[here](https://justinpombrio.net/2025/02/11/jj-cheat-sheet.html).

I highly recommend you give it a try.
