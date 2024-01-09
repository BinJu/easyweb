<template>
  <v-container fluid class="ma-2">
    <v-row class="child-flex">
      <div>
        <v-toolbar>
          <v-dialog>
            <template #activator="{ props }">
              <v-btn v-bind="props" text="New Folder" prepend-icon="mdi-folder" class="text-none"><template v-slot:prepend><v-icon color="green"></v-icon></template></v-btn>
            </template>
            <template #default="{ isActive }">
              <v-sheet>
                <v-label text="Create a folder" class="ma-4"/>
                <v-form ref="form" v-model="valid" fast-fail @submit.prevent>
                  <v-text-field
                      v-model="createFolder"
                      :rules="createFolderRules"
                      label="The Folder Name"
                  ></v-text-field>
                  <v-btn :disabled="!valid" type="submit" block class="mt-2" @click="this.newFolder(this.locations[this.locations.length-1].path + '/' + this.createFolder);isActive.value = false">Create</v-btn> 
                </v-form>
              </v-sheet>
            </template>
          </v-dialog>
          <!--v-btn prepend-icon="mdi-arrow-up" density="compact" class="text-none"><template v-slot:prepend><v-icon color="green"></v-icon></template>Upload</v-btn-->
          <v-dialog>
            <template #activator="{ props }">
              <v-btn v-bind="props" text="Upload" prepend-icon="mdi-arrow-up" class="text-none"><template v-slot:prepend><v-icon color="green"></v-icon></template></v-btn>
            </template>
            <template #default="{ isActive }">
              <v-sheet>
                <v-label text="Upload a file" class="ma-4"/>
                <v-form fast-fail @submit.prevent>
                  <v-file-input
                      label="Please select file to upload"
                      v-model="fileName"
                  ></v-file-input>
                  <v-btn type="submit" block class="mt-2" @click="uploadFile(fileName, 'upload');isActive.value = false">Upload</v-btn> 
                </v-form>
              </v-sheet>
            </template>
          </v-dialog>
        </v-toolbar>
      </div>
    </v-row>
    <v-row>
      <v-breadcrumbs :items="locations">
        <template #item="{item}"><v-btn variant="text" class="text-none" @click="openFolder('', item.path)"> {{ item.title }} </v-btn></template>
      </v-breadcrumbs>
    </v-row>
    <v-row>
      <v-container class="ma-5 d-xs-none">
        <v-row class="child-flex">
          <v-tooltip
            location="top center"
            origin="auto"
            no-click-animation
            v-for="(item, i) in items" :key="i">
            <template v-slot:activator="{ props }">
              <v-btn v-bind="props" :stacked="true" :key="i" variant="text" size="default" :prepend-icon="item.type == 'd' ? 'mdi-folder' : 'mdi-file'" class="text-none" @click.prevent="actItem(item)">
                <template #prepend><v-icon color="blue"></v-icon></template>
                <span class="item-text">{{ item.title }}</span>
               </v-btn>
            </template>
            <div>{{ item.title }}</div>
          </v-tooltip>
        </v-row>
      </v-container>
    </v-row>
  </v-container>
</template>

<script>
  import Axios from 'axios'
  export default {
    name: 'FileBrowser',
    data: () => ({
            valid: false,
            items: [],
            createFolder: "",
            // eslint-disable-next-line
            createFolderRules: [ v => !!v || "Folder name is required", v => /^[a-zA-Z][a-zA-Z0-9_\-]*$/.test(v) || 'Folder starts with letters' ],
            fileName: [],
            locations: []
    }),
    mounted() {
      let self = this;
      fetch("dir").then(function(response) {
        return response.json();
      }).then(function(data) {
        self.items = data
      })
    },
    methods: {
      newFolder(title) {
        fetch("new_dir?p=" + title, {method: 'POST'})
        //this.items.push({title: title, type: "f", path: "./"})
        this.createFolder = ""
        let curPath = this.locations[this.locations.length-1].path
        this.openFolder('', curPath)
      },
      actItem(item) {
        if (item.type == 'd') {
          this.openFolder(item.title, item.path)
        } else {
          this.downloadItem(item.title, item.path)
        }
      },
      downloadItem(fileName, path) {
        Axios.get("file?f=" + path, {headers: {}, responseType: 'blob'})
          .then(response => {
            const blob = new Blob([response.data], { type: 'application/plain' })
            const link = document.createElement('a')
            link.href = URL.createObjectURL(blob)
            link.download = fileName
            link.click()
            URL.resoveObjectURL(link.href)
          }).catch(console.error)
        //https://stackoverflow.com/questions/53772331/vue-html-js-how-to-download-a-file-to-browser-using-the-download-tag
      },
      openFolder(fileName, path) {
        let self = this;
        fetch("dir?p=" + path).then(function(response) {
          return response.json();
        }).then(function(data) {
          self.items = data
        })
        //Update locations
        let paths = []
        let lastPath = ""
        let temp_paths = path.split("/")
        for (let p in temp_paths) {
          paths.push({title: temp_paths[p], "path": lastPath + temp_paths[p]})
          lastPath += temp_paths[p] + "/"
        }
        self.locations = paths
      },
      async uploadFile(file, uri) {
        let curPath = this.locations[this.locations.length-1].path
        let formData = new FormData()
        formData.append('file', file[0])
        formData.append('path', curPath)
        let response = await fetch(uri, { method: 'POST', body: formData })
        this.openFolder('', curPath)
        return response
      }
  
    }
  }
</script>
<style scoped>
.item-text {
    width: 80px;
    height: 40px;
    overflow:hidden;
    text-overflow: ellipsis;
}
</style>
