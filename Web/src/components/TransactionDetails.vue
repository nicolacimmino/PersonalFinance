<template>
  <div class="transactionDetails">
    <div>
      {{ transaction.category }}
    </div>
    <div>
      {{ transaction.amount_cents / 100.0 }} {{ transaction.currency }}
    </div>
    <div>
      {{ transaction.description }}
    </div>
  </div>
</template>

<script>

export default {
  props: {
    transaction: Object
  },
}
</script>

<style scoped>
.transactionDetails {
  display: grid;
  grid-template: 50px / 3fr 2fr;
  text-align: left;
  padding: 15px;
}
</style>