import axios from "axios";
import {useApiKeyStore} from "@/stores/apiKeyStore.ts";

export default class TransactionApi {
    static async getAll() {
        return axios({
            headers: {
                "X-API-KEY": useApiKeyStore().getApiKey()
            },
            method: "get",
            url: "http://127.0.0.1:8000/api/transactions/",
        }).then((response) => {
            return response.data;
        });

    }
}