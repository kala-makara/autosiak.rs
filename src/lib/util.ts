export function asStr(obj: unknown): string {
    return JSON.stringify(obj, null, 4);
}