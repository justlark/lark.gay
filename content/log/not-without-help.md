
+++
title = "Not Without Help: Accessibility in activism"
description = "I built a tool called Not Without Help, with the goal of connecting activist groups with people who want to help and including people who might not otherwise think they have a role."
date = 2025-01-03

[taxonomies]
tags = ["community"]

[extra]
uuid = "9819017f-062f-4cf5-9f00-d45d0a04649a"
+++

After attending a protest with a friend, we started talking about accessibility
in the context of activism. In particular, thinking about how attending public
actions isn't an option for everyone; it's difficult enough for me and my friend
due to sensory issues. But there are other ways to get involved; I did some web
development work for the group organizing the protest, for example.

The problem is, a lot of folks are turned off the idea of getting involved
because they have a limited view of what activism looks like. We started
thinking about how you can find roles for people in activist groups they might
not otherwise think they have a role in. We were inspired by [this
infographic](https://drdevonprice.substack.com/p/burning-it-all-down-without-burning?open=false#%C2%A7figure-out-your-activist-character-class)
by Devon Price, which introduces the concept of activist "character
classes"--examples of roles folks can take in activist group that go beyond
attending protests.

I decided to build a website called [Not Without Help](https://notwithout.help),
with the goal of connecting activist groups with people who want to get
involved. It's an inbox for organizers to collect contact information from folks
who are interested, and where volunteers can share information about what kinds
of help they can offer. I used Devon Price's activist character classes as a
template for respondents to check off which roles they're interested in.

It's intended to be a replacement for tools like Google Forms, albeit with a
much more limited feature set. I implemented end-to-end encryption for
responses, so only the organizers can see the information people submit. I also
implemented a system for access control, so that organizers can control who they
authorize to view the responses, and revoke that access if necessary.

Right now, the tool is pretty simple and the feature set is fairly limited. I
have some ideas for what comes next, but I need to give more thought to exactly
what problems I'm trying to solve, and probably get feedback from activist
groups to understand what their needs are. Some ideas I've had so far:

- Building it out into a more generic form builder, or at least allowing
  organizers to define their own roles not on the preset list.
- Implementing a sort of job board where organizers can opt to list their group
  in a central directory, and volunteers can browse groups looking for help.

I'm happy with the progress I've made so far, and I'm excited to move forward
with the project and see what comes next.
