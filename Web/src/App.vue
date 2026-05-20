<template>
  <div class="mainContainer">
    <div class="main">
      <main>
        {{ envName }}
        <RouterView />
      </main>
    </div>
  </div>
</template>

<script setup>
import { config } from '@/config';
const envName = config.envName;
</script>

<script>
export default {
  name: 'App',
  data() {
    return {
      showMenu: false
    }
  },
  watch: {
    $route: function () {
      this.showMenu = false
    }
  }
}
</script>

<style>
.navbar {
  text-align: left;
  padding: 10px;
}

.navbar Div {
  padding: 10px;
  background-color: var(--color-background);
}

.main {
  grid-area: main;
}

.mainContainer {
  background-color: var(--color-background);
  grid-template:
    'navigation'
    'main';
}

html,
body {
  margin: 0;
  padding: 0;
  height: 100%;
  font-size: 100%;
  font-family: Poppins, sans-serif;
}

.navigation-link {
  font-size: 2em;
}
</style>