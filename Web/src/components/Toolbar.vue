<template>
  <div class="toolbar">
    <div class="toolbar-arrow">
      <i class="pi pi-arrow-up"
         :style="(currentCategoryFilter!=='') ? 'color:black' : 'color:grey'"
         @click="loadPreviousCategoryReport()">
      </i>
    </div>
    <div class="toolbar-eye">
      <i :class="(privacy) ? 'pi pi-eye' : 'pi pi-eye-slash'"
         @click="togglePrivacy()">
      </i>
    </div>
  </div>
</template>

<script>
import 'primeicons/primeicons.css'

export default {
  data() {
    return {
      privacy: Boolean
    }
  },
  methods: {
    togglePrivacy() {
      this.privacy = !this.privacy;
    }
  }
}
</script>


<style scoped>
.account-details {
  display: grid;
  grid-template:
    'description balance'
    'code balance-ref'
    'iban iban'
    'status status';
  padding: 0px;
  margin-bottom: 2px;
  background-color: #E9B87222;
}

.description {
  grid-area: description;
  font-size: smaller;
}

.status {
  grid-area: status;
  font-size: smaller;
  text-align: right;
}

.code {
  grid-area: code;
  text-align: left;
  font-size: xx-small;
  color: grey;
}

.balance-in-ref-currency {
  grid-area: balance-ref;
  text-align: right;
  font-size: xx-small;
  color: grey;
}

.balance {
  grid-area: balance;
  text-align: right;
}

.balance-ref {
  grid-area: balance-ref;
}

.balance-style {
  text-align: right;
  vertical-align: middle;
  font-size: medium;
  font-weight: bold;
}

.negative-balance {
  color: #A63D40;
}

.non-negative-balance {
  color: #90A959;
}

.iban {
  grid-area: iban;
  text-align: left;
  font-size: small;
}

</style>