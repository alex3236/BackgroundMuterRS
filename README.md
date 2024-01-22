# BackgroundMuterRS

Mute background games and applications! (Windows only)

This my first try in Rust, so there's a high probability that there will be quite a few mistakes.

## Usage

A tray icon will be shown in the system tray when you execute. Click on it to open the main window.

> [!NOTE]
> The memory usage will be less until you open the UI for the first time. I tried freeing those memory after closing the window, but it seemed to cause a memory leak, so I gave up for now.

Exclude explorer: When checked, File Explorer will not be recognized as a foreground app. (Because when using alt-tab or taskbar, explorer will be recognized as foreground.)

## TODO

- [ ] Whitelist mode
