<template>
  <div class="w-100 h-100 text-center d-flex flex-column">
    <div class="form-group m-2">
      <input type="file" id="formNomFichier" />
    </div>
    <div class="w-100">
      <button type="submit" class="btn w-25 btn-primary">
        <span class="glyphicon glyphicon-upload"></span> Upload
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "Upload",
});
</script>
