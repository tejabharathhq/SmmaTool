<script setup lang="ts">
import { columns } from "./columns"
import DataTable from "./DataTable.vue"
import type { LeadItem } from '@/types/LeadItem';
const data = useState<LeadItem[]>('recent-leads')


</script>

<template>
    <div class="w-full py-10 px-2">
        <DataTable :columns="columns" :data="data" />
    </div>
</template>