<template>
    <header class="sticky top-0 flex h-14 min-h-14 items-center gap-4 border-b bg-background px-4 md:px-6">
        <nav
            class="hidden flex-col gap-6 text-lg font-medium md:flex md:flex-row md:items-center md:gap-5 md:text-sm lg:gap-6">
            <NuxtLink href="#" class="flex items-center gap-2 text-lg font-semibold md:text-base">
                <Package2 class="h-6 w-6" />
                <span class="sr-only">Acme Inc</span>
            </NuxtLink>
            <NuxtLink to="/app/" class="text-muted-foreground transition-colors hover:text-foreground">
                Dashboard
            </NuxtLink>
            <NuxtLink to="/app/scrape" class="text-muted-foreground transition-colors hover:text-foreground">
                Scrape
            </NuxtLink>
            <NuxtLink to="/app/leads" class="text-muted-foreground transition-colors hover:text-foreground">
                Leads
            </NuxtLink>
            <NuxtLink to="/app/routines" class="text-muted-foreground transition-colors hover:text-foreground">
                Events
            </NuxtLink>
            <NuxtLink to="/app/settings" class="text-muted-foreground transition-colors hover:text-foreground">
                Settings
            </NuxtLink>
        </nav>
        <Sheet>
            <SheetTrigger as-child>
                <Button variant="outline" size="icon" class="shrink-0 md:hidden">
                    <Menu class="h-5 w-5" />
                    <span class="sr-only">Toggle navigation menu</span>
                </Button>
            </SheetTrigger>
            <SheetContent side="left">
                <nav class="grid gap-6 text-lg font-medium">
                    <NuxtLink href="#" class="flex items-center gap-2 text-lg font-semibold">
                        <Package2 class="h-6 w-6" />
                        <span class="sr-only">Acme Inc</span>
                    </NuxtLink>
                    <NuxtLink to="/app/" class="text-muted-foreground hover:text-foreground">
                        Dashboard
                    </NuxtLink>
                    <NuxtLink to="/app/scrape" class="text-muted-foreground hover:text-foreground">
                        Scrape
                    </NuxtLink>
                    <NuxtLink to="/app/appointments" class="text-muted-foreground hover:text-foreground">
                        Appointments
                    </NuxtLink>
                    <NuxtLink to="/app/routines" class="text-muted-foreground hover:text-foreground">
                        Routines
                    </NuxtLink>
                    <NuxtLink to="/app/settings" class="text-muted-foreground hover:text-foreground">
                        Settings
                    </NuxtLink>
                </nav>
            </SheetContent>
        </Sheet>
        <div class="flex w-full items-center gap-4 md:ml-auto md:gap-2 lg:gap-4">
            <form class="ml-auto flex-1 sm:flex-initial">
                <div class="relative">
                    <Search class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
                    <Input type="search" placeholder="Search Anything..."
                        class="pl-8 sm:w-[300px] md:w-[200px] lg:w-[300px]" />
                </div>
            </form>
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
            <DropdownMenu>
                <DropdownMenuTrigger as-child>
                    <Button variant="secondary" size="icon" class="rounded-full">
                        <CircleUser class="h-5 w-5" />
                        <span class="sr-only">Toggle user menu</span>
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                    <DropdownMenuLabel>My Account</DropdownMenuLabel>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem>Settings</DropdownMenuItem>
                    <DropdownMenuItem>Support</DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem @click="logout">Logout</DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenu>
        </div>
    </header>
</template>

<script setup lang="ts">
import { CircleUser, Menu, Package2, Search } from 'lucide-vue-next'
import { SunIcon, MoonIcon } from '@radix-icons/vue'
const colorMode = useColorMode()
const store = useUserStore();
const router = useRouter();
const logout = () => {
    localStorage.clear();
    store.setToken(null);
    store.setUser(null);

    router.push('/auth/signin')

}
</script>


<style scoped>
.router-link-exact-active {
    color: hsl(var(--foreground))
}
</style>