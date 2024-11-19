<template>
  <div class="toolbar-container">
    <div class="menu-bar toolbar-icon">
      <i class="pi pi-bars"
         style="color:black"
         @click="showMenu=true"
      >
        &nbsp;
      </i>
    </div>
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
  <div v-if="showMenu" class="navigation" @click="showMenu=false">
    <div class="navbar">
      <div>
        <RouterLink class="navigation-link" to="/">Home</RouterLink>
      </div>
      <div>
        <RouterLink class="navigation-link" to="/transactions">Transactions</RouterLink>
      </div>
      <div>
        <RouterLink class="navigation-link" to="/category_report">Categories</RouterLink>
      </div>
      <div>
        <RouterLink class="navigation-link" to="/budgets">Budgets</RouterLink>
      </div>
      <div>
        <RouterLink class="navigation-link" to="/accounts">Accounts</RouterLink>
      </div>
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
      showMenu: false
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
  grid-template: "menu-bar ref-currency arrow-up maximize eye";
  grid-template-columns: [ref-currency] 1fr [arrow-up] 1fr [maximize] 1fr [eye] 1fr;
  padding: 0px;
  margin-bottom: 20px;
  margin-top: 20px;
  background-color: white;
  text-align: center;
  /*position: sticky;*/
  /*top: 0;*/
}

.toolbar-icon {
  font-size: 1.5em;
}

.menu-bar {
  grid-area: menu-bar;
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

.navigation {
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

</style>