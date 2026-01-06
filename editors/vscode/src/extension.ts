import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

export function activate(context: vscode.ExtensionContext) {
    const config = vscode.workspace.getConfiguration('duck');
    const lspEnabled = config.get<boolean>('lsp.enabled', true);

    if (!lspEnabled) {
        console.log('Duck LSP is disabled');
        return;
    }

    const lspPath = config.get<string>('lsp.path', 'duck-lsp');

    const serverOptions: ServerOptions = {
        command: lspPath,
        args: [],
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'duck' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.duck'),
        },
    };

    client = new LanguageClient(
        'duck-lsp',
        'Duck Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
    console.log('Duck Language Server started');
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
