<template>
  <v-app>
    <v-main>
      <FileBrowser/>
    </v-main>
  </v-app>
</template>

<script>
import FileBrowser from './components/FileBrowser.vue'

export default {
  name: 'App',

  components: {
    FileBrowser
  },

  data: () => ({
    //
  }),
}
</script>
