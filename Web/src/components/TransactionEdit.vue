<template>
  <div>
    <div class="mask">
      <div class="wrapper">
        <div class="container">
          <div class="creditor pf-text-large">
            {{ transaction.creditorName }}
          </div>
          <div :class="(transaction.amountCents < 0) ? 'negative-price' : 'non-negative-price'">
            {{ formatMoney(transaction.amountCents) }} {{ transaction.currency }}
          </div>
          <!--          Edit category or destination account-->
          <div v-if="!this.isTransfer">
            <div v-if="this.editCategory" class="category">
              <select v-model="selectedCategory" @change="this.editCategory=false">
                <option v-for="category in categories" v-bind:key="category" v-bind:value="category">{{
                    category
                  }}
                </option>
              </select>
            </div>
            <div v-else @click="this.editCategory=true" class="category pf-text-large">
              {{ (selectedCategory) ? selectedCategory : '???' }}
            </div>
          </div>
          <div v-else>
            <div v-if="this.editDestinationAccount" class="destinationAccount pf-text-large">
              <select v-model="selectedDestinationAccount" @change="this.editDestinationAccount=false">
                <option v-for="account in accounts" v-bind:key="account" v-bind:value="account">{{
                    account.description
                  }}
                </option>
              </select>
            </div>
            <div v-else @click="this.editDestinationAccount=true" class="destinationAccount">
              {{ (selectedDestinationAccount) ? selectedDestinationAccount.description : '???' }}
            </div>
          </div>

          <div class="account-name pf-text-medium">
            {{ transaction.accountName }}
          </div>
          <div class="amount-in-ref-currency pf-text-medium">
            {{ formatMoney(transaction.amountCents_in_refCurrency) }} {{ transaction.refCurrency }}
          </div>
          <div class="booking-date pf-text-medium">
            {{ moment(transaction.bookingDate).format('DD-MM-YYYY') }}
          </div>
          <div v-if="editDescription" class="description pf-text-large">
            <textarea v-model="this.editedDescription" />
          </div>
          <div v-else class="description pf-text-large" @click="this.editDescription=true">
            {{ transaction.description }}
          </div>
          <div class="transfer">
            <label for="transfer-toggle">Transfer</label>
            <VueToggle id="transfer-toggle"
                       :value="this.transaction.type==='TRANSFER'"
                       :height="20"
                       :width="30"
                       checkedText=""
                       uncheckedText=""
                       checkedBg="#b4d455"
                       uncheckedBg="lightgrey"
                       @click="isTransfer = !isTransfer" />

          </div>
          <div class="transaction-id">
            {{ transaction.id }}
          </div>
          <div class="footer">
            <button class="button-cancel"
                    @click="$emit('cancel')">
              Cancel
            </button>
            <button class="button-save"
                    @click="$emit('save', {
                      category: selectedCategory,
                      accountTo: (selectedDestinationAccount && isTransfer) ? selectedDestinationAccount.id : null,
                      isTransfer: isTransfer,
                      description: editedDescription,
                       })"
                    :disabled="!saveEnabled()">
              Save
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import moment from 'moment/moment.js'
import { VueToggles } from 'vue-toggles'
import { formatMoney } from '@/utils/format'

export default {
  mounted() {
    this.selectedCategory = this.transaction.category
    this.selectedDestinationAccount = this.accounts.find((account) => account.id === this.transaction.accountTo)
    this.isTransfer = this.transaction.type === 'TRANSFER'
    this.editedDescription = this.transaction.description
  },
  props: {
    transaction: undefined,
    categories: undefined,
    accounts: undefined
  },
  data() {
    return {
      selectedCategory: String,
      editCategory: false,
      isTransfer: Boolean,
      selectedDestinationAccount: Object,
      editDestinationAccount: false,
      editDescription: false,
      editedDescription: String
    }
  },
  components: {
    VueToggle: VueToggles
  },
  methods: {
    moment: moment,
    formatMoney,
    saveEnabled() {
      if (this.selectedCategory !== this.transaction.category) {
        return true
      }

      if (this.editedDescription !== this.transaction.description) {
        return true
      }

      if (this.selectedDestinationAccount && this.selectedDestinationAccount.id !== this.transaction.accountTo) {
        return true
      }

      if (this.isTransfer && this.isTransfer !== (this.transaction.type === 'TRANSFER')) {
        return true
      }

      return false
    }
  }
}
</script>

<style scoped>
.mask {
  position: fixed;
  z-index: 9998;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: var(--color-overlay);
  display: table;
  transition: opacity 0.3s ease;
}

.wrapper {
  /*padding: 20px;*/
  width: 95%;
  margin: auto auto auto auto;
  /*display: table-cell;*/
  /*vertical-align: center;*/
  /*horiz-align: center;*/
  /*padding: 40px;*/
  /*background-color: red;*/
}

.container {
  margin-top: 20px;
  margin-left: auto;
  margin-right: auto;
  padding: 20px;
  row-gap: 5px;
  background-color: var(--color-background);
  border-radius: 10px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.33);
  transition: all 0.3s ease;
  display: grid;
  grid-template:
    'booking-date transaction-id'
    'creditor creditor'
    'category amount'
    'account-name amount-ref'
    'description description'
    'transfer transfer'
    'footer footer';
}

.transfer {
  padding-top: 10px;
  vertical-align: bottom;
}

.transfer label {
  padding-right: 5px;
  float: left;
}

.creditor {
  grid-area: creditor;
  padding-top: 10px;
  padding-bottom: 5px;
}

.description {
  grid-area: description;
  padding-top: 10px;
}

.transaction-id {
  grid-area: transaction-id;
  text-align: right;
  padding-top: 10px;
}

.booking-date {
  grid-area: booking-date;
  text-align: left;
  padding-top: 10px;
}

.account-name {
  grid-area: account-name;
  text-align: left;
  color: grey;
}

.amount-in-ref-currency {
  grid-area: amount-ref;
  text-align: right;
  color: grey;
}

.negative-price {
  grid-area: amount;
  text-align: right;
  vertical-align: bottom;
  font-weight: bold;
  color: #A63D40;
}

.non-negative-price {
  grid-area: amount;
  text-align: right;
  font-weight: bold;
  color: #90A959;
}

.category {
  grid-area: category;
  text-align: left;
  font-weight: bold;
}

.destinationAccount {
  grid-area: category;
  text-align: left;
  font-weight: bold;
}

.footer {
  grid-area: footer;
}

.button-save {
  float: right;
  margin: 5px;
  padding: 5px;
  background-color: var(--color-button-neutral);
  color: var(--color-text);
  border: 1px;
  border-radius: 5px;
}

.button-cancel {
  float: right;
  margin: 5px;
  padding: 5px;
  background-color: var(--color-button-positive);
  color: var(--color-text);
  border: 1px;
  border-radius: 5px;
}

</style>