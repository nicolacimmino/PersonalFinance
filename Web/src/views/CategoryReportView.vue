<template>
  <div class="transactionsTable">
    <div v-if="!loaded">
      Loading...
    </div>
    <div v-else>
      <Pie v-if="loaded "
           id="report-by-category"
           :options="chartOptions"
           :data="chartData"
      />
    </div>
  </div>
  <div class="transactionsTable">
    <div v-if="!loaded">
      Loading...
    </div>
    <div v-else>
      <div v-if="currentCategoryFilter!==''">
        <a v-on:click="loadPreviousCategoryReport()">Up</a>
      </div>
      <template v-for="spendingEntry in categoriesSpending" v-bind:key="spendingEntry.category">
        <CategorySpendingOverview :entry=spendingEntry
                                  v-on:click="loadByCategoryReport('EXPENSE',spendingEntry.category + '.')">
        </CategorySpendingOverview>
      </template>
    </div>
  </div>
</template>

<script>
import CategorySpendingOverview from "@/components/CategorySpendingOverview.vue";
import TransactionApi from "@/TransactionsApi.ts";
import TransactionsDataTransformations from "@/TransactionsDataTransformations.ts";
import {Pie} from 'vue-chartjs'
import {Chart as ChartJS, ArcElement, Tooltip, Legend} from 'chart.js'

ChartJS.register(ArcElement, Tooltip, Legend)

export default {
  components: {
    CategorySpendingOverview: CategorySpendingOverview,
    Pie: Pie
  },
  mounted() {
    this.loadByCategoryReport("EXPENSE", "")
  },
  data() {
    return {
      loaded: false,
      currentCategoryFilter: "",
      categoriesSpending: [],
      chartOptions: {
        responsive: true,
        maintainAspectRatio: false
      },
      chartData: {
        labels: [],
        datasets: [{
          backgroundColor: [],
          data: []
        }]
      }
    }
  },
  methods: {
    loadPreviousCategoryReport() {
      if (this.currentCategoryFilter === "") {
        return;
      }

      this.loadByCategoryReport("EXPENSE", this.currentCategoryFilter.substring(
          0, this.currentCategoryFilter.length - 4
      ));
    },
    loadByCategoryReport(typeFilter, categoryFilter) {
      TransactionApi.loadByCategoryReport().then(fetchedCategoriesReport => {
        let aggregatedData = TransactionsDataTransformations.aggregateSubLevels(
            fetchedCategoriesReport.reports, typeFilter, categoryFilter
        );

        if (aggregatedData.length === 0) {
          return;
        }

        this.currentCategoryFilter = categoryFilter;

        this.categoriesSpending = aggregatedData;

        this.chartData = {
          labels: this.categoriesSpending.map(
              item => {
                return item.category
              }
          ),
          datasets: [{
            backgroundColor: [
              "#CDDFA0",
              "#7B9EA8",
              "#E6C79C",
              "#78586F",
              "#6FD08C",
              "#334139",
            ],
            data: this.categoriesSpending.map(
                item => {
                  return Math.abs(item.total_cents) / 100.00
                }
            )
          }]
        }

        this.loaded = true;
      });
    },
  },
}
</script>

<style scoped>
.transactionsTable {
  border-radius: 10px;
  background-color: white;
}

.transactionsHeader {
  border-radius: 10px 10px 0 0;
  display: grid;
  grid-template: 10px / 0.6fr 1.4fr 1fr;
  text-align: left;
  padding: 15px;
  background-color: lightsteelblue;
  text-transform: uppercase;
}
</style>