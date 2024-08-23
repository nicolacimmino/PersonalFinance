<template>
  <div>
    <div class="modal-mask">
      <div class="modal-wrapper">
        <div class="modal-container">
          <div class="modal-header">
            <slot name="header">
              {{ transaction.booking_date }}
            </slot>
          </div>

          <div class="modal-body">
            <slot name="body">
              <select v-model="selectedCategory">
                <option v-for="category in categories" v-bind:key="category" v-bind:value="category">{{
                    category
                  }}
                </option>
              </select>
            </slot>
          </div>

          <div class="modal-footer">
            <slot name="footer">
              &nbsp;
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
export default {
  mounted() {
    this.selectedCategory = this.transaction.category
  },
  props: {
    transaction: undefined,
    categories: undefined,
  },
  data() {
    return {
      selectedCategory: String
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
</style>