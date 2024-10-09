<template>
  <div class="toolbar">
    <div class="toolbar-arrow">
      <span v-if="currentCategoryFilter!==''"
            class="pi pi-arrow-up"
            @click="loadPreviousCategoryReport()">
      </span>
      <span v-else class="pi pi-arrow-up" style="color: lightgrey" ></span>
    </div>
    <div class="toolbar-eye">
      <span :class="(privacy) ? 'pi pi-eye' : 'pi pi-eye-slash'"
            @click="togglePrivacy()">
      </span>
    </div>
  </div>

  <div class="pie-chart">
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

  <div class="category-table">
    <div v-if="!loaded">
      Loading...
    </div>
    <div v-else>
      <template v-for="spendingEntry in categoriesSpending" v-bind:key="spendingEntry.category">
        <CategorySpendingOverview :entry=spendingEntry
                                  :privacy=privacy
                                  @categoryClick="(category) => loadByCategoryReport('EXPENSE',category + '.')"
                                  @transactionsClick="(category) => showTransactionsByCategory(category)"
        >
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
import ChartDataLabels from 'chartjs-plugin-datalabels'
import 'primeicons/primeicons.css'

ChartJS.register(ArcElement, Tooltip, Legend, ChartDataLabels)

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
      privacy: true,
      currentCategoryFilter: "",
      categoriesSpending: [],
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
    togglePrivacy() {
      this.privacy = !this.privacy;
    },
    loadPreviousCategoryReport() {
      if (this.currentCategoryFilter === "") {
        return;
      }

      this.loadByCategoryReport("EXPENSE", this.currentCategoryFilter.substring(
          0, this.currentCategoryFilter.length - 4
      ));
    },
    showTransactionsByCategory(categoryFilter) {
      this.$router.push({
        path: '/transactions',
        query: {
          category: categoryFilter
        }
      });
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