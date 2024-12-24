# cli-wrapped 🌯
cli-wrapped is a fun CLI tool inspired by Spotify Wrapped. It keeps track of every command you run over the year, tallies their frequency, and generates a year end report highlighting your top commands.

## Features
- **Track your CLI activity:** Automatically log every command your execute in your terminal throughout the year.
- **Top Commands Report:** Get to know your Top 10 most used commands and invocations.

## Example
```bash
$ cli-wrapped

           ██████╗██╗     ██╗    ██╗    ██╗██████╗  █████╗ ██████╗ ██████╗ ███████╗██████╗ 
          ██╔════╝██║     ██║    ██║    ██║██╔══██╗██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔══██╗
          ██║     ██║     ██║    ██║ █╗ ██║██████╔╝███████║██████╔╝██████╔╝█████╗  ██║  ██║
          ██║     ██║     ██║    ██║███╗██║██╔══██╗██╔══██║██╔═══╝ ██╔═══╝ ██╔══╝  ██║  ██║
          ╚██████╗███████╗██║    ╚███╔███╔╝██║  ██║██║  ██║██║     ██║     ███████╗██████╔╝
           ╚═════╝╚══════╝╚═╝     ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝     ╚══════╝╚═════╝ 
                               ██████╗    ██████╗   ██████╗   ██╗  ██╗
                               ╚════██╗  ██╔═████╗  ╚════██╗  ██║  ██║
                                █████╔╝  ██║██╔██║   █████╔╝  ███████║
                               ██╔═══╝   ████╔╝██║  ██╔═══╝   ╚════██║
                               ███████╗  ╚██████╔╝  ███████╗       ██║
                               ╚══════╝   ╚═════╝   ╚══════╝       ╚═╝


     Count | Top Commands                  Count | Top Invocations
   --------+------------------------     --------+--------------------------------
     000000|                               000000|
     000000|                               000000|
     000000|                               000000|
     000000|                               000000|
     000000|                               000000|

   Total Commands > 0

```

## Build
1. Clone this repository and navigate into it:
    ```bash
    git clone https://github.com/lalitm1004/cli-wrapped.git
    cd cli-wrapped
    ```
2. Add the following to the end of your `.bashrc` file:
    ```bash
    function _log_command() {
        local command="$1"
        local cmd_name=$(echo "$command" | awk '{print $1}')

        if command -v "$cmd_name" >/dev/null 2>&1; then
            cli-wrapped log "$command"
        fi
    }

    if [ -z "$PROMPT_COMMAND" ]; then
        PROMPT_COMMAND='_log_command "$(history 1 | sed "s/^[ ]*[0-9]*[ ]*//")"'
    else
        PROMPT_COMMAND="_log_command \"\$(history 1 | sed 's/^[ ]*[0-9]*[ ]*//')\"; $PROMPT_COMMAND"
    fi
    ```
3. Build using `cargo build --release` and place the build `.exe` into your PATH so that bash can access it.

## Usage
- Run cli-wrapped
```bash
$ cli-wrapped
```

- Clear command logs [This will delete all logs across all years]
```bash
$ cli-wrapped clear
```