<template>
  <nav>
    <RouterLink to="/">Home</RouterLink>
    <RouterLink to="/transactions">Transactions</RouterLink>
  </nav>

  <main class="mainContainer">
    <RouterView/>
  </main>

</template>

<script>


export default {
  name: 'App'
}
</script>

<style>
.mainContainer {
  background-color: whitesmoke;
  padding: 20px;
  width: 80%;
}
</style>