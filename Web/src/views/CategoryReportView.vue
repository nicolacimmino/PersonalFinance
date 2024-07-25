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
        <template v-for="spendingEntry in categoriesSpending" v-bind:key="spendingEntry.category">
          <CategorySpendingOverview :entry=spendingEntry>
          </CategorySpendingOverview>
        </template>
      </div>
    </div>
</template>

<script>
import CategorySpendingOverview from "@/components/CategorySpendingOverview.vue";
import TransactionApi from "@/TransactionsApi.ts";
import {Pie} from 'vue-chartjs'
import {Chart as ChartJS, ArcElement, Tooltip, Legend} from 'chart.js'

ChartJS.register(ArcElement, Tooltip, Legend)

export default {
  components: {
    CategorySpendingOverview: CategorySpendingOverview,
    Pie: Pie
  },
  mounted() {
    this.loadByCategoryReport("EXPENSE")
  },
  data() {
    return {
      loaded: false,
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
    loadByCategoryReport(type) {
      TransactionApi.loadByCategoryReportAggregateFirstLevel().then(fetchedCategoriesSpending => {
        this.categoriesSpending = fetchedCategoriesSpending.filter(item => {
          return item.type === type
        });

        this.chartData = {
          labels: this.categoriesSpending.map(
              item => {
                return item.category
              }
          ),
          datasets: [{
            backgroundColor: this.categoriesSpending.map(
                item => {
                  return "#" + item.color
                }
            ),
            data: this.categoriesSpending.map(
                item => {
                  return item.total_cents
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