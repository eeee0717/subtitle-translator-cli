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

https://github.com/user-attachments/assets/e5d1152d-d472-4ed1-b697-76a6c5d9955e

> Note: The Claude-sonnet-3.5 model demonstrates superior translation performance compared to GPT-4o-mini.
