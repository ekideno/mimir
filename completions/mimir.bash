_mimir_complete() {
    local cur prev
    COMPREPLY=()

    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    if [[ $COMP_CWORD -eq 1 ]]; then
        COMPREPLY=( $(compgen -W "open" -- "$cur") )
        return
    fi

    if [[ $COMP_CWORD -eq 2 && "$prev" == "open" ]]; then
        mapfile -t COMPREPLY < <(mimir __Complete open -- "$cur")
        return
    fi
}

complete -F _mimir_complete mimir
