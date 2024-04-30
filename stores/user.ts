  import { defineStore } from "pinia";
  import type { User } from "~/types/User";
  export const useUserStore = defineStore("user", () => {
    const userState = ref<User | null>(null);
    const tokenState = ref<string | null>(null);
    function setUser(user: User | null) {
      userState.value = user;
    }
    function setToken(token: string | null) {
      tokenState.value = token;
    }

    return { userState, tokenState, setUser, setToken };
  });
