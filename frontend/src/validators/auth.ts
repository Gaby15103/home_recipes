import {z} from "zod"

export const loginSchema = z.object({
    email: z
        .string()
        .email("Invalid email"),
    password: z.string().min(8, "Password must be at least 8 characters"),
})
export const registerSchema = z.object({
    username: z.string().min(3,"Username must be at least 3 characters"),
    email: z.string().email("Invalid email"),
    password: z.string().min(8,"Password must be at least 8 characters"),
    first_name: z.string().min(1,"First name must be at least 1 character"),
    last_name: z.string().min(1,"Last name must be at least 1 character"),
})

