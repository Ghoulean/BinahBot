const FIND_REPLACE_CHARS: { [key: string] : string } = {
    "\r\n": "\n",
    "’": "'",
    "Ⅰ": "I",
    "Ⅱ": "II",
    "Ⅲ": "III",
}

export abstract class Util {
    /**
     * Find and replace special characters.
     */
    public static cleanString(s: string): string {
        for (const k in FIND_REPLACE_CHARS) {
            const v = FIND_REPLACE_CHARS[k]
            s = s.replace(k, v);
        }
        return s.trim();
    }

    public static areEqualShallow(a: any, b: any): boolean {
        if (a == b) {
            return true;
        }
        if (a == undefined || b == undefined) {
            return false;
        }
        for (var key in a) {
            if (!(key in b) || a[key] !== b[key]) {
                return false;
            }
        }
        for (var key in b) {
            if (!(key in a) || a[key] !== b[key]) {
                return false;
            }
        }
        return true;
    }
}
