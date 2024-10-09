<template>
  <div class="mainContainer">
    <div class="header">
      <nav>
        <RouterLink to="/">Home</RouterLink>
        <RouterLink to="/transactions">Transactions</RouterLink>
        <RouterLink to="/category_report">Categories</RouterLink>
        <RouterLink to="/budgets">Budgets</RouterLink>
        <RouterLink to="/accounts">Accounts</RouterLink>
      </nav>
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

.header {
  grid-area: header;
}

.main {
  grid-area: main;
}

.mainContainer {
  background-color: white;
  font-family: monospace;
  width: 100%;
  grid-template:
    'header'
    'main';
}


</style>