<template>
    <div>
        <component v-bind:is="currentView"></component>
    </div>    
</template>

<script lang="ts">
import Vue from 'vue'
import {lobby, noLobby, gameNotStarted, champSelect, inGame, endOfGame} from './view/view';

const AppProps = Vue.extend({
    props: {
        title: String,
        state: String,
    },

    components: {
        lobby,
        gameNotStarted,
        noLobby,
        champSelect,
        inGame,
        endOfGame
    },
})

export class MainComponent extends AppProps {
    get currentView() {
        return this.state;
    }

    get computedTitle() {
        if (this.title) {
            return this.title;
        } else {
            return "loading";
        }
    }
}

export enum ViewState {
    GameNotStarted = "gameNotStarted",
    NoLobby = "noLobby",
    Lobby = "lobby",
    ChampSelect = "champSelect",
    InGame = "inGame",
    EndOfGame = "endOfGame",
}

export default MainComponent;
</script>


<style lang="scss" scoped>
    div {
        display: flex;
        flex-direction: column;
        height: 100%;
        width: 100%;
    }
</style>
