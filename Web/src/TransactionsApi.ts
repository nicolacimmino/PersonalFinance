import axios from "axios";

export default class TransactionApi {
    static async getAllTransactions(account_id) {
        console.log("HERE" + account_id)
        let url = "http://127.0.0.1:8000/api/transactions/";
        if (account_id) {
            url = `http://127.0.0.1:8000/api/accounts/${account_id}/transactions/`;
        }

        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "get",
            url: url,
        }).then((response) => {
            return response.data;
        });

    }

    static async getAccount(id) {
        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "get",
            url: "http://127.0.0.1:8000/api/accounts/" + id,
        }).then((response) => {
            return response.data;
        });
    }

    static async getTransaction(id) {
        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "get",
            url: "http://127.0.0.1:8000/api/transactions/" + id,
        }).then((response) => {
            return response.data;
        });

    }

    static async loadByCategoryReport() {
        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "get",
            url: "http://127.0.0.1:8000/api/reports/by_category/",
        }).then((response) => {
            return response.data;
        });

    }

    static async loadAllAccounts() {
        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "get",
            url: "http://127.0.0.1:8000/api/accounts/",
        }).then((response) => {
            return response.data;
        });

    }

    static async updateTransactionCategory(id, category) {
        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "patch",
            url: "http://127.0.0.1:8000/api/transactions/" + id,
            data: {
                category: category
            }
        }).then((response) => {
            return response.data;
        });

    }

    static async getCategories() {
        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "get",
            url: "http://127.0.0.1:8000/api/categories/",
        }).then((response) => {
            return response.data;
        });

    }
}