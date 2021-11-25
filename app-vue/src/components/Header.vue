<template>
  <div
    class="
      w-100
      bg-primary
      d-flex
      justify-content-left
      align-items-center
      pb-2
      pt-2
    "
  >
    <img src="../assets/rhino_alt.png" alt="" style="max-height: 80px" />
    <p
      class="align-middle mb-0 mt-0 text-white display-5"
      style="font-family: 'Oswald', sans-serif"
    >
      RhinoDrive, drive safe
    </p>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
export default defineComponent({
  name: "HeaderMain",
});
</script>
