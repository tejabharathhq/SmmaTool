<template>
  <div class="w-full">
    <NuxtPage></NuxtPage>
  </div>
</template>

<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import type { LeadItem } from './types/LeadItem';
const recentLeads = useState<LeadItem[]>('recent-leads', () => []);
onMounted(() => {

})


const unlisten = await listen('lead-scraped', (event) => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object

  const res: any = event.payload
  const lead = res
  recentLeads.value.push(JSON.parse(res.message))
  console.log('Recent leads', lead)
})


</script>

<style>
* {
  box-sizing: border-box;
  margin: 0px;
  padding: 0px;
  font-family: sans-serif;
}
</style>