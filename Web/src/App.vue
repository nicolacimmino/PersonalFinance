<template>
  <h1>Test</h1>
  <p>
    <strong>Current route path:</strong> {{ $route.fullPath }}<br>
  </p>

  <nav>
    <RouterLink to="/">Home</RouterLink>
    <RouterLink to="/transactions">Transactions</RouterLink>
    <RouterLink to="/transactions/123">T123</RouterLink>
  </nav>

  <main>
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