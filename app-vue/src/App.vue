<template>
  <div v-if="isConnected" id="div-main" class="d-flex w-100">
    <div id="main-div" class="d-flex flex-column w-100">
      <HeaderMain />
      <div id="content" class="d-flex h-100">
        <div id="div-nav" class="col-3">
          <p class="text-center mt-2">Vos fichiers sauvegardés</p>
          <ListGroup />
        </div>
        <div class="w-100">
          <Upload />
        </div>
      </div>
    </div>
  </div>
  <div v-else>
    <Login />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapActions, mapState } from "vuex";
import { Constants } from "@/constants/Constants";
import FilesActionTypes from "@/store/files/files-action-types";
import ListGroup from "@/components/ListGroup.vue";
import Upload from "@/components/Upload.vue";
import Login from "@/components/Login.vue";
import HeaderMain from "@/components/Header.vue";

export default defineComponent({
  name: "App",
  components: {
    ListGroup,
    Upload,
    Login,
    HeaderMain,
  },
  methods: {
    ...mapActions(Constants.FILES_STORE, [FilesActionTypes.GET_LIST_NAMES]),
  },
  computed: {
    ...mapState(Constants.AUTH_STORE, ["isConnected"]),
  },
});
</script>
<style scoped>
#div-nav {
  border-right-style: solid;
  border-right-width: 1px;
  height: 100%;
}

#div-main {
  height: 100%;
  width: 100%;
}
</style>
