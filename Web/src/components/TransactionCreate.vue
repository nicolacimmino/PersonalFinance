<template xmlns="http://www.w3.org/1999/html">
  <div>
    <div class="mask">
      <div class="wrapper">
        <div class="container">
          <ul class="input-list">
            <li>
              {{ transactionType }}
            </li>
            <li>
              <label for="transfer-toggle">Transfer</label>
              <VueToggle id="transfer-toggle"
                         v-model="transfer"
                         :height="20"
                         :width="30"
                         checkedText=""
                         uncheckedText=""
                         checkedBg="#b4d455"
                         uncheckedBg="lightgrey"/>
            </li>
            <li>
              <label for="creditor">Creditor</label>
              <input id="creditor" v-model="transaction.creditor_name">
            </li>

            <li>
              <label for="amount">Amount</label>
              <input id="amount" v-model="transaction.amount">
            </li>

            <li>
              <label for="account">Account</label>
              <select id="account" v-model="transaction.account">
                <option v-for="account in canCreateTransactionAccounts" v-bind:key="account" v-bind:value="account">{{
                    account.description
                  }}
                </option>
              </select>
            </li>

            <li v-if="!this.transfer">
              <label for="category">Category</label>
              <select id="category" v-model="transaction.category">
                <option v-for="category in categories" v-bind:key="category" v-bind:value="category">
                  {{ category }}
                </option>
              </select>
            </li>

            <li v-if="this.transfer">
              <label for="contra-account">Contra Account</label>
              <select id="contra-account" v-model="transaction.destination_account">
                <option v-for="account in accounts" v-bind:key="account" v-bind:value="account">
                  {{ account.description }}
                </option>
              </select>
            </li>

            <li>
              <label for="booking-date">Date</label>
              <input id="booking-date" v-model="transaction.booking_date">
            </li>

            <li>
              <label for="description">Description</label>
              <textarea id="description" v-model="transaction.description"/>
            </li>
          </ul>

          <div class="footer">
            <button class="button-cancel"
                    @click="$emit('cancel')">
              Cancel
            </button>
            <button class="button-save"
                    @click="transaction.type=transactionType; $emit('save', transaction);"
            >
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
    this.transfer = false;
    this.transaction.booking_date = moment(new Date()).format("YYYY-MM-DD");
    this.transaction.description = "";
    this.transaction.amount = "0";
    this.transaction.type = "EXPENSE";
  },
  props: {
    categories: undefined,
    accounts: undefined
  },
  data() {
    return {
      transfer: Boolean,
      transaction: {
        description: String,
        booking_date: moment(new Date()).format("YYYY-MM-DD"),
        creditor_name: "",
        amount: Number,
        category: String,
        account: Object,
        destination_account: Object,
        type: String,
      }
    }
  },
  components: {
    VueToggle: VueToggles,
  },
  methods: {
    moment: moment,
  },
  computed: {
    canCreateTransactionAccounts() {
      return this.accounts.filter(item => {
        return item.can_create_transactions === "1";
      })
    },
    transactionType() {
      if (this.transfer) {
        return "TRANSFER";
      }
      if (this.transaction.amount < 0) {
        return "EXPENSE";
      }

      return "INCOME";
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
}

label {
  display: inline-block;
  width: 120px;
}

.footer {
  grid-area: footer;
  padding: 10px;
}

.button-save {
  float: right;
  margin: 5px;
  padding: 5px;
  background-color: lightgreen;
  border: 1px;
  border-radius: 5px;
}

.button-cancel {
  float: right;
  margin: 5px;
  padding: 5px;
  background-color: lightcoral;
  border: 1px;
  border-radius: 5px;
}

ul.input-list {
  list-style-type: none;
  margin: 0;
}

ul.input-list li {
  padding: 5px;
}

ul.input-list li input {
  width: 200px;
}

ul.input-list li textarea {
  width: 200px;
}

ul.input-list li select {
  width: 205px;
}

</style>