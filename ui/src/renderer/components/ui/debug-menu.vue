<template>
    <div class="DebugMenu">
        <div class="buttons">
            <button @click="switchState(gameNotStarted)">game-not-started</button>
            <button @click="switchState(noLobby)">no-lobby</button>
            <button @click="switchState(lobby)">lobby</button>
            <button @click="switchState(champSelect)">champ-select</button>
            <button @click="switchState(inGame)">in-game</button>
            <button @click="switchState(endOfGame)">end-of-game</button>
            <button @click="switchState(live)">live</button>
        </div>
    </div>    
</template>

<script lang="ts">
import Vue from 'vue'
import {ViewState} from './../lcu-state-component.vue';

const AppProps = Vue.extend({
    props: {
        title: String,
        state: String
    },
})

export default class MainComponent extends AppProps {
    gameNotStarted : String = ViewState.GameNotStarted;
    noLobby : String = ViewState.NoLobby;
    lobby : String = ViewState.Lobby;
    champSelect : String = ViewState.ChampSelect;
    inGame : String = ViewState.InGame;
    endOfGame : String = ViewState.EndOfGame;
    live : String = "live";

    switchState(state : String) : void {
        this.$emit('stateSwitch', state);
    }

}
</script>


<style lang="scss" scoped>
    .DebugMenu {
        display: flex;
        justify-content: center;
        flex-direction: row;
        background-color: #4d4d4d;
        padding-top: 5px;
        padding-bottom: 5px;
        -webkit-user-select: none;

        .col {
            flex: 1;
        }

        .col2 {
            flex: 2;
        }

        .title {
            color: #ffffff;
            margin-left: 7px;
            display: flex;
            align-items: center;
            font-family: 'Open Sans';
            display: flex;
            align-items: center;

            span {
                margin-left: 5px;
            }

            .logo {
                background-image: url("../../../../../static/logo_a.png");
                background-repeat: no-repeat;
                background-size: cover;
                width: 18px;
                height: 21px;
            }            
        }
        
        .buttons {
            -webkit-user-select: none;
            -webkit-app-region: drag;            
            button {
                margin-left: 4px;
                margin-right: 4px;
                -webkit-user-select: none;
                -webkit-app-region: no-drag;
                font-family: 'Open Sans';

            }            
        }
    }
</style>
