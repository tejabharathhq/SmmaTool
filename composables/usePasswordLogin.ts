import axios, { type AxiosResponse } from "axios";

interface LoginData {
  username: string;
  password: string;
}

interface LoginResponse {
  message: string;
  token: string;
}

export const usePasswordLogin = async (
  loginData: LoginData
): Promise<LoginResponse | void> => {
  const { apiBase } = useRuntimeConfig().public;

  try {
    const response: AxiosResponse = await axios.post(
      `${apiBase}/auth/password-login`,
      loginData
    );

    if (response.status === 200) {
      const responseData: LoginResponse = response.data;
      return responseData;
    } else {
      console.error("Failed to log in. Status:", response.status);
      throw new Error("Failed to log in.");
    }
  } catch (error: any) {
    console.error("Error logging in:", error);
    throw new Error(error.message);
  }
};
