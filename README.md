# Arquivos RTP downloader

[![Rust](https://github.com/vascoferreira25/arquivos_rtp_downloader/actions/workflows/rust.yml/badge.svg)](https://github.com/vascoferreira25/arquivos_rtp_downloader/actions/workflows/rust.yml)

Um programa para fazer download de videos do [RTP
Arquivos](https://arquivos.rtp.pt/).

A program to download videos from [RTP Arquivos](https://arquivos.rtp.pt/).

<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-refresh-toc -->
## Table of Contents

- [Arquivos RTP downloader](#arquivos-rtp-downloader)
    - [PT](#pt)
        - [Como dar build](#como-dar-build)
        - [Como executar](#como-executar)
    - [EN](#en)
        - [How to build](#how-to-build)
        - [How to run](#how-to-run)

<!-- markdown-toc end -->

## PT

Instruções em Português.

### Como dar build

Para criar o executavel é necessário ter a linguagem Rust instalada e executar:

```powershell
cargo build --release
```

### Como executar

O nome do executável é `artp-dl`. E para se fazer download dos videos é
necessário executar o seguinte comando:

```powershell
artp-dl.exe link1 link2 link3 ...
```

O programa faz download de 3 videos em simultâneo e pode-se especificar
qualquer quantidade de links para se fazer download.

Os videos vão para a pasta de onde o código for executado e terão o formato
`.ts`.

## EN

Instructions in English.

### How to build

To build this project, you need Rust installed and run:

```powershell
cargo build --release
```

### How to run

The executable is called `artp-dl`. And to download videos you just have to run:

```powershell
artp-dl.exe link1 link2 link3 ...
```

The program downloads 3 videos in parallel and you can specify any amount of
links to download.

The videos downloaded will go to the current working directory on the `.ts`
format.
