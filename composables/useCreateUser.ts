import axios, { type AxiosResponse } from "axios";

interface CreateUserData {
  username: string;
  password: string;
  email: string;
  firstName: string;
  lastName: string;
}

interface CreateUserResponse {
  message: string;
  token: string;
}

export const useCreateUser = async (
  userData: CreateUserData
): Promise<CreateUserResponse | undefined> => {
  const { apiBase } = useRuntimeConfig().public;

  try {
    const response: AxiosResponse = await axios.post(
      `${apiBase}/auth/create-user`,
      userData
    );

    if (response.status === 200) {
      const responseData: CreateUserResponse = response.data;
      console.log(responseData.message); // Log success message
      console.log(responseData.token); // Log token
      return responseData;
    } else {
      console.error("Failed to create user. Status:", response.status);
      throw new Error("Failed to create user.");

      // Handle other status codes if needed
    }
  } catch (error) {
    console.error("Error creating user:", error);
    // Handle error
    throw new Error("Failed to create user.");
  }
};
