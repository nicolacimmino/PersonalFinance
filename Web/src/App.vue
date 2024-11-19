<template>
  <div class="mainContainer">
    <div class="main">
      <main>
        <RouterView/>
      </main>
    </div>
  </div>
</template>

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
  },
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

html, body {
  margin: 0;
  padding: 0;
  height: 100%;
  font-size: 100%;
  font-family: Poppins, sans-serif;
}

.pf-text-large {
  font-size: 1em;
}

.pf-text-medium {
  font-size: 0.8em;
}

.pf-text-small {
  font-size: 0.6em;
}

.navigation-link {
  font-size: 2em;
}
</style>