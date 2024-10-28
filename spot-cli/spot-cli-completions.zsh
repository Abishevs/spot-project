#compdef spot-cli

autoload -U is-at-least

_spot-cli() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-v[Verbose mode- global]' \
'--verbose[Verbose mode- global]' \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
":: :_spot-cli_commands" \
"*::: :->spot" \
&& ret=0
    case $state in
    (spot)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:spot-cli-command-$line[1]:"
        case $line[1] in
            (session)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
":: :_spot-cli__session_commands" \
"*::: :->session" \
&& ret=0

    case $state in
    (session)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:spot-cli-session-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(end)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
":: :_spot-cli__session__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:spot-cli-session-help-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(end)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(pomodoro)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
":: :_spot-cli__pomodoro_commands" \
"*::: :->pomodoro" \
&& ret=0

    case $state in
    (pomodoro)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:spot-cli-pomodoro-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" \
'-d+[Duration of the Pomodoro in minutes.]:DURATION: ' \
'--duration=[Duration of the Pomodoro in minutes.]:DURATION: ' \
'-b+[Break time in minutes after each Pomodoro.]:BREAK_TIME: ' \
'--break=[Break time in minutes after each Pomodoro.]:BREAK_TIME: ' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(stop)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
":: :_spot-cli__pomodoro__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:spot-cli-pomodoro-help-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(stop)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(project)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
":: :_spot-cli__project_commands" \
"*::: :->project" \
&& ret=0

    case $state in
    (project)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:spot-cli-project-command-$line[1]:"
        case $line[1] in
            (new)
_arguments "${_arguments_options[@]}" \
'-d+[Project description]:DESCRIPTION: ' \
'--description=[Project description]:DESCRIPTION: ' \
'*-t+[Add project tags either with -t tag1 -t tag2 or -t '\''tag1,tag2'\'']:TAGS: ' \
'*--tags=[Add project tags either with -t tag1 -t tag2 or -t '\''tag1,tag2'\'']:TAGS: ' \
'-h[Print help]' \
'--help[Print help]' \
':name -- New Project name:' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
":: :_spot-cli__project__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:spot-cli-project-help-command-$line[1]:"
        case $line[1] in
            (new)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(config)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
'::project_name -- Set the project name.:' \
&& ret=0
;;
(generate-completions)
_arguments "${_arguments_options[@]}" \
'-s+[Specify the shell for the completion script]:SHELL:(bash elvish fish powershell zsh)' \
'--shell=[Specify the shell for the completion script]:SHELL:(bash elvish fish powershell zsh)' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
":: :_spot-cli__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:spot-cli-help-command-$line[1]:"
        case $line[1] in
            (session)
_arguments "${_arguments_options[@]}" \
":: :_spot-cli__help__session_commands" \
"*::: :->session" \
&& ret=0

    case $state in
    (session)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:spot-cli-help-session-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(end)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
(pomodoro)
_arguments "${_arguments_options[@]}" \
":: :_spot-cli__help__pomodoro_commands" \
"*::: :->pomodoro" \
&& ret=0

    case $state in
    (pomodoro)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:spot-cli-help-pomodoro-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(stop)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
(project)
_arguments "${_arguments_options[@]}" \
":: :_spot-cli__help__project_commands" \
"*::: :->project" \
&& ret=0

    case $state in
    (project)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:spot-cli-help-project-command-$line[1]:"
        case $line[1] in
            (new)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
(config)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(generate-completions)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_spot-cli_commands] )) ||
_spot-cli_commands() {
    local commands; commands=(
'session:' \
'pomodoro:' \
'project:' \
'config:' \
'generate-completions:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'spot-cli commands' commands "$@"
}
(( $+functions[_spot-cli__config_commands] )) ||
_spot-cli__config_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli config commands' commands "$@"
}
(( $+functions[_spot-cli__help__config_commands] )) ||
_spot-cli__help__config_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli help config commands' commands "$@"
}
(( $+functions[_spot-cli__help__session__end_commands] )) ||
_spot-cli__help__session__end_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli help session end commands' commands "$@"
}
(( $+functions[_spot-cli__session__end_commands] )) ||
_spot-cli__session__end_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli session end commands' commands "$@"
}
(( $+functions[_spot-cli__session__help__end_commands] )) ||
_spot-cli__session__help__end_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli session help end commands' commands "$@"
}
(( $+functions[_spot-cli__generate-completions_commands] )) ||
_spot-cli__generate-completions_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli generate-completions commands' commands "$@"
}
(( $+functions[_spot-cli__help__generate-completions_commands] )) ||
_spot-cli__help__generate-completions_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli help generate-completions commands' commands "$@"
}
(( $+functions[_spot-cli__help_commands] )) ||
_spot-cli__help_commands() {
    local commands; commands=(
'session:' \
'pomodoro:' \
'project:' \
'config:' \
'generate-completions:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'spot-cli help commands' commands "$@"
}
(( $+functions[_spot-cli__help__help_commands] )) ||
_spot-cli__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli help help commands' commands "$@"
}
(( $+functions[_spot-cli__pomodoro__help_commands] )) ||
_spot-cli__pomodoro__help_commands() {
    local commands; commands=(
'start:' \
'stop:' \
'status:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'spot-cli pomodoro help commands' commands "$@"
}
(( $+functions[_spot-cli__pomodoro__help__help_commands] )) ||
_spot-cli__pomodoro__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli pomodoro help help commands' commands "$@"
}
(( $+functions[_spot-cli__project__help_commands] )) ||
_spot-cli__project__help_commands() {
    local commands; commands=(
'new:' \
'list:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'spot-cli project help commands' commands "$@"
}
(( $+functions[_spot-cli__project__help__help_commands] )) ||
_spot-cli__project__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli project help help commands' commands "$@"
}
(( $+functions[_spot-cli__session__help_commands] )) ||
_spot-cli__session__help_commands() {
    local commands; commands=(
'start:' \
'end:' \
'status:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'spot-cli session help commands' commands "$@"
}
(( $+functions[_spot-cli__session__help__help_commands] )) ||
_spot-cli__session__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli session help help commands' commands "$@"
}
(( $+functions[_spot-cli__help__project__list_commands] )) ||
_spot-cli__help__project__list_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli help project list commands' commands "$@"
}
(( $+functions[_spot-cli__project__help__list_commands] )) ||
_spot-cli__project__help__list_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli project help list commands' commands "$@"
}
(( $+functions[_spot-cli__project__list_commands] )) ||
_spot-cli__project__list_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli project list commands' commands "$@"
}
(( $+functions[_spot-cli__help__project__new_commands] )) ||
_spot-cli__help__project__new_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli help project new commands' commands "$@"
}
(( $+functions[_spot-cli__project__help__new_commands] )) ||
_spot-cli__project__help__new_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli project help new commands' commands "$@"
}
(( $+functions[_spot-cli__project__new_commands] )) ||
_spot-cli__project__new_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli project new commands' commands "$@"
}
(( $+functions[_spot-cli__help__pomodoro_commands] )) ||
_spot-cli__help__pomodoro_commands() {
    local commands; commands=(
'start:' \
'stop:' \
'status:' \
    )
    _describe -t commands 'spot-cli help pomodoro commands' commands "$@"
}
(( $+functions[_spot-cli__pomodoro_commands] )) ||
_spot-cli__pomodoro_commands() {
    local commands; commands=(
'start:' \
'stop:' \
'status:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'spot-cli pomodoro commands' commands "$@"
}
(( $+functions[_spot-cli__help__project_commands] )) ||
_spot-cli__help__project_commands() {
    local commands; commands=(
'new:' \
'list:' \
    )
    _describe -t commands 'spot-cli help project commands' commands "$@"
}
(( $+functions[_spot-cli__project_commands] )) ||
_spot-cli__project_commands() {
    local commands; commands=(
'new:' \
'list:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'spot-cli project commands' commands "$@"
}
(( $+functions[_spot-cli__help__session_commands] )) ||
_spot-cli__help__session_commands() {
    local commands; commands=(
'start:' \
'end:' \
'status:' \
    )
    _describe -t commands 'spot-cli help session commands' commands "$@"
}
(( $+functions[_spot-cli__session_commands] )) ||
_spot-cli__session_commands() {
    local commands; commands=(
'start:' \
'end:' \
'status:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'spot-cli session commands' commands "$@"
}
(( $+functions[_spot-cli__help__pomodoro__start_commands] )) ||
_spot-cli__help__pomodoro__start_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli help pomodoro start commands' commands "$@"
}
(( $+functions[_spot-cli__help__session__start_commands] )) ||
_spot-cli__help__session__start_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli help session start commands' commands "$@"
}
(( $+functions[_spot-cli__pomodoro__help__start_commands] )) ||
_spot-cli__pomodoro__help__start_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli pomodoro help start commands' commands "$@"
}
(( $+functions[_spot-cli__pomodoro__start_commands] )) ||
_spot-cli__pomodoro__start_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli pomodoro start commands' commands "$@"
}
(( $+functions[_spot-cli__session__help__start_commands] )) ||
_spot-cli__session__help__start_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli session help start commands' commands "$@"
}
(( $+functions[_spot-cli__session__start_commands] )) ||
_spot-cli__session__start_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli session start commands' commands "$@"
}
(( $+functions[_spot-cli__help__pomodoro__status_commands] )) ||
_spot-cli__help__pomodoro__status_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli help pomodoro status commands' commands "$@"
}
(( $+functions[_spot-cli__help__session__status_commands] )) ||
_spot-cli__help__session__status_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli help session status commands' commands "$@"
}
(( $+functions[_spot-cli__pomodoro__help__status_commands] )) ||
_spot-cli__pomodoro__help__status_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli pomodoro help status commands' commands "$@"
}
(( $+functions[_spot-cli__pomodoro__status_commands] )) ||
_spot-cli__pomodoro__status_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli pomodoro status commands' commands "$@"
}
(( $+functions[_spot-cli__session__help__status_commands] )) ||
_spot-cli__session__help__status_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli session help status commands' commands "$@"
}
(( $+functions[_spot-cli__session__status_commands] )) ||
_spot-cli__session__status_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli session status commands' commands "$@"
}
(( $+functions[_spot-cli__help__pomodoro__stop_commands] )) ||
_spot-cli__help__pomodoro__stop_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli help pomodoro stop commands' commands "$@"
}
(( $+functions[_spot-cli__pomodoro__help__stop_commands] )) ||
_spot-cli__pomodoro__help__stop_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli pomodoro help stop commands' commands "$@"
}
(( $+functions[_spot-cli__pomodoro__stop_commands] )) ||
_spot-cli__pomodoro__stop_commands() {
    local commands; commands=()
    _describe -t commands 'spot-cli pomodoro stop commands' commands "$@"
}

if [ "$funcstack[1]" = "_spot-cli" ]; then
    _spot-cli "$@"
else
    compdef _spot-cli spot-cli
fi
