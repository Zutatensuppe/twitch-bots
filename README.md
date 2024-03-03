# list the bots that are currently in a channel

1. get list of all bots from https://api.twitchinsights.net/v1/bots/all
2. check all users in a channel
3. display the list of bots in that channel

## Usage

Create a file `CLIENT_ID` and add your client id to it and nothing else.

With cargo:

```console
cargo run CHANNEL_NAME
```

With binary (see [release page](https://github.com/Zutatensuppe/twitch-bots/releases/latest)):

```console
twitch-bots CHANNEL_NAME
```

## Build

Build for linux and windows (see scripts/build for details):

```console
./run build
```
