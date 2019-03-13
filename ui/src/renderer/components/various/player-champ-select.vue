<template>
    <div class="player">
        <div class="d2">
            <img class="ranked-icon" src="static/ranked-icon-placeholder.png" />
            <div class="header">
                <img class="summoner-icon" src="static/summoner-icon-placeholder.png" />
                <h1>{{PlayerData.SummonerName}}</h1>
            </div>
            <h2 v-if="PlayerData.Ranked">{{PlayerData.Ranked.Tier}} {{PlayerData.Ranked.Division}} - {{PlayerData.Ranked.LP}} LP</h2>            
        </div>

        <div class="d2 champ">
        <img class="champion-icon" src="static/champion-icon-placeholder.png" />
        </div>
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

    .d1 {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    .d2 {
        flex: 2;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        &.champ {
            justify-content: flex-start;
        }
    }

    .header {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        width: 100%;
        margin-bottom: 14px;
    }

    .champion-icon {
        width: 9vw;
        
        &:hover {
            border: solid #ffffff;
            border-width: 3px;
        }        
    }

    .summoner-icon {
        width: 30px;
        height: 30px;
        border-radius: 60%;
        border: solid #ffffff;
        border-width: 3px;
        margin-right: 7px;
    }

    .ranked-icon {
        width: 39px;
    }

    h1 {
        font-size: 1.8vw;
        font-weight: 700;
        text-align: center;
        overflow: hidden;
    }

    h2 {
        font-size: 1.5vw;
        font-weight: 700;
    }
}

</style>
