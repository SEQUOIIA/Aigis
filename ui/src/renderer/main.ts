import Vue from 'vue';
import MainApp from './main-component.vue';
import './styling/main.scss';

let app = new Vue({
    el: "#app",
    render: r => r(MainApp, {
        props: {title: "AIGIS"}
    })
});