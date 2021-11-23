<template>
  <img alt="Vue logo" src="./assets/logo.png"/>
  <HelloWorld msg="Welcome to Your Vue.js + TypeScript App" />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import HelloWorld from "./components/HelloWorld.vue";
import {mapActions, mapState} from "vuex";
import {Constants} from "@/constants/Constants";
import FilesActionTypes from "@/store/files/files-action-types";

export default defineComponent({
  name: "App",
  components: {
    HelloWorld,
  },
  mounted() {
    this[FilesActionTypes.GET_LIST_NAMES]();
  },
  computed:{
    ...mapActions(Constants.FILES_STORE, [FilesActionTypes.GET_LIST_NAMES]),
    ...mapState(Constants.FILES_STORE,['listFiles']),
  }
});
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
