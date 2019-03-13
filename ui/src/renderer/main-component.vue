<template>
    <div id="app">
        <TitleBarComponent />
        <DebugMenuComponent v-if="computedDebug == true" @stateSwitch="handleStateSwitch" />
        <LCUStateComponent :state=currentState />
    </div>    
</template>

<script lang="ts">
import Vue from 'vue'
import {ViewState, MainComponent as LCUStateComponent} from './components/lcu-state-component.vue';
import DebugMenuComponent from './components/ui/debug-menu.vue';
import TitleBarComponent from './components/ui/titlebar.vue';

const ENABLE_DEBUG = true;

const AppProps = Vue.extend({
    props: {
        title: String
    },

    components: {
        LCUStateComponent,
        DebugMenuComponent,
        TitleBarComponent
    },

    data: () => {
        return {
            currentState : ViewState.InGame
        }
    },

    methods: {
        handleStateSwitch(state : String) {
            this.currentState = state;
        }
    }
})

export default class MainComponent extends AppProps {
    get computedTitle() {
        if (this.title) {
            return this.title;
        } else {
            return "loading";
        }
    }

    get computedDebug() {
        return ENABLE_DEBUG;
    }

}
</script>


<style lang="scss" scoped>
    #app {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
    }
</style>
