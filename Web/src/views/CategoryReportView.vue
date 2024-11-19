<template>
  <ToolBar
      v-bind:upEnabled="upArrowEnabled"
      @privacy="(newPrivacy) => onPrivacyChange(newPrivacy)"
      @arrow-up="loadPreviousCategoryReport()"
      :eye-enabled="true"
  />
  <div class="pf-tabs">
    <div class="pf-tab" :class="(this.type==='EXPENSE') ? 'pf-tab-selected' : 'pf-tab-inactive'"
         @click="this.$router.push({ path: '/category_report', query: {type:'EXPENSE'}})"
    >
      <p class=" pf-text-large">Expense</p>
    </div>
    <div class="pf-tab" :class="(this.type==='INCOME') ? 'pf-tab-selected' : 'pf-tab-inactive'"
         @click="this.$router.push({ path: '/category_report', query: {type:'INCOME'}})"
    >
      <p class=" pf-text-large">Income</p>
    </div>
  </div>
  <div class="pie-chart">
    <Pie
        id="report-by-category"
        :options="chartOptions"
        :data="chartData"
    />
  </div>
  <div>
    <template v-for="entry in categories" v-bind:key="entry.category">
      <div>
        <CategorySpendingOverview :entry=entry
                                  :privacy=privacy
                                  @categoryClick="(category) => loadByCategoryReport(this.type,category + '.')"
                                  @transactionsClick="(category) => showTransactionsByCategory(category)"
        >
        </CategorySpendingOverview>
      </div>
    </template>
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
    Pie: Pie,
  },
  mounted() {
    this.privacy = (localStorage.getItem("privacy") === "true")
    this.loadByCategoryReport(this.type, "")
  },
  props: {
    type: {
      type: String,
      default: "EXPENSE"
    },
  },
  computed: {
    upArrowEnabled() {
      return (this.currentCategoryFilter !== "");
    }
  },
  watch: {
    $route: function () {
      this.loadByCategoryReport(this.type, "")
    }
  },
  data() {
    return {
      loaded: false,
      privacy: true,
      currentCategoryFilter: "",
      categories: [],
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

      this.loadByCategoryReport(this.type, this.currentCategoryFilter.substring(
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
        ).sort((a, b) => (a.total_cents > b.total_cents) ? 1 : -1);

        if (aggregatedData.length === 0) {
          this.loaded = true;
          return;
        }

        this.currentCategoryFilter = categoryFilter;

        this.categories = aggregatedData;

        this.chartData = {
          labels: this.categories.map(
              item => {
                return item.category
              }
          ),
          datasets: [{
            backgroundColor:
                ['#00429d', '#3e67ae', '#618fbf', '#85b7ce', '#b1dfdb', '#ffcab9', '#fd9291', '#e75d6f', '#c52a52', '#93003b']
            ,
            data: this.categories.map(
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
  padding: 15px;
  margin-bottom: 15px;
  background-color: #E9B87222;
}

</style>