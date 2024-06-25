import axios from "axios";

export default class TransactionApi {
    static async getAll(apiKey) {

        return axios({
            headers: {
                "X-API-KEY": apiKey
            },
            method: "get",
            url: "http://127.0.0.1:8000/api/transactions/",
        }).then((response) => {
            return response.data;
        });

    }
}