import { createStore } from "vuex";
import {Constants} from "@/constants/Constants";
import filesModules from "@/store/files/files-module.store";

export default createStore({
  state: {},
  mutations: {},
  actions: {},
  modules: {
    [Constants.FILES_STORE]: filesModules
  },
});
