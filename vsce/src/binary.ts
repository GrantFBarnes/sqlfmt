import * as fs from "fs";
import * as path from "path";
import * as tar from "tar";
import * as vscode from "vscode";
import * as zlib from "zlib";
import Stream from "stream";
import { promisify } from "util";

export async function getBinaryPath(rootPath: string): Promise<string> {
    return await vscode.window.withProgress({
        location: vscode.ProgressLocation.Notification,
        title: "Setting up sqlfmt binary...",
        cancellable: false,
    }, async () => {
        const binaryName: string = getPlatformBinaryName();
        const binaryPath: string = path.join(rootPath, binaryName);

        if (!fs.existsSync(binaryPath)) {
            await getBinary(rootPath);
        }

        return binaryPath;
    });
}

function getBinary(rootPath: string): Promise<void> {
    return new Promise(async (resolve, reject) => {
        const mkdir = promisify(fs.mkdir);
        await mkdir(rootPath, { recursive: true });

        const version: string = await getLatestReleaseTag();
        const assetName: string = getPlatformAssetName();

        const response: Response = await fetch(`https://github.com/GrantFBarnes/sqlfmt/releases/download/${version}/${assetName}`);
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
    });
}

function getLatestReleaseTag(): Promise<string> {
    return new Promise(async (resolve, reject) => {
        try {
            const response: Response = await fetch("https://api.github.com/repos/GrantFBarnes/sqlfmt/releases/latest");
            if (response.status !== 200) {
                return reject();
            }

            const json: any = await response.json();
            return resolve(json.tag_name);
        }
        catch {
            return reject();
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