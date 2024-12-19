function _log_command() {
    local command="$1"
    cli-wrapped log "$command"
}

if [ -z "$PROMPT_COMMAND" ]; then
    PROMPT_COMMAND='_log_command "$(history 1 | sed "s/^[ ]*[0-9]*[ ]*//")"'
else
    PROMPT_COMMAND="_log_command \"\$(history 1 | sed 's/^[ ]*[0-9]*[ ]*//')\"; $PROMPT_COMMAND"
fi