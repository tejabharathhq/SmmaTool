<template>
    <div class="h-screen w-full grid lg:grid-cols-2 relative">
        <div class="h-full w-full hidden lg:block">
            <img src="/placeholder.svg" class="h-full w-full object-cover" alt="" />
        </div>
        <NuxtLink class="absolute right-4 top-4 md:right-8 md:top-8" href="/auth/signin">
            <Button>
                Login
            </Button>
        </NuxtLink>

        <div class="h-full w-full flex flex-col justify-center">
            <div class="lg:p-8">
                <div class="mx-auto flex w-full flex-col justify-center space-y-6 sm:w-[350px]">
                    <div class="flex flex-col space-y-2 text-center">
                        <h1 class="text-2xl font-semibold tracking-tight">
                            Create an account
                        </h1>
                        <p class="text-sm text-muted-foreground">
                            Enter your details below to create your account.
                        </p>
                    </div>
                    <div class="grid gap-6">
                        <form @submit.prevent="onSubmit">
                            <div class="grid gap-2">
                                <div class="grid grid-cols-2 gap-4">
                                    <div class="grid gap-2">
                                        <Label for="first-name">First name</Label>
                                        <Input v-model="firstName" id="first-name" placeholder="Max" required />
                                    </div>
                                    <div class="grid gap-2">
                                        <Label for="last-name">Last name</Label>
                                        <Input v-model="lastName" id="last-name" placeholder="Robinson" required />
                                    </div>
                                </div>

                                <div class="grid gap-2">
                                    <Label for="username">Username</Label>
                                    <Input v-model="username" id="username" type="text" placeholder="robinson"
                                        required />
                                </div>

                                <div class="grid gap-2">
                                    <Label for="email">Email</Label>
                                    <Input v-model="email" id="email" type="email" placeholder="m@example.com"
                                        required />
                                </div>
                                <div class="grid gap-2">
                                    <Label for="password">Password</Label>
                                    <Input v-model="password" id="password" type="password" />
                                </div>
                                <Button :disabled="isLoading">
                                    <Loader2 v-if="isLoading" class="mr-2 h-4 w-4 animate-spin" />
                                    Sign Up with Email
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
                    <p class="px-8 text-center text-sm text-muted-foreground">
                        By clicking continue, you agree to our
                        <a href="/terms" class="underline underline-offset-4 hover:text-primary">
                            Terms of Service
                        </a>
                        and
                        <a href="/privacy" class="underline underline-offset-4 hover:text-primary">
                            Privacy Policy
                        </a>
                        .
                    </p>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { DiscordLogoIcon, } from '@radix-icons/vue';
import { Loader2 } from 'lucide-vue-next'
import { useCreateUser } from '~/composables/useCreateUser';
const store = useUserStore();
const firstName = ref('');
const lastName = ref('');
const email = ref('');
const username = ref('');
const password = ref('');
const isLoading = ref(false)


const onSubmit = async () => {
    isLoading.value = true;
    const obj = {
        firstName: firstName.value,
        lastName: lastName.value,
        username: username.value.toLowerCase(),
        email: email.value.toLowerCase(),
        password: password.value
    }
    try {
        const response = await useCreateUser(obj);
        if (typeof (response?.token) == 'string') store.setToken(response?.token!)
        console.log(response)
    }
    catch (error: any) { console.log(error) }

    isLoading.value = false
}

</script>
