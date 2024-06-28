<template>
  <div class="transactionRow">
    <div class="transactionHeader">
      <div>
        {{ transaction.category }}
      </div>
      <div>
        {{ transaction.amount_cents / 100.0 }} {{ transaction.currency }}
      </div>
    </div>
    <div class="transactionDetails">
      <div>
        {{ transaction.description }}
      </div>
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
.transactionRow {
  display: grid;
  width: 800px;
  grid-template: 50px / 1fr;
  text-align: left;
  border-bottom: 1px solid lightsteelblue;
  padding: 15px;
}

.transactionHeader {
  display: grid;
  grid-template: 50px / 3fr 2fr;
  text-align: left;
  padding: 15px;
}

.transactionDetails {
  display: grid;
  grid-template: 50px / 3fr 2fr;
  text-align: left;
  padding: 15px;
}

.transactionRow:nth-last-child(1) {
  border-bottom: none;
}

</style>