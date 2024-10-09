<template>
  <div class="budget-details">
    <div class="description">
      {{ budget.description }}
    </div>
    <div class="budget_spent">
      Spent: {{ Math.floor(Math.abs(budget.spent_cents / 100.0)) }} {{ budget.currency }}
      ( of {{ Math.floor(Math.abs(budget.amount_cents / 100.0)) }} )
    </div>
    <div class="budget_dates">
      {{ budget.from_date }} - {{ budget.to_date }}
    </div>
    <div class="budget_transactions">
      Transactions: {{ budget.transactions }}
    </div>
    <div class="budget_graph">
      <Pie id="{{budget.id}}"
           :options="chartOptions(budget)"
           :data="chartData(budget)"
      />
    </div>
  </div>

</template>

<script>
import 'primeicons/primeicons.css'
import {Pie} from 'vue-chartjs'
import {Chart as ChartJS, ArcElement, Tooltip, Legend} from 'chart.js'
import ChartDataLabels from 'chartjs-plugin-datalabels'
import 'primeicons/primeicons.css'

ChartJS.register(ArcElement, Tooltip, Legend, ChartDataLabels)

export default {
  components: {
    Pie: Pie
  },
  props: {
    budget: Object,
  },
  methods: {
    chartOptions(budget) {
      return {
        responsive: true,
        layout: {
          padding: 0,
        },
        plugins: {
          datalabels: {
            formatter: (value, ctx) => {
              if(ctx.chart.data.datasets[0].data.length === 1) {
                return null
              }
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
      }
    },
    isBudgetExceeded(budget) {
      return budget.spent_cents > budget.amount_cents;
    },
    isBudgetCloseToMax(budget) {
      return budget.spent_cents > budget.amount_cents * 0.75;
    },
    chartData(budget) {
      let colors = ["#09ed55", "#CCCCCC"];

      if (this.isBudgetCloseToMax(budget)) {
        colors = ["#edd609", "#CCCCCC"]
      }

      if (this.isBudgetExceeded(budget)) {
        colors = ["#f79188"]
      }

      return {
        datasets: [{
          backgroundColor: colors,
          data: [
            Math.floor(Math.abs(budget.spent_cents / 100.0)),
            (budget.spent_cents < budget.amount_cents) ?
                Math.floor(Math.abs(budget.amount_cents / 100.0) - Math.abs(budget.spent_cents / 100.0)) : null,
          ].filter(item => {
            return item !== null
          })
        }]
      }
    }
  }
}
;
</script>

<style scoped>
.budget-details {
  display: grid;
  grid-template: 'description description'
                 'budget_spent budget_graph'
                 'budget_dates budget_graph'
                 'budget_transactions budget_graph';
  padding: 5px;
  margin-bottom: 5px;
  background-color: #E9B87222;
}

.description {
  grid-area: description;
  text-align: left;
  font-size: medium;
  font-weight: bold;
  background: #2c3e50;
  color: white;
}

.budget_transactions {
  grid-area: budget_transactions;
  font-size: x-small;
}

.budget_dates {
  grid-area: budget_dates;
  font-size: small;
}

.budget_spent {
  grid-area: budget_spent;
  font-size: small;
}

.budget_graph {
  grid-area: budget_graph;
  width: 90px;
  align-content: center;
}

</style>
