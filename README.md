# Tauri + T3 Chat

Nothing big, just https://t3.chat wrapped in a tauri shell to use as a desktop app until the official one drops :)

PS: also supports Ctrl + Alt + X to open the chat if its open in the background somewhere

## Running the Application

### Development Mode

To launch the application in development mode, use the following command:

```bash
pnpm tauri dev
```

This command will start the development server and open the application window. Any changes you make to your code will be automatically reflected in the application.

### Building for Production

To build the application for production, use the following command:

```bash
pnpm tauri build
```

This command will compile your application and create distributable packages for your target platforms. You can find the built packages in the `src-tauri/target/release/bundle` directory.

## Modifying Application Shortcuts

Application-specific logic and potentially shortcut definitions are often managed within the Rust backend. To change or define application shortcuts, you will need to modify the `src-tauri/src/lib.rs` file.
