<template>
    <div class="container">
        <div class="row py-5 mt-4 align-items-center">
            <!-- For Demo Purpose -->
            <div class="col-md-5 pr-lg-5 mb-5 mb-md-0">
              <img src="../assets/rhino.png" alt="" class="img-fluid mb-3 d-none d-md-block" />
                <h1>Accédez à votre RhinoDrive</h1>
            </div>

            <!-- Registeration Form -->
            <div class="col-md-7 col-lg-6 ml-auto"  v-show="(currentComponent === 'connexion' ? true : false)">
                <form action="#">
                    <div class="row">
                        <!-- Email Address -->
                        <div class="input-group col-lg-12 mb-4">
                            <div class="input-group-prepend">
                                <span class="input-group-text bg-white px-4 border-md border-right-0">
                                    <i class="fa fa-envelope text-muted"></i>
                                </span>
                            </div>
                            <input id="email" type="email" name="email" placeholder="Adresse mail" class="form-control bg-white border-left-0 border-md" v-model="login">
                        </div>

                        <!-- Password -->
                        <div class="input-group col-lg-6 mb-4">
                            <div class="input-group-prepend">
                                <span class="input-group-text bg-white px-4 border-md border-right-0">
                                    <i class="fa fa-lock text-muted"></i>
                                </span>
                            </div>
                            <input id="password" type="password" name="password" placeholder="Votre mot de passe" class="form-control bg-white border-left-0 border-md" v-model="passwd">
                        </div>
                        <!-- Submit Button -->
                        <div class="form-group col-lg-12 mx-auto mb-0 text-center">
                            <a @click="connection()" href="#" class="btn btn-primary btn-block py-2">
                                <span class="font-weight-bold">Entrer</span>
                            </a>
                        </div>

                        <!-- Divider Text -->
                        <div class="form-group col-lg-12 mx-auto d-flex align-items-center my-4">
                            <div class="border-bottom w-100 ml-5"></div>
                            <span class="px-2 small text-muted font-weight-bold text-muted">Ou</span>
                            <div class="border-bottom w-100 mr-5"></div>
                        </div>

                        <!-- Already Registered -->
                        <div class="text-center w-100">
                            <p class="text-muted font-weight-bold"> Nouveau ? <a @click="swapComponent('inscription')" href="#" class="text-primary ml-2">Créer un compte</a></p>
                        </div>

                    </div>
                </form>
            </div>
            <div class="col-md-7 col-lg-6 ml-auto"  v-show="(currentComponent === 'inscription' ? true : false)">
                <form action="#">
                    <div class="row">

                        <!-- First Name -->
                        <div class="input-group col-lg-6 mb-4">
                            <div class="input-group-prepend">
                                <span class="input-group-text bg-white px-4 border-md border-right-0">
                                    <i class="fa fa-user text-muted"></i>
                                </span>
                            </div>
                            <input id="firstName" type="text" name="firstname" placeholder="Prénom" class="form-control bg-white border-left-0 border-md">
                        </div>

                        <!-- Last Name -->
                        <div class="input-group col-lg-6 mb-4">
                            <div class="input-group-prepend">
                                <span class="input-group-text bg-white px-4 border-md border-right-0">
                                    <i class="fa fa-user text-muted"></i>
                                </span>
                            </div>
                            <input id="lastName" type="text" name="lastname" placeholder="Nom" class="form-control bg-white border-left-0 border-md">
                        </div>

                        <!-- Email Address -->
                        <div class="input-group col-lg-12 mb-4">
                            <div class="input-group-prepend">
                                <span class="input-group-text bg-white px-4 border-md border-right-0">
                                    <i class="fa fa-envelope text-muted"></i>
                                </span>
                            </div>
                            <input id="email" type="email" name="email" placeholder="Adresse mail" class="form-control bg-white border-left-0 border-md" v-model="login">
                        </div>

                        <!-- Password -->
                        <div class="input-group col-lg-6 mb-4">
                            <div class="input-group-prepend">
                                <span class="input-group-text bg-white px-4 border-md border-right-0">
                                    <i class="fa fa-lock text-muted"></i>
                                </span>
                            </div>
                            <input id="password" type="password" name="password" placeholder="Votre mot de passe" class="form-control bg-white border-left-0 border-md" v-model="passwd">
                        </div>

                        <!-- Password Confirmation -->
                        <div class="input-group col-lg-6 mb-4">
                            <div class="input-group-prepend">
                                <span class="input-group-text bg-white px-4 border-md border-right-0">
                                    <i class="fa fa-lock text-muted"></i>
                                </span>
                            </div>
                            <input id="passwordConfirmation" type="text" name="passwordConfirmation" placeholder="Confirmez votre mot de passe" class="form-control bg-white border-left-0 border-md">
                        </div>

                        <!-- Submit Button -->
                        <div class="form-group col-lg-12 mx-auto mb-0 text-center">
                            <a @click="connection()" href="#" class="btn btn-primary btn-block py-2">
                                <span class="font-weight-bold">Créer votre compte</span>
                            </a>
                        </div>

                        <!-- Divider Text -->
                        <div class="form-group col-lg-12 mx-auto d-flex align-items-center my-4">
                            <div class="border-bottom w-100 ml-5"></div>
                            <span class="px-2 small text-muted font-weight-bold text-muted">Ou</span>
                            <div class="border-bottom w-100 mr-5"></div>
                        </div>

                        <!-- Already Registered -->
                        <div class="text-center w-100">
                            <p class="text-muted font-weight-bold"> Déjà un compte ? <a @click="swapComponent('connexion')" href="#" class="text-primary ml-2">Connexion</a></p>
                        </div>

                    </div>
                </form>
            </div>
        </div>
    </div>
</template>

<script>
import {defineComponent} from "vue";
import {mapActions} from "vuex";
import {Constants} from "@/constants/Constants";
import AuthActionTypes from "@/store/auth/auth-action-types";

export default defineComponent({
  name: "Signin",
  data(){
    return {
      currentComponent: 'connexion',
      login: "",
      passwd: ""
    }
  },
  methods: {
    ...mapActions(Constants.AUTH_STORE, [AuthActionTypes.TRY_CONNECTION]),
    connection(){
      this[AuthActionTypes.TRY_CONNECTION]({login: this.login, password: this.passwd});
    },
    swapComponent: function(component)
    {
      console.log(component)
      this.currentComponent = component;
    }
  }
})
</script>