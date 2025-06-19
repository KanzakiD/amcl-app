<template>
  <div data-tauri-drag-region class="amcl-toolbar">
    <span data-tauri-drag-region v-if="!isMacOS" class="amcl-toolbar-title">
      AMCL v{{ appVersion }}
    </span>
    <!-- 新模块暂时留空 -->
    <n-button-group class="amcl-toolbar-button">
      <n-button v-if="!isHomePath()" quaternary @click="backToParent">
        <font-awesome-icon icon="fa-solid fa-arrow-left" />
      </n-button>
      <n-button v-if="isHomePath()" quaternary @click="pushToSettings">
        <font-awesome-icon icon="fa-solid fa-gear" />
      </n-button>
      <n-button v-if="!isMacOS" quaternary @click="minimizeApp">
        <font-awesome-icon icon="fa-solid fa-minus" />
      </n-button>
      <n-button v-if="!isMacOS" quaternary @click="closeApp">
        <font-awesome-icon icon="fa-solid fa-xmark" />
      </n-button>
    </n-button-group>
  </div>
</template>

<script lang="ts" setup>
import { appWindow } from '@tauri-apps/api/window'
import { useRouter } from 'vue-router'
import { onMounted, ref } from 'vue'
import { appGlobal } from '../app'

const router = useRouter()
const isMacOS = ref(false)
const appVersion = ref()

const isHomePath = () => {
  return router.currentRoute.value.name == 'Home'
}

const backToParent = () => {
  router.back()
}

const pushToSettings = async () => {
  await router.push('settings')
}

const minimizeApp = () => appWindow.minimize()

const closeApp = () => appWindow.close()

onMounted(() => {
  isMacOS.value = appGlobal.os.type == 'Darwin'
  appVersion.value = appGlobal.app.appVersion
})
</script>

<style lang="scss" scoped>
.amcl-toolbar {
  color: white;
  height: 50px;
  display: flex;
  align-items: center;
  position: relative;
  background-color: var(--primary-color);
  padding: 0 16px;
  /* 让内容左右分布 */
  justify-content: space-between;
}

.amcl-toolbar-title {
  font-size: 16px;
  margin-right: 24px;
  line-height: 50px;
}

.amcl-toolbar-button {
  display: flex;
  gap: 8px;
  button {
    height: 40px;
    width: 40px;
    color: white !important;
    font-size: 18px;
    border-radius: 4px;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }
}
</style>
