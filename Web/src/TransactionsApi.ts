import axios from "axios";
import Alert from "@/models/alert.ts";

const HOST = import.meta.env.VITE_HOST;

export default class TransactionApi {
    static async getAllTransactions(account_id, category) {
        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "get",
            url: `${HOST}/api/transactions/?category=${category}&account=${account_id}`,
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

    static async loadKpis() {
        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "get",
            url: `${HOST}/api/reports/kpis`,
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

    static async updateTransactionCategory(id, category, type, description) {
        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "patch",
            url: `${HOST}/api/transactions/${id}`,
            data: {
                type: type,
                category: category,
                description: description
            }
        }).then((response) => {
            return response.data;
        });

    }

    static async createTransaction(newTransaction) {
        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "post",
            url: `${HOST}/api/transactions`,
            data: {
                type: newTransaction.type,
                account_id: newTransaction.account.id,
                booking_date: newTransaction.booking_date,
                category: newTransaction.category,
                creditor_name: newTransaction.creditor_name,
                description: newTransaction.description,
                amount_cents: Math.round(newTransaction.amount * 100.0),
                account_to: (newTransaction.destination_account) ? newTransaction.destination_account.id : null
            }
        }).then((response) => {
            return response.data;
        }).catch(() => {
            return "ERROR";
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

    static async loadAllAlerts(): Promise<Alert[]> {
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