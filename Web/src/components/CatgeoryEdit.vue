<template>
  <div v-if="loading">
    Loading...
  </div>
  <div v-else>
    <div class="modal-mask">
      <div class="modal-wrapper">
        <div class="modal-container">
          <div class="modal-header">
            <slot name="header">
              Edit Category
            </slot>
          </div>

          <div class="modal-body">
            <slot name="body">
              <select v-model="selectedCategory">
                <option v-for="category in categories" v-bind:key="category" v-bind:value="category">{{ category }}</option>
              </select>
            </slot>
          </div>

          <div class="modal-footer">
            <slot name="footer">
              Choose
              <button class="modal-default-button" @click="$emit('save', selectedCategory)">
                Save
              </button>
              <button class="modal-default-button" @click="$emit('cancel')">
                Cancel
              </button>
            </slot>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import TransactionApi from "@/TransactionsApi.ts";

export default {
  mounted() {
    this.loadAllCategories()
    this.selectedCategory = this.category
  },
  props: {
      category: String
  },
  data() {
    return {
      loading: false,
      categories: [],
      selectedCategory: String
    }
  },
  methods: {
    loadAllCategories() {
      this.loading = true;
      TransactionApi.getCategories().then(fetchedCategories => {
        this.categories = fetchedCategories;
        this.loading = false;
      });
    },
    onTransactionClick(id) {
      this.$router.push({name: "transaction_details", params: {id: id}})
    }
  },
  computed: {
    byDate() {
      return this.transactions.reduce((acc, transaction) => {
        (acc[transaction.booking_date] = acc[transaction.booking_date] || []).push(transaction)
        return acc
      }, {})
    }
  }
}
</script>

<style scoped>
.modal-mask {
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

.modal-wrapper {
  display: table-cell;
  vertical-align: middle;
}

.modal-container {
  width: 300px;
  margin: 0px auto;
  padding: 20px 30px;
  background-color: #fff;
  border-radius: 2px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.33);
  transition: all 0.3s ease;
  font-family: Helvetica, Arial, sans-serif;
}

.modal-header h3 {
  margin-top: 0;
  color: #42b983;
}

.modal-body {
  margin: 20px 0;
}

.modal-default-button {
  float: right;
}

/*
 * The following styles are auto-applied to elements with
 * transition="modal" when their visibility is toggled
 * by Vue.js.
 *
 * You can easily play with the modal transition by editing
 * these styles.
 */

.modal-enter {
  opacity: 0;
}

.modal-leave-active {
  opacity: 0;
}

.modal-enter .modal-container,
.modal-leave-active .modal-container {
  -webkit-transform: scale(1.1);
  transform: scale(1.1);
}
</style>