<template>
  <div>
    <div v-if="existingApiKey === ''">
      Please enter your API key.
      <input type="text" v-model="newApiKey"> <input type="button" value="Save" v-on:click="saveApiKey()">
    </div>
    <div v-else>
      Welcome!
    </div>
  </div>
</template>


<script>

export default {
  name: 'HomeView',
  methods: {
    saveApiKey() {
      localStorage.setItem("pfinanceApiKey", this.newApiKey)
      this.existingApiKey = this.newApiKey;
    }
  },
  mounted() {
    this.existingApiKey = localStorage.getItem("pfinanceApiKey") || ""
  },
  data() {
    return {
      newApiKey: "",
      existingApiKey: ""
    }
  }
}
</script>
