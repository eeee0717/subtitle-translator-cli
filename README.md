# Subtitle-Translator-Cli
## Description
This is a simple CLI tools to translate subtitle files.

## Usage
### Setup
copy `.env.cp` to `.env` and fill in the required information.
```bash
cp .env.cp .env
```
### Translate
```bash
stc open-ai -s <SOURCE_LANGUAGE> -t <TARGET_LANGUAGE> -p <PATH>
```

![Demo](./img/demo.mp4)
> Note: The Claude-sonnet-3.5 model demonstrates superior translation performance compared to GPT-4o-mini.