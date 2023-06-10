# rstrt

## Installation
### Build from source
Requires [Rust](https://www.rust-lang.org/tools/install).

```bash
git clone git@github.com:FilipHarald/rstrt.git
cd rstrt
cargo build --release
cp ./target/release/rstrt $HOME/bin/rstrt
```

## Usage
### When listing files
```bash
find ~/* | rstrt
```

### Use in prompt
Use this in your `.bashrc`.
```bash
promptFunc() {
  if [[ -x $(realpath "$HOME/bin/rstrt") ]]
  then
    WD=`pwd | $HOME/bin/rstrt --ps1-escape`
    COLORIZED_DIR=`echo "${WD}" | awk -F/ '{print $NF}'`
    PS1="\`echo -e \"\[\a\]\[\033[01;32m\]\h\[\033[01;34m\] ${COLORIZED_DIR} \$ \"\`"
  else
    PS1='\[\a\]\[\033[01;32m\]\h\[\033[01;34m\] \W \$ \[\033[00m\]'
  fi
}
PROMPT_COMMAND="promptFunc"
```

