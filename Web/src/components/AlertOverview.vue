<template>
  <div class="alert-details">
    <div class="alert-level">
      <span v-if="alert.level==='ERROR'" class='pi pi-exclamation-triangle' style="color: red"/>
      <span v-if="alert.level==='WARN'" class='pi pi-exclamation-circle' style="color: darkgoldenrod"/>
    </div>
    <div class="alert-message pf-text-medium">
      <div v-if="alert.item==='TRANSACTIONS'">
        <a :href='"/transactions?edit_id=" + alert.item_id + "&edit_return=/"'>
          <span v-html="formatPlainTextAlert(alert.message)"></span>
        </a>
      </div>
      <div v-else>
        <span v-html="formatPlainTextAlert(alert.message)"></span>
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
  background-color: #E9B87222;
}

.alert-level {
  grid-area: level;
}

.alert-message {
  grid-area: message;
  text-align: left;
}
</style>