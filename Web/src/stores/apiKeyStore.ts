import {defineStore} from 'pinia'

export const useApiKeyStore = defineStore('apiKey', () => {
    let apiKey = "";

    function setApiKey(ak: String) {
        apiKey = ak;
    }

    function getApiKey(): String {
        return apiKey;
    }

    return {setApiKey, getApiKey}
})
