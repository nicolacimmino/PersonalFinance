<template>
<!--  <div class="pie-chart">-->
<!--    <div v-if="!loaded">-->
<!--      Loading...-->
<!--    </div>-->
<!--    <div v-else>-->
<!--      <Pie v-if="loaded "-->
<!--           id="report-by-category"-->
<!--           :options="chartOptions"-->
<!--           :data="chartData"-->
<!--      />-->
<!--    </div>-->
<!--  </div>-->

  <div class="budgets-table">
    <div v-if="!loaded">
      Loading...
    </div>
    <div v-else>
      <template v-for="budget in activeBudgets" v-bind:key="budget.id">
        <BudgetOverview :budget=budget></BudgetOverview>
      </template>
    </div>
  </div>
</template>

<script>
import CategorySpendingOverview from "@/components/CategorySpendingOverview.vue";
import TransactionApi from "@/TransactionsApi.ts";
import {Pie} from 'vue-chartjs'
import {ArcElement, Chart as ChartJS, Legend, Tooltip} from 'chart.js'
import ChartDataLabels from 'chartjs-plugin-datalabels'
import 'primeicons/primeicons.css'
import BudgetOverview from "@/components/BudgetOverview.vue";

ChartJS.register(ArcElement, Tooltip, Legend, ChartDataLabels)

export default {
  components: {
    BudgetOverview,
    Pie: Pie
  },
  mounted() {
    this.loadByCategoryReport("EXPENSE", "")
  },
  data() {
    return {
      loaded: false,
      privacy: true,
      currentCategoryFilter: "",
      activeBudgets: [],
      chartOptions: {
        responsive: true,
        maintainAspectRatio: false,
        tooltips: {
          enabled: false
        },
        plugins: {
          datalabels: {
            formatter: (value, ctx) => {
              let sum = 0;
              ctx.chart.data.datasets[0].data.map(data => {
                sum += data;
              });
              return (value * 100 / sum).toFixed(2) + "%";
            },
            color: '#333333',
            backgroundColor: '#DDDDDDAE',
            borderRadius: 10,
            anchor: 'end'
          }
        }
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
    showTransactionsByCategory(categoryFilter) {
      this.$router.push({
        path: '/transactions',
        query: {
          category: categoryFilter
        }
      });
    },
    loadByCategoryReport() {
      TransactionApi.loadBudgets().then(fetchedBudgets => {
        this.activeBudgets = fetchedBudgets.filter(item => {
          return item.active === true
        });

        this.chartData = {
          labels: this.activeBudgets.map(
              item => {
                return item.description
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
            data: this.activeBudgets.map(
                item => {
                  return Math.abs(item.spent_cents_in_ref_currency) / 100.00
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

.pie-chart {
  display: grid;
  padding: 0px;
  margin-bottom: 2px;
  background-color: #E9B87222;
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

.toolbar {
  display: grid;
  grid-template: "none none none none none none none none none none arrow eye";
  padding: 0px;
  margin-bottom: 10px;
  margin-top: 10px;
  background-color: white;
}

.toolbar-arrow {
  grid-area: arrow;
}

.toolbar-eye {
  grid-area: eye;
}
</style>