<template>
    <div class="titlebar">
        <div class="col title">
            <div class="logo"></div>
            <span>Aigis</span>
        </div>        
        <div class="controls">
            <a data-radium="true" style="user-select: none; -webkit-app-region: no-drag; cursor: default; width: 46px; height: 100%; line-height: 0; display: flex; justify-content: center; align-items: center;"><svg x="0px" y="0px" viewBox="0 0 10.2 1" data-radium="true" style="width: 10px; height: 10px; "><rect fill="rgba(0, 0, 0, .4)" width="10.2" height="1"></rect></svg></a>
            <a data-radium="true" style="user-select: none; -webkit-app-region: no-drag; cursor: default; width: 46px; height: 100%; line-height: 0; display: flex; justify-content: center; align-items: center;"><svg x="0px" y="0px" viewBox="0 0 10.2 10.2" data-radium="true" style="width: 10px; height: 10px;"><path fill="rgba(0, 0, 0, .4)" d="M2.1,0v2H0v8.1h8.2v-2h2V0H2.1z M7.2,9.2H1.1V3h6.1V9.2z M9.2,7.1h-1V2H3.1V1h6.1V7.1z"></path></svg></a>        
            <a class="red" data-radium="true" style="user-select: none; -webkit-app-region: no-drag; cursor: default; width: 46px; height: 100%; line-height: 0; display: flex; justify-content: center; align-items: center;"><svg x="0px" y="0px" viewBox="0 0 10.2 10.2" data-radium="true" style="width: 10px; height: 10px;"><polygon fill="rgba(0, 0, 0, .4)" points="10.2,0.7 9.5,0 5.1,4.4 0.7,0 0,0.7 4.4,5.1 0,9.5 0.7,10.2 5.1,5.8 9.5,10.2 10.2,9.5 5.8,5.1 "></polygon></svg></a>
        </div>
    </div>    
</template>

<script lang="ts">
import Vue from 'vue'

const AppProps = Vue.extend({

})

export default class MainComponent extends AppProps {

}
</script>


<style lang="scss" scoped>
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

    .col {
        flex: 1;
    }

    .col2 {
        flex: 2;
    }
    
    .controls {
        display: flex;
        height: 100%;

        a {
            height: 100%;
            &:hover {
                transition: background-color 0.1s ease 0s; background-color: rgb(229, 229, 229);
            }

            &.red {
                &:hover {
                    transition: background-color 0.1s ease 0s; background-color: rgb(232, 17, 35);
                }
            }
        }
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

    .titlebar {
        display: flex;
        flex-direction: row;
        align-items: center;
        -webkit-user-select: none;
        -webkit-app-region: drag;   
        width: 100%;
        height: 31px;
        background: rgb(109, 196, 236);
    }
</style>
