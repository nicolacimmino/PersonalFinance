<template>
  <div class="transactionDetails">
    <div>
      {{ transaction.category }}
    </div>
    <div>
      {{ transaction.description }}
    </div>
    <div class="priceCol">
      {{ transaction.amount_cents / 100.0 }} {{ transaction.currency }}
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
  grid-template: 10px / 1fr 3fr 1fr;
  text-align: left;
  padding: 5px;
  overflow: clip;
}

.priceCol {
  text-align: right;
}

</style>