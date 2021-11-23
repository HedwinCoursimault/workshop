<template>
  <ListGroup />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapActions } from "vuex";
import { Constants } from "@/constants/Constants";
import FilesActionTypes from "@/store/files/files-action-types";
import ListGroup from "@/components/ListGroup.vue";

export default defineComponent({
  name: "App",
  components: {
    ListGroup,
  },
  async mounted() {
    await this[FilesActionTypes.GET_LIST_NAMES]();
  },
  methods: {
    ...mapActions(Constants.FILES_STORE, [FilesActionTypes.GET_LIST_NAMES]),
  },
});
</script>
