import * as fs from "fs";
import * as path from "path";
import * as tar from "tar";
import * as zlib from "zlib";
import Stream from "stream";
import cp from "child_process";
import { promisify } from "util";

export async function getBinaryPath(rootPath: string): Promise<string> {
    const binaryName: string = getPlatformBinaryName();
    const binaryPath: string = path.join(rootPath, binaryName);

    const latestVersion: string = await getLatestVersion();

    if (!fs.existsSync(binaryPath)) {
        await getBinary(rootPath, latestVersion);
    }

    const installedVersion: string = await getInstalledVersion(binaryPath);
    if (latestVersion != "" && installedVersion != latestVersion) {
        await getBinary(rootPath, latestVersion);
    }

    return binaryPath;
}

function getLatestVersion(): Promise<string> {
    return new Promise(async (resolve) => {
        try {
            const response: Response = await fetch("https://api.github.com/repos/GrantFBarnes/sqlfmt/releases/latest");
            if (response.status !== 200) {
                return resolve("");
            }

            const json: any = await response.json();
            return resolve(json.tag_name);
        }
        catch {
            return resolve("");
        }
    });
}

function getBinary(rootPath: string, latestVersion: string): Promise<void> {
    return new Promise(async (resolve, reject) => {
        try {
            const mkdir = promisify(fs.mkdir);
            await mkdir(rootPath, { recursive: true });

            const assetName: string = getPlatformAssetName();

            const response: Response = await fetch(`https://github.com/GrantFBarnes/sqlfmt/releases/download/${latestVersion}/${assetName}`);
            if (response.status !== 200) {
                return reject();
            }

            const assetPath: string = path.join(rootPath, assetName);
            const assetWriteStream: fs.WriteStream = fs.createWriteStream(assetPath);
            assetWriteStream.write(Buffer.from(await response.arrayBuffer()));
            assetWriteStream.end();
            assetWriteStream.on("error", reject);
            assetWriteStream.on("finish", () => {
                Stream.pipeline(
                    fs.createReadStream(assetPath),
                    zlib.createGunzip(),
                    tar.extract({ cwd: rootPath }),
                    (err) => {
                        if (err) {
                            reject();
                        } else {
                            resolve();
                        }
                    },
                );
            });
        }
        catch {
            return reject();
        }
    });
}

function getInstalledVersion(binaryPath: string): Promise<string> {
    return new Promise((resolve, reject) => {
        try {
            const process = cp.spawn(binaryPath, ["--version"]);

            let output: string = "";

            process.stdout.on("data", (data: any) => {
                output += data.toString();
            });

            process.on("close", (code: any) => {
                if (code === 0) {
                    resolve(`v${output.trim()}`);
                } else {
                    reject();
                }
            });

            process.on("error", () => reject());
        }
        catch {
            reject();
        }
    });
}

function getPlatformBinaryName(): string {
    switch (process.platform) {
        case "win32":
            return "sqlfmt.exe";
        case "darwin":
        case "linux":
            return "sqlfmt";
        default:
            throw new Error("Unsupported platform");
    }
}

function getPlatformAssetName(): string {
    switch (process.platform) {
        case "win32":
            return "sqlfmt-x86_64-pc-windows-msvc.tar.gz";
        case "darwin":
            return "sqlfmt-aarch64-apple-darwin.tar.gz";
        case "linux":
            return "sqlfmt-x86_64-unknown-linux-gnu.tar.gz";
        default:
            throw new Error("Unsupported platform");
    }
}