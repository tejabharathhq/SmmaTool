<template>
    <div class="w-full relative">
        <div class="grid w-full max-w-sm items-center gap-1.5 mx-auto min-h-screen">
            <div class="flex flex-col gap-y-1.5">
                <Label for="location-input">Location</Label>

                <Input v-model="locationQuery" id="location-input" type="text" placeholder="Enter a location" />
                <Input v-model="maxResults" id="max-results-input" type="number"
                    placeholder="Maximum number of results to scrape" />

                <Button @click="handleScrape">Start Scraping</Button>

                <p>{{ generatedUrl }}</p>
            </div>
        </div>

        <NuxtLink to="/" class="absolute top-5 right-5">
            <Button>
                <ArrowLeftIcon></ArrowLeftIcon>
            </Button>

        </NuxtLink>
    </div>

</template>

<script setup>
import { invoke } from "@tauri-apps/api/core";
import { ArrowLeftIcon } from '@radix-icons/vue'

const GOOGLE_MAPS_URL = 'https://www.google.com/maps/search/';
const locationQuery = ref('');
const maxResults = ref('');
const generatedUrl = ref('');

const handleScrape = async () => {
    const formattedQuery = locationQuery.value.replace(/\s+/g, '+');
    const url = `${GOOGLE_MAPS_URL}${formattedQuery}`;
    console.log("Generated URL:", url);
    generatedUrl.value = url;

    const response = await invoke("scrape", { query: url, max: `${maxResults.value}` })
}
</script>
