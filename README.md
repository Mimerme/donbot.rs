# donbot.rs
### DonBot is aimed to be an automated content creation toolkit for streamers and enthuisiets alike. 

## Structure
All drivers and build targets should be stored under ```/src```.
A general rule of thumb for drivers is that they should contain all the enviornment depdent code (e.g. Code that runs only under a specific operating system).
All other code should be stored under subdirectores ```/src/*/```. I call theses additional __modules__

__Sidenote on configuration__ : DonBot will always use ```config.ini```  to fetch configuration values. 

## Build Targets
- ```auto_stitch``` : downloads the top Twitch clips for a game and combines them into a single Youtube video

- ```chat_note``` : play a sound file every time you get a chat message in your channel

- ```config_oauth``` : configure the Youtube oauth token. works in conjunction with ```auto_stitch```

- ```updater``` : setups up a GitHub webhook that automatically updates DonBot
