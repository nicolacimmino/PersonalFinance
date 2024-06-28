<template>
  Details<br>
  Id: {{ id }}<br>
  APIKEY: {{ apiKey }}<br>

</template>

<script>
import {useApiKeyStore} from "@/stores/apiKeyStore.ts";

export default {
  props: {
    id: String,
  },
  computed: {
    apiKey() {
      return useApiKeyStore().getApiKey();
    }
  }
}
</script>

