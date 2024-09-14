import axios from "axios";

const HOST = import.meta.env.VITE_HOST;

export default class TransactionApi {
    static async getAllTransactions(account_id, category) {
        let url = `${HOST}/api/transactions/?category=${category}`;
        if (account_id) {
            url = `${HOST}/api/accounts/${account_id}/transactions/`;
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
            url: `${HOST}/api/accounts/${id}`,
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
            url: `${HOST}/api/transactions/${id}`,
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
            url: `${HOST}/api/reports/by_category/`,
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
            url: `${HOST}/api/accounts/`,
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
            url: `${HOST}/api/transactions/${id}`,
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
            url: `${HOST}/api/categories/`,
        }).then((response) => {
            return response.data;
        });

    }
}