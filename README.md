![logo](https://github.com/SEQUOIIA/Aigis/blob/master/static/logo_white_transparent.png)
# Aigis -  Your reliable DUO partner in League of legends
Master - [![Build Status](https://travis-ci.com/SEQUOIIA/Aigis.svg?branch=master)](https://travis-ci.com/SEQUOIIA/Aigis)

Source code soon to be publicly available

## Program
Soon
## Lib
Soon
## Tools
### Dumper (tools/analyser)
Dump data from LCU for research purposes. It can currently do the following:
  * Listen for all events available through the websocket, and save them to disk
  * Get the latest swagger.json
  
Both options will require the League client to be open. The websocket option will listen until either the League client closes down or the dumper tool is closed down.
```
dumper 0.1.0
Emil H. Clausen (SEQUOIIA) <sequoiia@hummel.yt>
Dump data straight from LCU

USAGE:
    aigis_dumper.exe [FLAGS] --league-path <league-path> [SUBCOMMAND]

FLAGS:
    -d, --debug
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --league-path <league-path>    Path to the League of Legends directory

SUBCOMMANDS:
    help         Prints this message or the help of the given subcommand(s)
    swagger      Dump swagger.json from LCU
    websocket    Dump data from LCU websocket
```
  
### Analyser (tools/dumper)
Takes as input a websocket dump and outputs every single event as a JSON file, also creates a file called "**AIGIS_uri-list**" that lists every event.

```
analyser 0.1.0
Emil H. Clausen (SEQUOIIA) <sequoiia@hummel.yt>
Analyse JSON dumps from the dumper tool

USAGE:
    aigis_analyser.exe [FLAGS] [OPTIONS] <path>

FLAGS:
    -d, --debug
    -h, --help       Prints help information
    -l, --legacy     Include this arg for parsing legacy websocket dumps
    -V, --version    Prints version information

OPTIONS:
    -o, --output-path <output-path>    Path to directory where Rust structs will be output [default: output]

ARGS:
    <path>    Path to the JSON dump
```

The "AIGIS_uri-list" file looks like this:
```
/riotclient/affinity - _riotclient_affinity.json
/lol-clash/v1/time - _lol-clash_v1_time.json
/lol-clash/v1/visible - _lol-clash_v1_visible.json
/lol-clash/v1/enabled - _lol-clash_v1_enabled.json
/lol-clash/v1/ready - _lol-clash_v1_ready.json
/lol-clash/v1/playmode-restricted - _lol-clash_v1_playmode-restricted.json
/lol-clash/v2/playmode-restricted - _lol-clash_v2_playmode-restricted.json
/lol-gameflow/v1/session - _lol-gameflow_v1_session.json
/lol-gameflow/v1/gameflow-phase - _lol-gameflow_v1_gameflow-phase.json
```

Lets say I wanted to see how "/lol-gameflow/v1/session" looks like. Simply just open its accompanying JSON file "_lol-gameflow_v1_session.json", which looks like this:
```
{"data":{"gameClient":{"observerServerIp":"","observerServerPort":0,"running":false,"serverIp":"","serverPort":0,"visible":false},"gameData":{"gameId":0,"gameName":"","isCustomGame":false,"password":"","playerChampionSelections":[],"queue":{"areFreeChampionsAllowed":false,"assetMutator":"","category":"None","description":"","detailedDescription":"","gameMode":"","gameTypeConfig":{"advancedLearningQuests":false,"allowTrades":false,"banMode":"","banTimerDuration":0,"battleBoost":false,"crossTeamChampionPool":false,"deathMatch":false,"doNotRemove":false,"duplicatePick":false,"exclusivePick":false,"id":0,"learningQuests":false,"mainPickTimerDuration":0,"maxAllowableBans":0,"name":"","onboardCoopBeginner":false,"pickMode":"","postPickTimerDuration":0,"reroll":false,"teamChampionPool":false},"id":-1,"isRanked":false,"isTeamBuilderManaged":false,"isTeamOnly":false,"mapId":0,"maxLevel":0,"maxSummonerLevelForFirstWinOfTheDay":0,"maximumParticipantListSize":0,"minLevel":0,"minimumParticipantListSize":0,"name":"","numPlayersPerTeam":0,"queueAvailability":"Available","queueRewards":{"isChampionPointsEnabled":false,"isIpEnabled":false,"isXpEnabled":false,"partySizeIpRewards":[]},"shortName":"","spectatorEnabled":false,"type":""},"spectatorsAllowed":false,"teamOne":[],"teamTwo":[]},"gameDodge":{"dodgeIds":[],"phase":"None","state":"Invalid"},"map":{"assets":null,"categorizedContentBundles":null,"description":"","gameMode":"","gameModeName":"","gameModeShortName":"","gameMutator":"","id":0,"isRGM":false,"mapStringId":"","name":"","platformId":"","platformName":"","properties":null},"phase":"Lobby"},"eventType":"Update","uri":"/lol-gameflow/v1/session"}
```

Analyser is currently unaware of endpoints that takes parameters, so some events may occur more than once, e.g.

```
/lol-game-client-chat/v1/buddies/friend1 - _lol-game-client-chat_v1_buddies_friend1.json
/lol-game-client-chat/v1/buddies/poro2 - _lol-game-client-chat_v1_buddies_poro2.json
```
