export class Player {
    SummonerName : String;
    SummonerID : Number;
    Ranked : Ranked;
    LobbyOwner : boolean;
    ChampionID : Number;
}

export class Ranked {
    Tier : String;
    Division : Number;
    LP : Number;
}

export function newTestPlayer() : Player {
    let payload : Player = new Player();
    payload.SummonerName = "SEQUOIIA";
    payload.LobbyOwner = false;
    payload.ChampionID = -1;

    let ranked : Ranked = new Ranked();
    ranked.Tier = "Platnium";
    ranked.Division = 4;
    ranked.LP = 1;
    payload.Ranked = ranked;

    return payload;
}

export function newTestPlayerEnemy() : Player {
    let payload : Player = new Player();
    payload.ChampionID = -1;
    payload.SummonerName = "Enemy";

    return payload;
}

export function newTestPlayerLobbyOwner() : Player {
    let payload : Player = newTestPlayer();
    payload.LobbyOwner = true;
    payload.SummonerName = "SEQUOIIA12345678";

    return payload;
}