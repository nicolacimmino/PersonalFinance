<template>
  <div class="toolbar-container">
    <div class="ref-currency">
      <i :class="(refCurrency) ? 'pi pi-euro' : 'pi pi-money-bill'"
         @click="$emit('ref-currency')">
      </i>
    </div>
    <div class="arrow-up">
      <i class="pi pi-arrow-up"
         @click="$emit('arrow-up')"
         :style="(up) ? 'color:black' : 'color:grey'">
      </i>
    </div>
    <div class="maximize">
      <i :class="(maximize) ? 'pi pi-window-maximize' : 'pi pi-window-minimize'"
         @click="$emit('maximize')">
      </i>
    </div>
    <div class="eye">
      <i :class="(privacy) ? 'pi pi-eye' : 'pi pi-eye-slash'"
         @click="$emit('privacy')">
      </i>
    </div>
  </div>
</template>

<script>
import 'primeicons/primeicons.css'

export default {
  props: {
    privacy: Boolean,
    maximize: Boolean,
    up: Boolean,
    refCurrency: Boolean
  }
}
</script>


<style scoped>
.toolbar-container {
  display: grid;
  grid-template: "none ref-currency arrow-up maximize eye";
  grid-template-columns: [none] 6fr [ref-currency] 1fr [arrow-up] 1fr [maximize] 1fr [eye] 1fr;
  padding: 0px;
  margin-bottom: 10px;
  margin-top: 10px;
  background-color: white;
  font-size: 1em;
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