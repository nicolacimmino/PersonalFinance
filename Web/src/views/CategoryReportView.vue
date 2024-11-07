<template>
  <ToolBar
      v-bind:upEnabled="upArrowEnabled"
      @privacy="(newPrivacy) => onPrivacyChange(newPrivacy)"
      @arrow-up="loadPreviousCategoryReport()"
      :eye-enabled="true"
  />
  <div v-if="!loaded">
    Loading...
  </div>
  <div v-else>
    <div class="pie-chart">
      <Pie
          id="report-by-category"
          :options="chartOptions"
          :data="chartData"
      />
    </div>
    <div class="category-overview">
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
import ToolBar from "@/components/ToolBar.vue";
import TransactionApi from "@/TransactionsApi.ts";
import TransactionsDataTransformations from "@/TransactionsDataTransformations.ts";
import {Pie} from 'vue-chartjs'
import {Chart as ChartJS, ArcElement, Tooltip, Legend} from 'chart.js'
import ChartDataLabels from 'chartjs-plugin-datalabels'
import 'primeicons/primeicons.css'

ChartJS.register(ArcElement, Tooltip, Legend, ChartDataLabels)

export default {
  components: {
    ToolBar,
    CategorySpendingOverview: CategorySpendingOverview,
    Pie: Pie
  },
  mounted() {
    this.privacy = (localStorage.getItem("privacy") === "true")
    this.loadByCategoryReport("EXPENSE", "")
  },
  computed: {
    upArrowEnabled() {
      return (this.currentCategoryFilter !== "");
    }
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
    onPrivacyChange(newPrivacy) {
      this.privacy = newPrivacy
    },
    loadPreviousCategoryReport() {
      if (this.currentCategoryFilter === "") {
        this.loaded = true;
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
      this.loaded = false;
      TransactionApi.loadByCategoryReport().then(fetchedCategoriesReport => {
        let aggregatedData = TransactionsDataTransformations.aggregateSubLevels(
            fetchedCategoriesReport.reports, typeFilter, categoryFilter
        );

        if (aggregatedData.length === 0) {
          this.loaded = true;
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
  padding: 10px;
  margin-bottom: 15px;
  background-color: #E9B87222;
}

.category-overview {

}
</style>