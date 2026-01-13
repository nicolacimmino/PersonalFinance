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
const envName = import.meta.env.VITE_ENV
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
  background-color: white;
}

.main {
  grid-area: main;
}

.mainContainer {
  background-color: white;
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