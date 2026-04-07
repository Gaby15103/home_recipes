import {z} from "zod"

export const loginSchema = z.object({
    email: z
        .string()
        .email("Invalid email"),
    password: z.string().min(8, "Password must be at least 8 characters"),
})
export const registerSchema = z.object({
    username: z.string().min(3),
    first_name: z.string().min(1),
    last_name: z.string().min(1),
    email: z.string().email(),
    email_confirmation: z.string().email(),
    password: z.string().min(8),
}).refine((data) => data.email === data.email_confirmation, {
    message: "Emails do not match",
    path: ["email_confirmation"], // This attaches the error to the confirmation field
});

