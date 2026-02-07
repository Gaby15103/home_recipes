const COOKIE_NAME = "lang";
const ONE_YEAR = 60 * 60 * 24 * 365;

export function getLangCookie(): string | null {
    const match = document.cookie
        .split("; ")
        .find(row => row.startsWith(`${COOKIE_NAME}=`));

    return match ? match.split("=")[1] : null;
}

export function setLangCookie(lang: string) {
    document.cookie = `${COOKIE_NAME}=${lang}; path=/; max-age=${ONE_YEAR}`;
}
