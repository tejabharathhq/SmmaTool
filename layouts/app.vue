<template>
    <div class="flex min-h-screen h-screen w-full flex-col">
        <AppHeader></AppHeader>
        <main class="flex flex-1 flex-col gap-4 md:gap-8">
            <slot></slot>
        </main>
    </div>
</template>
<script setup lang="ts">
</script>
