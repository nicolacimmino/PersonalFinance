import axios from "axios";

export default class TransactionApi {
    static async getAllTransactions() {
        return axios({
            headers: {
                "X-API-KEY": localStorage.getItem("pfinanceApiKey")
            },
            method: "get",
            url: "http://127.0.0.1:8000/api/transactions/",
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

    static async loadByCategoryReportAggregateFirstLevel() {
        const response = await this.loadByCategoryReport();

        let result = [];

        response.reports.reduce(function (res, report) {
            const aggregation_category = report.category.substring(0, 3)

            if (res[aggregation_category] === undefined) {
                res[aggregation_category] = {
                    category: aggregation_category,
                    total_cents: 0,
                    currency: report.currency,
                    color: report.color,
                    type: report.type
                };
                result.push(res[aggregation_category])
            }

            res[aggregation_category].total_cents += report.total_cents;

            return res;
        }, {});

        result = Object.keys(result).map(function (k) {
            return result[k];
        });

        console.log(result);

        return result;
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