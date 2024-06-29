<template>
  <div class="transactionDetails">
    <div v-on:click="editDialog=true">
      {{ transaction.category }}
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
.transactionDetails {
  display: grid;
  grid-template: 50px / 3fr 2fr;
  text-align: left;
  padding: 15px;
}
</style>