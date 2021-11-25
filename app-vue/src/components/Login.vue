<template>
  <form>
    <div class="row">
      <label> Login : </label>
      <input type="text" v-model="login">
    </div>
    <div class="row">
      <label> Password : </label>
      <input type="password" v-model="passwd">
    </div>
  </form>
  <button @click="connection()" type="button" class="btn btn-primary">Se connecter</button>
</template>

<script>
import {defineComponent} from "vue";
import {mapActions} from "vuex";
import {Constants} from "@/constants/Constants";
import AuthActionTypes from "@/store/auth/auth-action-types";

export default defineComponent({
  name: "Login",
  data(){
    return {
      login: "",
      passwd: ""
    }
  },
  methods: {
    ...mapActions(Constants.AUTH_STORE, [AuthActionTypes.TRY_CONNECTION]),
    connection(){
      this[AuthActionTypes.TRY_CONNECTION]({login: this.login, password: this.passwd});
    }
  }
})
</script>