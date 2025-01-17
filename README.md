# Tauri + T3 Chat

Nothing big, just https://t3.chat wrapped in a tauri shell to use as a desktop app until the official one drops :)

PS: also supports Ctrl + Alt + X to open the chat if its open in the background somewhere

## Running the Application

### Development Mode

To launch the application in development mode, use the following command:

```bash
pnpm tauri dev
```

### Building for Production

To build the application for production, use the following command:

```bash
pnpm tauri build
```

## Modifying Application Shortcuts

To change opening shortcut, you can modify the `src-tauri/src/lib.rs` file.
