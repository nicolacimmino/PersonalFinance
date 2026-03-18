export default class TransactionsDataTransformations {
    static aggregateSubLevels(reports, typeFilter, categoryFilter) {
        let result = [];

        reports.filter(item => {
            return item.type == typeFilter;
        }).filter(item => {
            return item.category.startsWith(categoryFilter);
        }).reduce(function (res, report) {
            const filterTokens = categoryFilter.split(".").filter(item => {
                return item !== ""
            });
            const categoryTokens = report.category.split(".");
            const aggregation_category = categoryTokens.slice(0, filterTokens.length + 1).join(".");

            if (res[aggregation_category] === undefined) {
                res[aggregation_category] = {
                    category: aggregation_category,
                    totalCents: 0,
                    currency: report.currency,
                    color: report.color,
                    type: report.type,
                    transactionCount: 0,
                    subcategories: []
                };
                result.push(res[aggregation_category])
            }

            res[aggregation_category].totalCents += report.totalCents;
            res[aggregation_category].transactionCount += report.transactionCount;
            res[aggregation_category].subcategories.push(report);

            return res;
        }, {});

        result = Object.keys(result).map(function (k) {
            return result[k];
        });

        result = result.sort((a, b) => (a.category > b.category) ? 1 : ((b.category > a.category) ? -1 : 0))

        return result;
    }

}