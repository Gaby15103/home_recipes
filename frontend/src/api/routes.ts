export const API_ROOT = "/api";

export const AuthRoutes = {
    login: () => `${API_ROOT}/auth/login`,
    register: () => `${API_ROOT}/auth/register`,
    confirmEmail: () => `${API_ROOT}/auth/confirm_email`,
    logout: () => `${API_ROOT}/auth/logout`,
    forgotPassword: () => `${API_ROOT}/auth/forgot_password`,
    resetPassword: () => `${API_ROOT}/auth/reset_password`,
};

export const UserRoutes = {
    me: () => `${API_ROOT}/user/me`,
    updateMe: () => `${API_ROOT}/user/me`,
    getById: (id: string) => `${API_ROOT}/user/${id}`,
    update: (id: string) => `${API_ROOT}/user/profile/${id}`,
};

export const TagRoutes = {
    list: () => `${API_ROOT}/tags`,
    create: () => `${API_ROOT}/tags`,
    update: (id: string) => `${API_ROOT}/tags/${id}`,
    // delete: (id: string) => `${API_ROOT}/tags/${id}`, // add if implemented
};

export const RecipeRoutes = {
    all: () => `${API_ROOT}/recipes`,
    byPage: () => `${API_ROOT}/recipes/by_page`,
    get: (id: string) => `${API_ROOT}/recipes/${id}`,
    get_last: () => `${API_ROOT}/recipes/last`,
    byAuthor: (id: string) => `${API_ROOT}/recipes/author/${id}`,

    create: () => `${API_ROOT}/recipes`,
    update: (id: string) => `${API_ROOT}/recipes/${id}`,
    delete: (id: string) => `${API_ROOT}/recipes/${id}`,

    analytics: (id: string) => `${API_ROOT}/recipes/${id}/analytics`,
    trackView: (id: string) => `${API_ROOT}/recipes/${id}/views`,

    favorite: (id: string) => `${API_ROOT}/recipes/${id}/favorite`,
    favorites: (id: string) => `${API_ROOT}/recipes/favorites/${id}`,

    rate: (id: string) => `${API_ROOT}/recipes/${id}/rating`,
    unrate: (id: string) => `${API_ROOT}/recipes/${id}/rating`,
    getRating: (id: string) => `${API_ROOT}/recipes/${id}/rating`,

    getComments: (id: string) => `${API_ROOT}/recipes/${id}/comments`,
    addComment: (id: string) => `${API_ROOT}/recipes/${id}/comments`,

    restoreVersion: (recipeId: string, versionId: string) =>
        `${API_ROOT}/recipes/${recipeId}/versions/${versionId}/restore`,
};

export const DebugRoutes = {
    multipart: () => `${API_ROOT}/debug`,
};

export const TwoFactorRoutes = {
    qrCode: () => `${API_ROOT}/auth/two-factor/qr-code`,          // GET
    secretKey: () => `${API_ROOT}/auth/two-factor/secret-key`,   // GET
    recoveryCodes: () => `${API_ROOT}/auth/two-factor/recovery-codes`, // GET
    enable: () => `${API_ROOT}/auth/two-factor/enable`,          // POST
    disable: () => `${API_ROOT}/auth/two-factor/disable`,        // POST
    status: () => `${API_ROOT}/auth/two-factor/status`,          // GET
    verify: () => `${API_ROOT}/auth/two-factor/verify`,          // GET
};
export const IngredientRoutes = {
    list: () => `${API_ROOT}/ingredient/list`,          // GET
}
export const LanguageRoutes = {
    list: () => `${API_ROOT}/language/list`,          // GET
}
export const UploadRoutes = {
    single: () => `${API_ROOT}/upload`
}
export const UnitRoutes = {
    get: (id: string) => `${API_ROOT}/units/${id}`,
    list: () => `${API_ROOT}/units`,
    create: () => `${API_ROOT}/units`,
    update: (id: string) => `${API_ROOT}/units/${id}`
}
export const OcrRoutes = {
    process: () => `${API_ROOT}/ocr/process`,
    process_regions: () => `${API_ROOT}/ocr/process_regions`,
    create: () => `${API_ROOT}/ocr/confirm`,
}
export const SystemRoutes = {
    health: () => `${API_ROOT}/health`,
}