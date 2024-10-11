<template>
  <div class="mainContainer">
    <div class="navigation">
      <div class="navbar">
        <div>
          <RouterLink to="/">Home</RouterLink>
        </div>
        <div>
          <RouterLink to="/transactions">Transactions</RouterLink>
        </div>
        <div>
          <RouterLink to="/category_report">Categories</RouterLink>
        </div>
        <div>
          <RouterLink to="/budgets">Budgets</RouterLink>
        </div>
        <div>
          <RouterLink to="/accounts">Accounts</RouterLink>
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
  name: 'App'
}
</script>

<style>

.navbar {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  font-size: small;
  text-align: center;
}
.navigation {
  grid-area: navigation;
}

.main {
  grid-area: main;
}

.mainContainer {
  background-color: white;
  font-family: monospace;
  grid-template:
    'navigation'
    'main';
}

html, body {
  margin: 0;
  padding: 0;
  height: 100%;
}

</style>