<template>
  <div class="alert-details">
    <div class="alert-level">
      <span v-if="alert.level==='ERROR'" class='pi pi-exclamation-triangle' style="color: red"/>
      <span v-if="alert.level==='WARN'" class='pi pi-exclamation-circle' style="color: darkgoldenrod"/>
    </div>
    <div class="alert-area">
      {{ alert.area }}
    </div>
    <div class="alert-message">
      <div v-if="alert.item=='TRANSACTIONS'">
        <a :href='"/transactions?edit_id=" + alert.item_id + "&edit_return=/"'>
          {{ alert.message }}
        </a>
      </div>
      <div v-else>
        {{ alert.message }}
      </div>
    </div>
  </div>
</template>

<script>

export default {
  props: {
    alert: Object,
  }
}
</script>

<style scoped>
.alert-details {
  display: grid;
  grid-template: 'level area'
                 'message message';
  padding: 0px;
  margin-bottom: 2px;
  background-color: #E9B87222;
}

.alert-level {
  grid-area: level;
}

.alert-area {
  grid-area: area;
  font-size: 20px;
  text-align: left;
}

.alert-message {
  grid-area: message;
  font-size: 20px;
  text-align: left;
}
</style>