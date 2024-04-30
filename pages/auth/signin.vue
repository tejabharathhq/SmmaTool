<template>
    <div class="h-screen w-full grid lg:grid-cols-2 relative">
        <div class="h-full w-full hidden lg:block">
            <img src="/placeholder.svg" class="h-full w-full object-cover" alt="" />
        </div>
        <NuxtLink class="absolute right-4 top-4 md:right-8 md:top-8" href="/auth/signup">
            <Button>
                Register
            </Button>
        </NuxtLink>

        <div class="h-full w-full flex flex-col justify-center">
            <div class="lg:p-8">
                <div class="mx-auto flex w-full flex-col justify-center space-y-6 sm:w-[350px]">
                    <div class="flex flex-col space-y-2 text-center">
                        <h1 class="text-2xl font-semibold tracking-tight">
                            Welcome Back!
                        </h1>
                        <p class="text-sm text-muted-foreground">
                            Please enter your credentials to access your account.
                        </p>
                    </div>
                    <div class="grid gap-6">
                        <form @submit.prevent="onSubmit">
                            <div class="grid gap-2">
                                <div class="grid gap-2">
                                    <Label for="username">Username</Label>
                                    <Input v-model="username" id="email" type="username" placeholder="username"
                                        required />
                                </div>
                                <div class="grid gap-2">
                                    <Label for="password">Password</Label>
                                    <Input v-model="password" id="password" type="password" />
                                </div>
                                <Button :disabled="isLoading">
                                    <Loader2 v-if="isLoading" class="mr-2 h-4 w-4 animate-spin" />
                                    Sign In with Username
                                </Button>
                            </div>
                        </form>
                        <div class="relative">
                            <div class="absolute inset-0 flex items-center">
                                <span class="w-full border-t" />
                            </div>
                            <div class="relative flex justify-center text-xs uppercase">
                                <span class="bg-background px-2 text-muted-foreground">
                                    Or continue with
                                </span>
                            </div>
                        </div>
                        <Button variant="outline" type="button" :disabled="isLoading">
                            <Loader2 v-if="isLoading" class="mr-2 h-4 w-4 animate-spin" />
                            <DiscordLogoIcon v-else class="mr-2 h-4 w-4" />
                            Discord
                        </Button>
                    </div>
                    <div class="px-8 text-center text-sm text-muted-foreground">
                        Already have an account?
                        <NuxtLink to="/auth/signin" class="underline">
                            Sign in
                        </NuxtLink>
                    </div>
                </div>

            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { DiscordLogoIcon, } from '@radix-icons/vue';
import { Loader2 } from 'lucide-vue-next'
const username = ref('');
const password = ref('');
const store = useUserStore();
const router = useRouter();
const onSubmit = async () => {
    console.log('hooked')
    const obj = {
        username: username.value.toLowerCase(),
        password: password.value
    }
    try {
        const authResponse = await usePasswordLogin(obj);
        store.setToken(authResponse!.token)
        const userResponse = await getUserData(authResponse!.token);
        store.setUser(userResponse!);
        localStorage.setItem('token', authResponse!.token);
        router.push('/app');
    } catch (error: any) {
        console.error(error.message)
    }
    console.log('finished the request')
    console.log(obj);
}

const isLoading = ref(false)
</script>
