<template>
  <div>
    <div>
      <div>
        {{ entry.category }} {{ Math.abs(entry.total_cents / 100.0) }} {{ entry.currency }}
      </div>
    </div>
    <div class="transactionDetails">
      <div>
        Transactions: {{ entry.transactions_count }}
      </div>
    </div>
  </div>

</template>

<script>

export default {
  props: {
    entry: Object
  },
}
</script>
