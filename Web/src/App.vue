<template>
  <div class="mainContainer">
    <div class="menu_button pi pi-bars" v-on:click="showMenu=true">
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
      </div>
    </div>
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

.menu_button {
  position: fixed;
  top: 30px;
  left: 30px;
  /*border-radius: 50%;*/
  /*background-color: var(--pf-c-dark-gray);*/
  /*width: 50px;*/
  /*color: white;*/
}
</style>