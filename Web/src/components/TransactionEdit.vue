<template>
  <div>
    <div class="mask">
      <div class="wrapper">
        <div class="container">
          <div class="creditor">
            {{ transaction.creditor_name }}
          </div>
          <div :class="(transaction.amount_cents < 0) ? 'negative-price' : 'non-negative-price'">
            {{ transaction.amount_cents / 100.0 }} {{ transaction.currency }}
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
            <div v-else @click="this.editCategory=true" class="category">
              {{ (selectedCategory) ? selectedCategory : "-" }}
            </div>
          </div>
          <div v-else>
            <div v-if="this.editDestinationAccount" class="destinationAccount">
              <select v-model="selectedDestinationAccount" @change="this.editDestinationAccount=false">
                <option v-for="account in accounts" v-bind:key="account" v-bind:value="account">{{
                    account.description
                  }}
                </option>
              </select>
            </div>
            <div v-else @click="this.editDestinationAccount=true" class="destinationAccount">
              {{ (selectedDestinationAccount) ? selectedDestinationAccount.description : "???" }}
            </div>
          </div>

          <div class="account-name">
            {{ transaction.account_name }}
          </div>
          <div class="amount-in-ref-currency">
            {{ transaction.amount_cents_in_ref_currency / 100.0 }} {{ transaction.ref_currency }}
          </div>
          <div class="booking-date">
            {{ moment(transaction.booking_date).format("DD-MM-YYYY") }}
          </div>
          <div class="description">
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
                       @click="isTransfer = !isTransfer"/>

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
                    @click="$emit('save', selectedCategory, (selectedDestinationAccount) ? selectedDestinationAccount.id : null, isTransfer)"
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
import moment from "moment/moment.js";
import {VueToggles} from "vue-toggles";

export default {
  mounted() {
    this.selectedCategory = this.transaction.category;
    this.selectedDestinationAccount = this.accounts.find((account) => account.id === this.transaction.account_to);
    this.isTransfer = this.transaction.type === "TRANSFER";
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
      editDestinationAccount: false
    }
  },
  components: {
    VueToggle: VueToggles,
  },
  methods: {
    moment: moment,
    saveEnabled() {
      if (this.selectedCategory !== this.transaction.category) {
        return true;
      }

      if (this.selectedDestinationAccount && this.selectedDestinationAccount.id !== this.transaction.account_to) {
        return true
      }

      if (this.isTransfer && this.isTransfer !== (this.transaction.type === "TRANSFER")) {
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
  background-color: rgba(0, 0, 0, 0.5);
  display: table;
  transition: opacity 0.3s ease;
}

.wrapper {
  display: table-cell;
  vertical-align: center;
  padding: 20px;
}

.container {
  margin: 0px auto;
  padding: 30px;
  background-color: mintcream;
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
  padding-top: 5px;
}

.transfer label {
  padding-right: 5px;
  float: left;
}

.creditor {
  grid-area: creditor;
  background-color: #28282822;
  padding: 0 0 0 5px;
  font-size: 10px;
}

.description {
  grid-area: description;
  font-size: 12px;
}

.transaction-id {
  grid-area: transaction-id;
  text-align: right;
  font-size: 12px;
}

.booking-date {
  grid-area: booking-date;
  text-align: left;
  font-size: 12px;
}

.account-name {
  grid-area: account-name;
  text-align: left;
  font-size: 12px;
  color: grey;
  background-color: #28282822;
  padding: 0 0 0 5px;
}

.amount-in-ref-currency {
  grid-area: amount-ref;
  text-align: right;
  font-size: 12px;
  color: grey;
  background-color: #28282822;
  padding: 0 5px 0 0;
}

.negative-price {
  grid-area: amount;
  text-align: right;
  vertical-align: bottom;
  font-size: 15px;
  font-weight: bold;
  color: #A63D40;
  background-color: #28282822;
  padding: 0 5px 0 0;
}

.non-negative-price {
  grid-area: amount;
  text-align: right;
  font-size: 15px;
  font-weight: bold;
  color: #90A959;
  background-color: #28282822;
  padding: 0 5px 0 0;
}

.category {
  grid-area: category;
  text-align: left;
  font-size: 15px;
  font-weight: bold;
  background-color: #28282822;
  padding: 0 0 0 5px;
}

.destinationAccount {
  grid-area: category;
  text-align: left;
  font-size: 15px;
  font-weight: bold;
  background-color: #28282822;
  padding: 0 0 0 5px;
}

.footer {
  grid-area: footer;
}

.button-save {
  float: right;
  margin: 5px;
  padding: 5px;
  background-color: lightgrey;
  border: 1px;
  border-radius: 5px;
  font-family: monospace;
}

.button-cancel {
  float: right;
  margin: 5px;
  padding: 5px;
  background-color: lightgreen;
  border: 1px;
  border-radius: 5px;
  font-family: monospace;
}

</style>