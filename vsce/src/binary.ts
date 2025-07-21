export async function getBinaryPath(): Promise<string> {
    return new Promise((resolve, reject) => {
        resolve("sqlfmt");
    });
}