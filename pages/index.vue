<script setup lang="ts">
import { SunIcon, MoonIcon } from '@radix-icons/vue'
import { invoke } from "@tauri-apps/api/core";
import { Button } from '@/components/ui/button'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'

const colorMode = useColorMode()

const submit = async () => {
    const response = await invoke("greet", { name: 'ass' });
}

</script>

<template>
    <nav class="w-full flex items-center justify-between px-2 py-2">
        <div class="w-full flex items-center justify-between">
            <h2 class="font-bold uppercase">DUNCECO</h2>
        </div>
        <div class="flex items-center">

            <div class="gap-x-3 mr-2 flex items-center">

                <Button @click="submit">Greet</Button>
                <NuxtLink to="/leads">
                    <Button>Leads</Button>
                </NuxtLink>

                <NuxtLink to="/scrape">
                    <Button>Scrape</Button>
                </NuxtLink>

                <NuxtLink to="/auth/signin">
                    <Button>Signin</Button>
                </NuxtLink>
            </div>
            <DropdownMenu>
                <DropdownMenuTrigger as-child>
                    <Button variant="ghost" size="icon">
                        <SunIcon v-if="colorMode.preference == 'light'"></SunIcon>
                        <MoonIcon v-else></MoonIcon>
                        <span class="sr-only">Toggle theme</span>
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                    <DropdownMenuItem @click="colorMode.preference = 'light'">
                        Light
                    </DropdownMenuItem>
                    <DropdownMenuItem @click="colorMode.preference = 'dark'">
                        Dark
                    </DropdownMenuItem>
                    <DropdownMenuItem @click="colorMode.preference = 'system'">
                        System
                    </DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenu>
            <!--  <Button variant="ghost" size="icon">
                <GearIcon></GearIcon>
            </Button> -->
        </div>
    </nav>


</template>