export abstract class Util {
    /**
     * Remove and replace special characters.
     */
    public static cleanString(s: string): string {
        return s.replace("\r\n", "\n").replace("’", "'").trim();
    }

    public static areEqualShallow(a: any, b: any): boolean {
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
