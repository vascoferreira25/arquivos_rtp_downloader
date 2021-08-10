# Arquivos RTP downloader

A program to download videos from (RTP Arquivos)[https://arquivos.rtp.pt/].

## How to build

To build this project, you need Rust installed and run:

```powershell
cargo build --release
```

## How to run

The executable is called `artp-dl`. And to download videos you just have to run:

```powershell
artp-dl.exe link1 link2 link3 ...
```

The program downloads 3 videos in parallel and you can specify any amount of
links to download.

The videos downloaded will go to the current working directory on the `.ts`
format.
