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

import {useApiKeyStore} from "@/stores/apiKeyStore.ts";

export default {
  name: 'HomeView',
  methods: {
    saveApiKey() {
      console.log("TEST")
      console.log(this.newApiKey)
      useApiKeyStore().setApiKey(this.newApiKey);
      this.existingApiKey = this.newApiKey
    }
  },
  mounted() {
    this.existingApiKey = useApiKeyStore().getApiKey();
  },
  data() {
    return {
      newApiKey: "",
      existingApiKey: ""
    }
  }
}
</script>
