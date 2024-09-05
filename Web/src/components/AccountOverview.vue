<template>
  <div>
    <div>
      <div>
        {{ account.description }}
      </div>
    </div>
    <div>
      {{ account.currency }}
    </div>
    <div>
      {{ account.iban }}
    </div>
    <div>
      {{ account.status }}
    </div>
  </div>

</template>

<script>

export default {
  props: {
    account: Object
  },
}
</script>
