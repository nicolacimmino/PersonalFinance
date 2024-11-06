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

    static async getAccounts() {
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

    static async loadBudgets() {
        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "get",
            url: `${HOST}/api/budgets/`,
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

    static async updateTransactionCategory(id, category, type) {
        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "patch",
            url: `${HOST}/api/transactions/${id}`,
            data: {
                type: type,
                category: category
            }
        }).then((response) => {
            return response.data;
        });

    }

    static async updateTransactionAccountTo(id, account_to) {
        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "patch",
            url: `${HOST}/api/transactions/${id}`,
            data: {
                type: "TRANSFER",
                account_to: account_to
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

    static async loadAllAlerts() {
        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "get",
            url: `${HOST}/api/alerts/`,
        }).then((response) => {
            return response.data;
        });

    }
}