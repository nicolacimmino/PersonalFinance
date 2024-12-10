<template>
  <div class="alert-details">
    <div class="alert-level">
      <span v-if="alert.level==='ERROR'" class='pi pi-exclamation-triangle'/>
      <span v-if="alert.level==='WARN'" class='pi pi-exclamation-circle'/>
    </div>
    <div class="alert-message pf-text-medium">
      <div v-if="alert.item==='TRANSACTIONS'">
        <a :href='"/transactions?edit_id=" + alert.item_id + "&edit_return=/"'>
          <span v-html="formatPlainTextAlert(alert.message)"></span>
        </a>
      </div>
      <div v-else-if="alert.item==='BUDGETS'">
        <a href="/budgets">
          <span v-html="formatPlainTextAlert(alert.message)"></span>
        </a>
      </div>
      <div v-else>
        <span v-html="formatPlainTextAlert(alert.message)"></span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
      </div>
    </div>
  </div>
</template>

<script>

import Alert from "@/models/alert.ts"

export default {
  props: {
    alert: Alert,
  },
  methods: {
    formatPlainTextAlert(message) {
      return message.replace(":", ":<br>")
    }
  }
}
</script>

<style scoped>
.alert-details {
  display: grid;
  grid-template: 'level message';
  grid-template-columns: 3fr 18fr;
  padding: 10px;
  margin-bottom: 2px;
  background-color: var(--color-background);
  color: var(--color-text);
}

.alert-level {
  grid-area: level;
}

.alert-message {
  grid-area: message;
  text-align: left;
}
</style>