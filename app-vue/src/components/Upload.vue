<template>
  <div class="w-100 h-100 text-center d-flex flex-column mt-2 justify-content-center">
    <div class="form-group m-2">
      <input type="file" id="formNomFichier" @change="onFileChanged" />
    </div>
    <div class="w-100">
      <button type="submit" class="btn w-25 btn-primary" @click="upload">
        <span class="glyphicon glyphicon-upload"></span> Upload
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapActions } from "vuex";
import { Constants } from "@/constants/Constants";
import FilesActionTypes from "@/store/files/files-action-types";

export default defineComponent({
  name: "Upload",

  methods: {
    ...mapActions(Constants.FILES_STORE, [FilesActionTypes.UPLOAD_FILE]),
    async upload() {
      console.log(this.file);
      this[FilesActionTypes.UPLOAD_FILE](this.file);
    },
    onFileChanged(event: any) {
      this.file = event.target.files[0];
    },
  },
  data() {
    return {
      file: "",
    };
  },
});
</script>
