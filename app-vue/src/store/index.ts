import { createStore } from "vuex";
import { Constants } from "@/constants/Constants";
import filesModules from "@/store/files/files-module.store";

export default createStore({
  modules: {
    [Constants.FILES_STORE]: filesModules,
  },
});
