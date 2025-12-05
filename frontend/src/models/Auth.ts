export interface LoginRequest {
    user: {
        email: string;
        password: string;
    };
}

export interface LoginResponse {
    user: {
        email: string;
        token: string;
        username: string;
        first_name: string;
        last_name: string;
        avatar_url: string | null;
        preferences: Record<string, any>;
    };
}

export interface RegisterRequest {
    user: {
        email: string;
        password: string;
        username: string;
        first_name: string;
        last_name: string;
    };
}

export type RegisterResponse = LoginResponse;
