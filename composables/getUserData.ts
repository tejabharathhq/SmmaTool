import axios, { type AxiosResponse } from "axios";

interface UserResponse {
  id: string;
  username: string;
  email: string;
  firstName: string;
  lastName: string;
}

export const getUserData = async (
  token: string
): Promise<UserResponse | void> => {
  const { apiBase } = useRuntimeConfig().public;

  try {
    const response: AxiosResponse<UserResponse> = await axios.get(
      `${apiBase}/user`,
      {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      }
    );

    if (response.status === 200) {
      const userData: UserResponse = response.data;
      return userData;
      // Handle the user data here
    } else {
      console.error("Failed to fetch user data. Status:", response.status);
      throw new Error(`Failed to fetch user data. Status ${response.status}`);
    }
  } catch (error: any) {
    console.error("Error fetching user data:", error);
    throw new Error(error.message);

    // Handle error
  }
};
