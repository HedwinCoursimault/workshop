import axios from "axios";
import FilesActionTypes from "@/store/files/files-action-types";
import { Constants } from "@/constants/Constants";
import FilesMutationTypes from "@/store/files/files-mutation-types";
import { Module } from "vuex";

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
    async [FilesActionTypes.GET_LIST_NAMES]({ commit }: any): Promise<void> {
        if (Constants.WEB_URL !== ""){
            await axios
                .get(`${Constants.WEB_URL}/${Constants.FILES_NAMES}`)
                .then((response: any) => {
                    console.log(response);
                    commit(FilesMutationTypes.UPDATE_FILES_NAMES, response);
                })
                .catch((error: any) => {
                    console.error(error);
                });
        }
    },
    async [FilesActionTypes.UPLOAD_FILE](
      { commit },
      file: Blob
    ): Promise<void> {
      await axios
        .post(`${Constants.WEB_URL}/${Constants.UPLOAD}`, file)
        .then(() => {
          console.log("success");
        })
        .catch((error: any) => {
          console.error(error);
        });
    },
    async [FilesActionTypes.DOWNLOAD_FILE](
      { commit },
      fileName: string
    ): Promise<void> {
      await axios
        .get(`${Constants.WEB_URL}/${Constants.DOWNLOAD}/${fileName}`)
        .then((response: any) => {
          const url = window.URL.createObjectURL(response);
          const a = document.createElement("a");
          a.style.display = "none";
          a.href = url;
          // the filename you want
          a.download = "todo-1.json";
          document.body.appendChild(a);
          a.click();
          window.URL.revokeObjectURL(url);
        })
        .catch((error: any) => {
          console.error(error);
        });
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
