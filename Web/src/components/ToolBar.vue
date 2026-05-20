<template>
  <div class="toolbar-container">
    <div class="menu-bar toolbar-icon">
      <i class="pi pi-bars"
         style="color: var(--color-icon-active)"
         @click="showMenu=true"
      >
        &nbsp;
      </i>
    </div>
    <div class="ref-currency toolbar-icon">
      <i :class="(refCurrency) ? 'pi pi-money-bill' : 'pi pi-euro'"
         :style="(refCurrencyEnabled) ? 'color: var(--color-icon-active)' : 'color: var(--color-icon-inactive)'"
         @click="handleToggleRefCurrency()"
      >
        &nbsp;
      </i>
    </div>
    <div class="arrow-up toolbar-icon">
      <i class="pi pi-arrow-up"
         :style="(upEnabled) ? 'color: var(--color-icon-active)' : 'color: var(--color-icon-inactive)'"
         @click="$emit('arrow-up')"
      >
        &nbsp;
      </i>
    </div>
    <div class="maximize toolbar-icon">
      <i :class="(compact) ? 'pi pi-window-maximize' : 'pi pi-window-minimize'"
         :style="(compactEnabled) ? 'color: var(--color-icon-active)' : 'color: var(--color-icon-inactive)'"
         @click="handleToggleCompact()"
      >
        &nbsp;
      </i>
    </div>
    <div class="eye toolbar-icon">
      <i :class="(privacy) ? 'pi pi-eye' : 'pi pi-eye-slash'"
         :style="(eyeEnabled) ? 'color: var(--color-icon-active)' : 'color: var(--color-icon-inactive)'"
         @click="handleTogglePrivacy()"
      >
        &nbsp;
      </i>
    </div>
  </div>
  <div v-if="showMenu" class="navigation">
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
      <div>
        <RouterLink class="navigation-link" to="/indicators">Indicators</RouterLink>
      </div>
      <hr>
      <div>
        <select
            id="viewYear"
            :value="year"
            @change="(e) => { changeYear(e); showMenu=false; }"
        >
          <option v-for="y in years" :key="y" :value="y">
            {{ y }}
          </option>
        </select>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import 'primeicons/primeicons.css'
import { useSettings } from '@/composables';

export default {
  props: {
    upEnabled: Boolean,
    compactEnabled: Boolean,
    refCurrencyEnabled: Boolean,
    eyeEnabled: Boolean,
  },
  setup() {
    const settings = useSettings()
    return {
      privacy: settings.privacy,
      compact: settings.compact,
      refCurrency: settings.refCurrency,
      year: settings.year,
      togglePrivacy: settings.togglePrivacy,
      toggleCompact: settings.toggleCompact,
      toggleRefCurrency: settings.toggleRefCurrency,
      setYear: settings.setYear
    }
  },
  computed: {
    years() {
      const startYear = 2024;
      const currentYear = new Date().getFullYear();
      const list = [];

      for (let y = startYear; y <= currentYear; y++) {
        list.push(y);
      }

      return list;
    }
  },
  emits: ['ref-currency', 'arrow-up', 'compact', 'privacy', 'changeYear'],
  data() {
    return {
      showMenu: false,
    }
  },
  methods: {
    handleTogglePrivacy() {
      if (!this.eyeEnabled) {
        return;
      }
      this.togglePrivacy();
      this.$emit('privacy', this.privacy);
    },
    handleToggleCompact() {
      if (!this.compactEnabled) {
        return;
      }
      this.toggleCompact();
      this.$emit('compact', this.compact);
    },
    handleToggleRefCurrency() {
      if (!this.refCurrencyEnabled) {
        return;
      }
      this.toggleRefCurrency();
      this.$emit('ref-currency', this.refCurrency);
    },
    changeYear(event?: Event) {
      if (event) {
        const target = event.target as HTMLSelectElement;
        this.setYear(parseInt(target.value, 10));
      }
      this.$emit('changeYear', this.year);
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
  background-color: var(--color-background);
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