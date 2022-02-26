# list the bots that are currently in a channel

1. get list of all bots from https://api.twitchinsights.net/v1/bots/all
2. check all users in a channel
3. display the list of bots in that channel

# Usage

With cargo:
```
cargo run CHANNEL_NAME
```


With binary (see [release page](https://github.com/Zutatensuppe/twitch-bots/releases/latest)):
```
twitch-bots CHANNEL_NAME
```


# Build

Build for linux and windows (see scripts/build for details):
```
./run build
```
