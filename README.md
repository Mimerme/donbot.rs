# DonBot.rs : automated content creation

DonBot is a toolkit with a variety of features, all geared towards automatically generating entertaining content.

## Feature List

- [x] Automatically curate and upload highlight reels of Twitch clips to YouTube
- [ ] Bind a key to generate a clip from a stream
- [ ] Automatically generate clips from a stream based on chat messages
- [x] Play a sound on a local machine whenever a chat messages

The code is currently pretty messy and needs some cleaning up so that it can be easily understood and reconfigured


## Project Structure
All drivers and build targets should be stored under ```/src```.
A general rule of thumb for drivers is that they should contain enviornment dependent code (e.g. Code that may run only under a specific operating system, system configruations, etc.).

All other code should be stored under subdirectores ```/src/*/```. I call them __modules__

__Sidenote on configuration__ : DonBot will always use ```config.ini```  to fetch configuration values. Modules and drivers should only access their own configurations specified by the section names.

## Build Targets
- ```auto_stitch``` : downloads the top Twitch clips for a game and combines them into a single Youtube video

- ```chat_note``` : play a sound file every time you get a chat message in your channel

- ```config_oauth``` : create a Youtube OAuth token

- ```updater``` : setups up a GitHub webhook that automatically rebuilds the latest DonBot targets

- ```gen_enc``` : generates a GStreamer encoding profile based on a source file

## Notes:
Hello person who is reading this that isn't me. I'm surprised that you decided the check out this project cause there isn't really anything to show yet, but its coming along! I'm activly working on this, but the development is pretty informal because of the loose project specification that I gave it. So I'm bouncing from feature to feature, not fully commiting to anything much because of school and applying to jobs and stuff, but it'll get done. Eventually (1/12/2020)
