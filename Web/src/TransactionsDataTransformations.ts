export default class TransactionsDataTransformations {
    static aggregateSubLevels(reports, typeFilter, categoryFilter) {
        let result = [];

        reports.filter(item => {
            return item.type == typeFilter;
        }).filter(item => {
            return item.category.startsWith(categoryFilter);
        }).reduce(function (res, report) {
            const aggregation_category = report.category.substring(0, categoryFilter.length + 3)

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

        return result;
    }

}