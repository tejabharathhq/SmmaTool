<template>
    <div class="w-full relative min-h-screen max-h-screen h-screen">
        <div class="flex h-full max-h-full">
            <div class="w-72 max-w-72 min-w-64 py-10 px-2.5 border-r-muted border-r-2 ">
                <h2 class="font-semibold"> SCRAPER</h2>

                <form @submit.prevent="handleScrape" class="w-full space-y-1.5 py-6">
                    <Label for="business">Business Category/Type</Label>
                    <Input id="business" type="text" placeholder="Name" v-model="businessCategory" />
                    <p class="text-muted-foreground text-sm py-2">Enter the type of business here, for example, salons
                        or roofing companies.</p>
                    <div class="py-1"></div>
                    <Label for="location">Location of the business</Label>
                    <Input id="location" type="text" placeholder="Location" v-model="location" />
                    <p class="text-muted-foreground text-sm py-2">Enter the location of the businesses/companies, for
                        example, New York.</p>
                    <div class="py-1"></div>
                    <Label for="cycles">Cycles</Label>
                    <Slider :default-value="[33]" :max="100" :step="1" v-model="cycles" />
                    <p class="text-muted-foreground text-sm py-2 whitespace-nowrap"> <span>How many Scrape Cycles?
                        </span>
                        <span>{{ cycles[0] }} Cycles</span>
                    </p>
                    <div class="py-4"></div>
                    <Button class="w-full my-4" type="submit">
                        Submit
                    </Button>
                </form>

                <Button @click="handleCopy" class="w-full" variant="secondary">Copy</Button>
            </div>
            <div class="w-full h-full p-3">
                <Table>
                    <TableCaption>A list of the most recent scrapes</TableCaption>
                    <TableHeader>
                        <TableRow>
                            <TableHead>
                                Name
                            </TableHead>
                            <TableHead>
                                Address
                            </TableHead>
                            <TableHead>Phone</TableHead>
                            <TableHead>
                                Website
                            </TableHead>

                            <TableHead>
                                Has Website?
                            </TableHead>

                            <TableHead>
                                Has Phone?
                            </TableHead>

                            <TableHead>
                                Ratings
                            </TableHead>

                            <TableHead>
                                Reviews
                            </TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow v-for="lead in leads" :key="lead.name">
                            <TableCell class="font-medium">
                                {{ lead.name }}
                            </TableCell>
                            <TableCell>{{ lead.address }}</TableCell>
                            <TableCell>{{ lead.phone }}</TableCell>
                            <TableCell>{{ lead.website }}</TableCell>
                            <TableCell>{{ lead.has_website }}</TableCell>
                            <TableCell>{{ lead.has_phone }}</TableCell>
                            <TableCell class="text-right">
                                {{ lead.total_reviews }}
                            </TableCell>
                            <TableCell class="text-right">
                                {{ lead.ratings }}
                            </TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </div>
        </div>



    </div>

</template>

<script setup>
import { invoke } from "@tauri-apps/api/core";

const GOOGLE_MAPS_URL = 'https://www.google.com/maps/search/';
const generatedUrl = ref('');
const businessCategory = ref('');
const location = ref('');
const cycles = ref([33]); // Default value for cycles
const leads = ref([]); // Placeholder for leads data
let responseJSON = '[]'

const handleScrape = async () => {
    const formattedQuery = `${businessCategory.value}+in+${location.value}`.replace(/\s+/g, '+');
    const url = `${GOOGLE_MAPS_URL}${formattedQuery}`;
    console.log("Generated URL:", url);
    generatedUrl.value = url;
    console.log(url, cycles.value[0])
    const response = await invoke("scrape", { query: url, max: `${cycles.value[0]}` });
    responseJSON = response
    leads.value = JSON.parse(response);
}


const handleCopy = () => {
    navigator.clipboard.writeText(responseJSON);
}
</script>
