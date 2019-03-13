<template>
    <div class="lobby">
        <div class="content">
            <div class="players friendly">
                <PlayerComponent :PlayerData=testPlayerLobbyOwner :state=champSelect />
                <PlayerComponent :PlayerData=testPlayer />
                <PlayerComponent :PlayerData=testPlayer />
                <PlayerComponent :PlayerData=testPlayer />
                <PlayerComponent :PlayerData=testPlayer />                
            </div>

            <div class="players enemy">
                <PlayerComponent :PlayerData=testPlayer />
                <PlayerComponent :PlayerData=testPlayer />
                <PlayerComponent :PlayerData=testPlayer />
                <PlayerComponent :PlayerData=testPlayer />                                                
                <PlayerComponent :PlayerData=testPlayer />                                                
            </div>            
        </div>
        <div class="logo"></div>
    </div>    
</template>

<script lang="ts">
import Vue from 'vue'
import PlayerComponent from './../various/player-champ-select.vue';
import {newTestPlayer, Player, newTestPlayerLobbyOwner, newTestPlayerEnemy} from '../../models/player';
import {ViewState} from './../../components/lcu-state-component.vue';

const AppProps = Vue.extend({
    components: {
        PlayerComponent
    }
})

export default class MainComponent extends AppProps {
    testPlayer : Player = newTestPlayer();
    testPlayerEnemy : Player = newTestPlayerEnemy();
    testPlayerLobbyOwner : Player = newTestPlayerLobbyOwner();
    champSelect : String = ViewState.ChampSelect;
}
</script>


<style lang="scss" scoped>
    .lobby {
        background-image: url("../../../../../static/bg-placeholder.png");
        background-repeat: no-repeat;
        background-color: rgb(43, 43, 43);
        background-size: cover;
    }

    h1 {
        color: #f3f3f3;
        -webkit-user-select: none;
        font-family: 'Open Sans';
    }

    .content {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        width: 100%;
        height: 100%;
    }

    .players {
        width: 90%;
        height: 100%;
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;

        &.friendly {
            background-color:rgba(162, 255, 101, 0.1);
        }

        &.enemy {
            background-color:rgba(255, 119, 101, 0.1);
        }        
    }

    .logo {
        position: fixed;
        background-image: url("../../../../../static/logo_a.png");
        background-repeat: no-repeat;
        background-size: cover;
        width: 69px;
        height: 80px;
        bottom: 0;
        margin-left: 15px;
        margin-bottom: 15px;
        opacity:0.5;
        z-index: 99;
    }
</style>
