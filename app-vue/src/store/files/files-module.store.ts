import axios from "axios";
import FilesActionTypes from "@/store/files/files-action-types";
import {Constants} from "@/constants/Constants";
import FilesMutationTypes from "@/store/files/files-mutation-types";

interface State {
    listFiles: string[];
}

const filesModules = {
    namespaced: true,
    state: () => ({
        listFiles: []
    }),
    actions: {
        [FilesActionTypes.GET_LIST_NAMES]({commit}: any): Promise<any>{
            return axios.get(`${Constants.WEB_URL}/${Constants.FILES_NAMES}`)
                .then((response: any) => {
                    console.log(response);
                    commit(FilesMutationTypes.UPDATE_FILES_NAMES, response);
                }
            ).catch((error: any)=> {
                    console.error(error);
                });
        }
    },
    mutations: {
        [FilesMutationTypes.UPDATE_FILES_NAMES](state: State, filesNames: string[]){
            state.listFiles = filesNames;
        }
    }

}

export default filesModules;