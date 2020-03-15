# DonBot.rs : automated content creation

DonBot is an automated content creation toolkit.

[![Build Status](https://travis-ci.com/Mimerme/donbot.rs.svg?branch=master)](https://travis-ci.com/Mimerme/donbot.rs)

## Feature List
If you're a developer here is a list of what the ```donbot``` library currently implements. As you can see its quite all over the place, but the library mainly serves as a wrapper for a lot of common automation tasks in case other developers decide the write more drivers. The library should only contain non-platform dependent code. 

- [-] **Youtube**
	- [x] Upload videos
	- [ ] Retrieve comments
	- [x] Config OAuth
- [-] **Twitch**
	- [x] Create clip
	- [x] Download clip
	- [x] Find UserID
	- [x] Get top clips
- [-] **GStreamer (Multimedia)**
	- [x] Stitch multiple video clips together
	- [x] Generate video encoding profile
	- [ ] Apply an overlay on the videos

Also ```donbot``` gets all of its configuration values from ```config.ini```

## Drivers and Build Targets
If you're a casual user or want to write some automation tasks using the ```donbot``` library "drivers" are what you're looking for. Drivers are a conglomerate of automation tasks from ```donbot```. All platform dependent code should be stored here. All drivers should also have a corresponding build target in ```cargo.toml```

- [x]  ```auto_stitch``` : downloads the top Twitch clips for a game and combines them into a single Youtube video
- [ ] Bind a key to generate a clip from a stream and optionally download it
- [ ] Automatically generate clips from a stream based on chat messages
- [x] ```chat_note``` : play a sound file every time you get a chat message in your channel
- [x] ```config_oauth``` : create a Youtube OAuth token

- [-] ```updater``` : setups up a GitHub webhook that automatically rebuilds the latest DonBot targets

- [-] ```update_bins``` : setups up a GitHub webhook that automatically downloads newly built binaries from Travis

- [x]```gen_enc``` : generates a GStreamer encoding profile based on a source file

The code is currently pretty messy and needs some cleaning up so that it can be easily understood and reconfigured

 
---------------------------------------------------------

### Project Structure
All drivers and build targets should be stored under ```/src```.
A general rule of thumb for drivers is that they should contain enviornment dependent code (e.g. Code that may run only under a specific operating system, system configruations, etc.).

All other DonBot related code should be stored under subdirectores ```/src/don_bot/*/```. I call them __modules__

Code that that is not specifically written for DonBot (aka Code that can be easily reused)is stored under ```/src/utils.rs```. This includes additional things that beatify the output, filter characters from filenames, etc.

__Sidenote on configuration__ : DonBot will always use ```config.ini```  to fetch configuration values. Modules and drivers should only access their own configurations specified by the section names. Code in ```/src/utils/``` should not read or write to it. 


## Build Structure
lol none of this is implemented yet

~~This repository uses Travis to build and test release binaries. If you're looking to build this project on your location machine look at the ```.travis.yml``` file to see
what you need to install. (Note: cargo won't be able to manage all of the depepdencies DonBot needs like GStreamer, so you'll need to install those yourself).~~

~~There are two types of tests in DonBot: normal cargo tests and limited tests (specified by the conditional compilation macro ```#[cfg(bar = "limit_test")]```). Limited tests are not run by Travis and involve consuming API quotas, having certain uneccesary files available, etc. Examples of this are tests that upload a Youtube video or concat local video files.~~


-----------------------------------------------------------

# Build instructions
Some basic instructions to build the DonBot targets from source on different systems. Use these as some general guidelines. DonBot runs on Rust so it uses ```cargo``` for dependency management for the most part. However we also have some targets with platform specific dependencies, which we must install here.

### General instructions
1) https://gitlab.freedesktop.org/gstreamer/gstreamer-rs

2) ```rustup default nightly```

3) https://www.freedesktop.org/wiki/Software/pkg-config/

4) https://github.com/openssl/openssl/releases/tag/OpenSSL_1_0_2u

### Windows
2)

### Linux
2) Follow the instructions to install gstreamer above

### OSx
2) Follow the instructions to install gstreamer above


####### Notes:
Hello person who is reading this that isn't me. I'm surprised that you decided the check out this project cause there isn't really anything to show yet, but its coming along! I'm activly working on this, but the development is pretty informal because of the loose project specification that I gave it. So I'm bouncing from feature to feature, not fully commiting to anything much because of school and applying to jobs and stuff, but it'll get done. Eventually (1/12/2020)

School is picking up, but I got an job and there is a need for DonBot to be cleaned up, so active development will probably be coming along, but a lot of it will be cleaning up the build process. (2/28/2020)