export class EnvVarRetriever {
    constructor() {}

    public contains(key: string): boolean {
        return !!this.get(key);
    }

    public get(key: string): string | undefined {
        return process.env[key];
    }

    public getRequired(key: string): string {
        if (this.contains(key)) {
            return this.get(key)!;
        }
        throw new Error(`Environment value ${key} not found`);
    }
}