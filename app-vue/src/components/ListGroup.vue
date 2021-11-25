<template>
  <ul class="list-group">
    <div v-if="listFiles.length <= 1">{{listFiles.length}} fichier trouvé</div>
    <div v-else> {{listFiles.length}} fichiers trouvés</div>
    <li v-for="(fileItem, index) in listFiles" :key="index" class="list-group-item">
      <div class="row">
        <div class="fichierTitle col-10">{{ fileItem.name }}</div>
        <div class="col-2">
          <button @click="download(fileItem)" type="button" class="btn btn-primary">
            <BIconDownload/>
          </button>
        </div>
      </div>
      <div class="row">
        <div class="fichierContent col-9"> Date : {{ fileItem.date }}</div>
        <div class="fichierContent col-3 contentAtRight"> Taille : {{ fileItem.taille }}</div>
      </div>
    </li>
  </ul>
</template>

<script lang="ts">
import { mapActions, mapState } from "vuex";
import { Constants } from "@/constants/Constants";
import { defineComponent } from "vue";
import FilesActionTypes from "@/store/files/files-action-types";
import { BIconDownload} from 'bootstrap-icons-vue';
export default defineComponent({
  name: "ListGroup",
  components: {
    BIconDownload
  },
  computed: {
    ...mapState(Constants.FILES_STORE, ["listFiles"]),
  },
  mounted() {
    this[FilesActionTypes.GET_LIST_NAMES]();
  },
  methods: {
    ...mapActions(Constants.FILES_STORE, [FilesActionTypes.DOWNLOAD_FILE, FilesActionTypes.GET_LIST_NAMES]),
    async download(file: any) {
      this[FilesActionTypes.DOWNLOAD_FILE](file);
    },
  },
});
</script>
<style>
.fichierTitle{
    display: inline-block;
}
.fichierButton{
    display: inline-block;
}
.fichierContent{
  font-size: 10px;
  display: inline-block;
}
.fileAtRight{
    text-align: right;
}
</style>
