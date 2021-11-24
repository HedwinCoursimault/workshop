import AuthMutationTypes from "@/store/auth/auth-mutation-types";
import axios from "axios";
import AuthActionTypes from "@/store/auth/auth-action-types";
import {Constants} from "@/constants/Constants";

interface State {
    isConnected: boolean;
    key: string;
    token: string;
}

const authModule = {
    namespaced: true,
    state: () => ({
        isConnected: true,
        key: "",
        token: ""
    }),
    actions: {
        async [AuthActionTypes.TRY_CONNECTION]({commit}: any, auth: any){
            await axios.post(`${Constants.WEB_URL}/${Constants.CONNECTION}`,auth)
                .then((response: any) => {
                    commit(AuthMutationTypes.UPDATE_CONNECITON, true);
                    commit(AuthMutationTypes.UPDATE_KEY, "");
                    commit(AuthMutationTypes.UPDATE_TOKEN, response);
                }).catch((error: any) => {
                    commit(AuthMutationTypes.UPDATE_CONNECITON, false);
                    console.error(error)
                });
        }
    },
    mutations: {
        [AuthMutationTypes.UPDATE_CONNECITON](state: State, isConnected: boolean){
            state.isConnected = isConnected;
        },
        [AuthMutationTypes.UPDATE_KEY](state: State, key: string){
            state.key = key;
        },
        [AuthMutationTypes.UPDATE_TOKEN](state: State, token: string){
            state.token =`Bearer ${token}`
        }
    }
}

export default authModule;