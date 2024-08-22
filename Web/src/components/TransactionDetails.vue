<template>
  <div class="transaction-details">
    <div>
      <a v-on:click="editDialog=true">{{ transaction.category }}</a>
    </div>
    <div>
      {{ transaction.amount_cents / 100.0 }} {{ transaction.currency }}
    </div>
    <div>
      {{ transaction.description }}
    </div>

    <transition name="modal">
      <TransactionEdit :category="transaction.category" v-if="editDialog" @cancel="editDialog = false"
                       @save="(newCategory) => onCategoryChange(newCategory)">
      </TransactionEdit>
    </transition>
  </div>
</template>

<script>
import TransactionEdit from "@/components/CatgeoryEdit.vue";

export default {
  components: {
    TransactionEdit: TransactionEdit,
  },
  props: {
    transaction: Object
  },
  data() {
    return {
      editDialog: false
    }
  },
  methods: {
    onCategoryChange(newCategory) {
      this.editDialog = false;
      console.log(newCategory);
      this.$emit("updateCategory", newCategory);
    }
  }
}
</script>

<style scoped>
.transaction-details {
  display: grid;
  grid-template: 200px / 1fr 1fr 3fr;
  text-align: left;
  padding: 15px;
  font-family: "Courier New", monospace;
}
</style>