<template>
  <div>
    <header
      class="container-fluid sticky-top p-2 bg-transparent"
      style="z-index: 2000; backdrop-filter: blur(4px);">
    <div class="grid-aside w-100">
      <div class="d-flex align-content-center align-items-center w-100 h-100">
        <NuxtLink
          class="btn rounded text-decoration-none hover-dim me-2"
          to="/"
          title="Home">
          <PhHouse weight="fill" size="24" />
        </NuxtLink>
        <div class="d-flex justify-content-end align-items-center w-100 h-100 gap-2">
          <span class="badge bg-dark me-2">{{ appVersion }}</span>
          <PhGithubLogo weight="fill" size="24" />
          <div class="vr me-2"></div>
          <PhArrowsClockwise weight="fill" size="24" />
        </div>
      </div>
    </div>
    </header>
    <main class="container-fluid">
      <slot></slot>
    </main>
    <div class="container-fluid">
      <footer class="py-3 my-4 d-flex flex-wrap justify-content-between align-items-start border-top">
        <div class="col-md-4 d-flex align-items-center justify-content-center mb-2 mb-md-0 me-md-auto">
          <div class="text-secondary">Blossomi Shymae</div>
        </div>
      </footer>
    </div>
  </div>
</template>

<script setup>
import { PhGithubLogo, PhHouse, PhArrowsClockwise } from '@phosphor-icons/vue';
import { getVersion } from "@tauri-apps/api/app";

const appVersion = await getVersion();
</script>