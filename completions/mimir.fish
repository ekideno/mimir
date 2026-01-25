function __mimir_complete_open
    set -l prefix (commandline -ct)
    mimir __Complete open -- $prefix
end

complete -c mimir -n '__fish_use_subcommand' -a open

complete -c mimir -n '__fish_seen_subcommand_from open' -a '(__mimir_complete_open)'
