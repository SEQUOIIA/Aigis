<template>
    <div class="player">
        <h2 class="type" v-if="ComputedShowLobbyOwner == true">LOBBY OWNER</h2>
        <img class="ranked-icon" src="static/ranked-icon-placeholder.png" />
        <img class="summoner-icon" src="static/summoner-icon-placeholder.png" />
        <h1>{{PlayerData.SummonerName}}</h1>
        <h2>{{PlayerData.Ranked.Tier}} {{PlayerData.Ranked.Division}} - {{PlayerData.Ranked.LP}} LP</h2>
    </div>    
</template>

<script lang="ts">
import Vue from 'vue'
import {Player} from '../../models/player';
import {ViewState} from './../../components/lcu-state-component.vue';


const AppProps = Vue.extend({
    props: {
        PlayerData : Player,
        state: String,
    },
})

export default class MainComponent extends AppProps {
    get ComputedShowLobbyOwner() {
        if (this.PlayerData.LobbyOwner == true) {
            if (this.state) {
                if (this.state === ViewState.ChampSelect) {
                    return false;
                }

                return true;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }
}
</script>


<style lang="scss" scoped>

.player {
    display: flex;
    flex: 1;
    color: #ffffff;
    display: flex;
    flex-direction: column;
    align-items: center;
    font-family: 'Open Sans';
    -webkit-user-select: none;
    word-break: break-all;
    height: 100%;
    justify-content: center;
    transition: background-color .25s ease-in-out;
    cursor: pointer;

    &:hover {
        background-color:rgba(255, 255, 255, 0.1);
    }

    .summoner-icon {
        width: 175px;
        border-radius: 60%;
        border: solid #ffffff;
        border-width: 3px;
        margin-bottom: 8px;
    }

    .ranked-icon {
        width: 39px;
    }

    h1 {
        font-size: 1.8vw;
        font-weight: 700;
        margin-bottom: 15px;
        text-align: center;
        overflow: hidden;
    }

    h2 {
        font-size: 1.5vw;
        font-weight: 700;

        &.type {
            font-size: 1.7em;
            position: relative;
            top: -45px;
            margin-bottom: -27px;
        }
    }
}

</style>
