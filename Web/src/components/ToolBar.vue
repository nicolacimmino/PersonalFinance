<template>
  <div class="toolbar-container">
    <div class="ref-currency toolbar-icon">
      <i :class="(refCurrency) ? 'pi pi-money-bill' : 'pi pi-euro'"
         :style="(refCurrencyEnabled) ? 'color:black' : 'color:#EEEEEE'"
         @click="toggleRefCurrency(); $emit('ref-currency', this.refCurrency)"
      >
        &nbsp;
      </i>
    </div>
    <div class="arrow-up toolbar-icon">
      <i class="pi pi-arrow-up"
         :style="(upEnabled) ? 'color:black' : 'color:#EEEEEE'"
         @click="$emit('arrow-up')"
      >
        &nbsp;
      </i>
    </div>
    <div class="maximize toolbar-icon">
      <i :class="(compact) ? 'pi pi-window-maximize' : 'pi pi-window-minimize'"
         :style="(compactEnabled) ? 'color:black' : 'color:#EEEEEE'"
         @click="toggleCompact(); $emit('compact', this.compact)"
      >
        &nbsp;
      </i>
    </div>
    <div class="eye toolbar-icon">
      <i :class="(privacy) ? 'pi pi-eye' : 'pi pi-eye-slash'"
         :style="(eyeEnabled) ? 'color:black' : 'color:#EEEEEE'"
         @click="togglePrivacy(); $emit('privacy', this.privacy);"
      >
        &nbsp;
      </i>
    </div>
  </div>
</template>

<script>
import 'primeicons/primeicons.css'

export default {
  props: {
    upEnabled: Boolean,
    compactEnabled: Boolean,
    refCurrencyEnabled: Boolean,
    eyeEnabled: Boolean
  },
  data() {
    return {
      privacy: (localStorage.getItem("privacy") === "true"),
      compact: (localStorage.getItem("compact") === "true"),
      refCurrency: (localStorage.getItem("refCurrency") === "true"),
    }
  },
  methods: {
    togglePrivacy() {
      if (!this.eyeEnabled) {
        return;
      }
      this.privacy = !this.privacy
      localStorage.setItem("privacy", this.privacy.toString());
    },
    toggleCompact() {
      if (!this.compactEnabled) {
        return
      }

      this.compact = !this.compact
      localStorage.setItem("compact", this.compact.toString());
    },
    toggleRefCurrency() {
      if (!this.refCurrencyEnabled) {
        return;
      }

      this.refCurrency = !this.refCurrency
      localStorage.setItem("refCurrency", this.refCurrency.toString());
    },
  }
}
</script>


<style scoped>
.toolbar-container {
  display: grid;
  grid-template: "ref-currency arrow-up maximize eye";
  grid-template-columns: [ref-currency] 1fr [arrow-up] 1fr [maximize] 1fr [eye] 1fr;
  padding: 0px;
  margin-bottom: 10px;
  margin-top: 10px;
  background-color: white;
  text-align: center;
  /*position: sticky;*/
  /*top: 0;*/
}

.toolbar-icon {
  font-size: 1.5em;
}

.ref-currency {
  grid-area: ref-currency;
}

.arrow-up {
  grid-area: arrow-up;
}

.maximize {
  grid-area: maximize;
}

.eye {
  grid-area: eye;
}
</style>