export abstract class Util {
    /**
     * Remove and replace special characters.
     */
    public static cleanString(s: string): string {
        return s.replace("\r\n", "\n").replace("’", "'").trim();
    }
}
