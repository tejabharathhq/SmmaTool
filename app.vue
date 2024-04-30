<template>
  <NuxtLayout>
    <NuxtPage></NuxtPage>
  </NuxtLayout>
</template>

<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import type { LeadItem } from './types/LeadItem';
import socket from './socket';
const router = useRouter();
const recentLeads = useState<LeadItem[]>('recent-leads', () => []);
const store = useUserStore();

/* onMounted(async () => {
  const updateTimeString = '2024-4-30';

  const today = new Date();
  const day = today.getDate();
  const month = today.getMonth() + 1; // Months are zero-based, so January is 0
  const year = today.getFullYear();

  const currentTimeString = `${year}-${month}-${day}`
  if (updateTimeString != currentTimeString) {
    return router.push('/update')
  }

  console.log(`${year}-${month}-${day}`);
  const token = localStorage.getItem('token');
  if (token) {
    try {
      const userResponse = await getUserData(token);
      store.setUser(userResponse!);
      router.push('/app')
      console.log(store.$state.userState)
    } catch (error) {
      router.push('/auth/signin')
    }

  } else {
    router.push('/auth/signup')
  }

}) */


onMounted(() => {
  router.push('/app')
})

const unlisten = await listen('lead-scraped', (event) => {
  const res: any = event.payload
  const lead = JSON.parse(res.message)
  recentLeads.value.push(JSON.parse(res.message))
  if (socket.connected) socket.emit('lead-scraped', lead)
  console.log('Recent leads', lead)
})

onUnmounted(() => {
  unlisten();
})

</script>

<style>
* {
  box-sizing: border-box;
  margin: 0px;
  padding: 0px;
  font-family: sans-serif;
}

html,
body {
  width: 100%;
}
</style>