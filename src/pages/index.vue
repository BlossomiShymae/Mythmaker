<template>
  <div class="m-2 w-100">
    <div class="d-flex gap-3 mb-2">
      <img class="img-fluid rounded-circle" style="max-width: 80px" :src="`https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/profile-icons/${summoner.profileIconId}.jpg`"/>
      <div>
        <h3>{{ summoner.gameName }}#{{ summoner.tagLine }}</h3>
        <p>{{ playerSummary.overallChallengeLevel.toLowerCase().replace(/\b(\s\w|^\w)/g, (x) => x.toUpperCase() ) }}</p>
      </div>
    </div>
    <div class="d-flex justify-content-between align-items-center gap-2 mb-2">
      <div>{{ crystal.percentile.toFixed(2) }}%</div>
      <progress style="width: 100%" :value="crystal.currentValue" :max="crystal.nextThreshold"></progress>
      <div>{{ crystal.currentValue }}/{{ crystal.nextThreshold }}</div>
    </div>

    <div class="input-group mb-3">
      <input type="text" class="form-control" placeholder="Search for a challenge..."/>
      <div class="input-group-append">
        <button class="btn btn-outline-light" type="button">Search</button>
      </div>
    </div>
    
    <div class="d-flex flex-wrap gap-3 w-100">
      <div v-for="(value, key, index) in ranked" :key="index"
        class="d-flex gap-3 border border-top-0 border-end-0 border-bottom-0 border-3 border-light ps-2"
        style="width: 400px;"> 
        <div class="d-flex flex-column justify-contenter-center align-items-center gap-1" style="max-width: 80px">
          <img
          v-if="value.currentLevel !== 'NONE'"
          class="img-fluid"
          :src="`https://raw.communitydragon.org/latest/game/assets/challenges/config/${value.id}/tokens/${value.currentLevel}.png`.toLowerCase()"
          loading="lazy"/>
          <img v-else
          class="img-fluid"
          :src="`https://raw.communitydragon.org/latest/game/assets/challenges/config/${value.id}/tokens/IRON.png`.toLowerCase()"
          loading="lazy"/>
          <span class="badge bg-dark">{{ value.id }}</span>
        </div>
        <div class="d-flex flex-column w-100">
          <h5>{{ value.name }}</h5>
          <p class="h-100">{{ value.description }}</p>
          <div class="d-flex justify-content-between align-items-center gap-2">
            <div>{{ value.percentile.toFixed(2) }}%</div>
            <progress style="width: 100%" :value="value.currentValue" :max="value.nextThreshold"></progress>
            <div>{{ value.currentValue }}/{{ value.nextThreshold }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { invoke } from "@tauri-apps/api";

const challenges = await invoke("get_local_player_challenges") as any[];
const ranked = challenges.sort((a, b) => a.percentile - b.percentile);

const categories = await invoke("get_local_player_categories") as any;
const crystal = categories["0"];

const summoner = await invoke("get_current_summoner") as any;
const playerSummary = await invoke("get_local_player_summary") as any;
</script>