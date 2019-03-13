<template>
    <div class="container">
        <div class="logo"></div>
        <h1>Waiting for lobby</h1>
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
    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        background-color: #f5feff;
    }

    .logo {
        background-image: url("../../../../../static/logo_a.png");
        background-repeat: no-repeat;
        background-size: cover;
        width: 310px;
        height: 350px;
        margin-bottom: 10px;
    }

    h1 {
        font-family: 'Open Sans';
        font-weight: 700;
        -webkit-user-select: none;
    }
</style>
