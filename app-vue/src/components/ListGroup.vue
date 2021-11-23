<template>
  <ul class="list-group">
    <li v-for="(name, index) in listFiles" :key="index" class="list-group-item">
      <div class="d-flex">
        <div class="col-8">{{ name }}</div>
        <div class="col-2">
          <button @click="download(name)" type="button" class="btn btn-primary">
            Télécharger
          </button>
        </div>
      </div>
    </li>
  </ul>
</template>

<script lang="ts">
import { mapActions, mapState } from "vuex";
import { Constants } from "@/constants/Constants";
import { defineComponent } from "vue";
import FilesActionTypes from "@/store/files/files-action-types";

export default defineComponent({
  name: "ListGroup",
  computed: {
    ...mapState(Constants.FILES_STORE, ["listFiles"]),
  },
  methods: {
    ...mapActions(Constants.FILES_STORE, [FilesActionTypes.DOWNLOAD_FILE]),
    async download(fileName: string) {
      this[FilesActionTypes.DOWNLOAD_FILE](fileName);
    },
  },
});
</script>
