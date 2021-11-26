import axios from "axios";
import FilesActionTypes from "@/store/files/files-action-types";
import { Constants } from "@/constants/Constants";
import FilesMutationTypes from "@/store/files/files-mutation-types";
import { Module } from "vuex";
import crypto from "crypto-js";

interface State {
  listFiles: any[];
}

const filesModules: Module<any, any> = {
  namespaced: true,
  state: () => ({
    listFiles: [
      { "id": "", "name": "Tg morgan", "type" : "pdf", "date": "29-10-2020", "taille" : "120" },
      { "id": "", "name": "Tg hedwin", "type" : "png", "date": "03-02-2021", "taille" : "37" },
      { "id": "", "name": "Rust c de la merde", "type" : "pdf", "date": "13-02-2019" , "taille" : "79"},
    ]
  }),
  actions: {
    async [FilesActionTypes.GET_LIST_NAMES]({ commit, rootState }: any): Promise<void> {
            await axios
                .get(`${Constants.WEB_URL}/${Constants.FILES_NAMES}`, {headers: {'authorization' : rootState.auth.token}})
                .then((response: any) => {
                    commit(FilesMutationTypes.UPDATE_FILES_NAMES, response.data);
                })
                .catch((error: any) => {
                    console.error(error);
                });
    },
    async [FilesActionTypes.UPLOAD_FILE](
      { commit, rootState, dispatch },
      file: File
    ): Promise<void> {
        const formData = new FormData();
        formData.append("file", file);
        console.log('file-upload',file);
      await axios
        .post(`${Constants.WEB_URL}/${Constants.UPLOAD}`, formData, {headers: {'authorization' : rootState.auth.token, 'content-type': 'multipart/form-data'}})
        .then(() => {
          dispatch(FilesActionTypes.GET_LIST_NAMES);
        })
        .catch((error: any) => {
          console.error(error);
        });
    },
    async [FilesActionTypes.DOWNLOAD_FILE](
      { commit, rootState },
      file: any
    ): Promise<void> {
        if (rootState.auth.key.length === 4){
            await axios
                .get(`${Constants.WEB_URL}/${Constants.DOWNLOAD}/${file.id}`, {headers: {'authorization' : rootState.auth.token}, responseType:'blob'})
                .then((response: any) => {
                    const fileURL = window.URL.createObjectURL(new Blob([response.data]));
                    const fileLink = document.createElement('a');

                    fileLink.href = fileURL;
                    fileLink.setAttribute('download', file.name);
                    document.body.appendChild(fileLink);

                    fileLink.click();
                })
                .catch((error: any) => {
                    console.error(error);
                });
        }else{
            await axios
                .get(`${Constants.WEB_URL}/${Constants.DOWNLOAD}/${file.id}`, {headers: {'authorization' : rootState.auth.token}})
                .then((response: any) => {
                    const fileURL = window.URL.createObjectURL(new Blob([response.data]));
                    const fileLink = document.createElement('a');

                    fileLink.href = fileURL;
                    fileLink.setAttribute('download', file.name);
                    document.body.appendChild(fileLink);

                    fileLink.click();
                })
                .catch((error: any) => {
                    console.error(error);
                });
        }
    },
  },
  mutations: {
    [FilesMutationTypes.UPDATE_FILES_NAMES](
      state: State,
      filesNames: string[]
    ) {
      state.listFiles = filesNames;
    },
  },
};

export default filesModules;
