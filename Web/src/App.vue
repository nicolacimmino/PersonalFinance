<template>
  <div class="mainContainer">
    <div class="header">
      <nav>
        <RouterLink to="/">Home</RouterLink>
        <RouterLink to="/transactions">Transactions</RouterLink>
        <RouterLink to="/category_report">Categories</RouterLink>
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
  background-color: whitesmoke;
  padding: 20px;
  width: 80%;
  display: grid;
  grid-template:
    'header'
    'main';
}
</style>