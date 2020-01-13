# DonBot.rs : automated content creation

DonBot is a toolkit with a variety of features, all geared towards automatically generating entertaining content.

[![Build Status](https://travis-ci.com/Mimerme/donbot.rs.svg?branch=master)](https://travis-ci.com/Mimerme/donbot.rs)

## Feature List

- [x] Automatically curate and upload highlight reels of Twitch clips to YouTube
- [ ] Bind a key to generate a clip from a stream
- [ ] Automatically generate clips from a stream based on chat messages
- [x] Play a sound on a local machine whenever a chat messages

The code is currently pretty messy and needs some cleaning up so that it can be easily understood and reconfigured


## Project Structure
All drivers and build targets should be stored under ```/src```.
A general rule of thumb for drivers is that they should contain enviornment dependent code (e.g. Code that may run only under a specific operating system, system configruations, etc.).

All other DonBot related code should be stored under subdirectores ```/src/don_bot/*/```. I call them __modules__

Code that that does is not DonBot specific is stored under ```/src/utils/```. This includes additional things that beatify the output, filter characters from filenames, etc.

__Sidenote on configuration__ : DonBot will always use ```config.ini```  to fetch configuration values. Modules and drivers should only access their own configurations specified by the section names. Code in ```/src/utils/``` should not read or write to it. 

## Build Targets
- ```auto_stitch``` : downloads the top Twitch clips for a game and combines them into a single Youtube video

- ```chat_note``` : play a sound file every time you get a chat message in your channel

- ```config_oauth``` : create a Youtube OAuth token

- ```updater``` : setups up a GitHub webhook that automatically rebuilds the latest DonBot targets

- ```gen_enc``` : generates a GStreamer encoding profile based on a source file

## Build Structure
This repository uses Travis to build and test release binaries. If you're looking to build this project on your location machine look at the ```.travis.yml``` file to see
what you need to install. (Note: cargo won't be able to manage all of the depepdencies DonBot needs like GStreamer, so you'll need to install those yourself). 

There are two types of tests in DonBot: normal cargo tests and limited tests (specified by the conditional compilation macro ```#[cfg(bar = "limit_test")]```). Limited tests are not run by Travis and involve consuming API quotas, having certain uneccesary files available, etc. Examples of this are tests that upload a Youtube video or concat local video files.

## Notes:
Hello person who is reading this that isn't me. I'm surprised that you decided the check out this project cause there isn't really anything to show yet, but its coming along! I'm activly working on this, but the development is pretty informal because of the loose project specification that I gave it. So I'm bouncing from feature to feature, not fully commiting to anything much because of school and applying to jobs and stuff, but it'll get done. Eventually (1/12/2020)
