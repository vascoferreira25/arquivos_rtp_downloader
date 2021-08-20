# Arquivos RTP downloader

[![Rust](https://github.com/vascoferreira25/arquivos_rtp_downloader/actions/workflows/release.yml/badge.svg)](https://github.com/vascoferreira25/arquivos_rtp_downloader/actions/workflows/release.yml)

Um programa para fazer download de videos do [RTP Arquivos](https://arquivos.rtp.pt/).

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

Este programa tem 2 versões, uma com uma interface gráfica e outra para ser
executada na consola.

A interface gráfica é a seguinte:

![Dark Theme](./examples/dark_theme.png)

![Light Theme](./examples/light_theme.png)

Basta copiar os links do site dos Arquivos RTP, inserir na caixa de texto e
adicionar.  Quando estiveram todos os links pretendidos, clicar em
`Download`. O programa irá fazer o download dos videos para a mesma pasta do
executável. Durante o download o programa fica irresponsivo até terminar. Se
algum dos links for inválido e programa irá fechar.

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
qualquer quantidade de links para se fazer download. O programa trata de fazer
o download de todos os videos automaticamente.

Os videos vão para a pasta de onde o código for executado e terão o formato
`.ts`.

## EN

Instructions in English.

This program has 2 versions, one with a graphical user interface and another to
be executed through the command line.

The interface looks like this:

![Dark Theme](./examples/dark_theme.png)

![Light Theme](./examples/light_theme.png)

You just need to copy the links from Arquivos RTP, paste it into the text box
and add the link. After all the links are inserted, click on `Download`. The
program will download all the videos into the folder of the executable. During
the download the program stays irresponsive. If one of the links is invalid,
the program will crash.

### How to build

To build this project, you need Rust installed and run:

```powershell
cargo build --release
```

### How to run

The executable is called `artp-dl`. And to download videos you just have to
run:

```powershell
artp-dl.exe link1 link2 link3 ...
```

The program downloads 3 videos in parallel and you can specify any amount of
links to download. The program will handle all the links automatically.

The videos downloaded will go to the current working directory on the `.ts`
format.
