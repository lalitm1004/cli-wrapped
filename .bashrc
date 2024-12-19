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