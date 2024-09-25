```bash
# Cargo wacth install
function cwi() {
    cargo watch -x "install --path ."
}

# cargo watch example zsh/back function
# usage `cwe xp_file_name`
function cwe() {
    cargo watch -q -c -x "run -q --example '$1'"
}

# - `cwt test_my_fn` for a test function  name match
# - `cwt test_file_name test_my_fn`
function cwt() {
    if [[$# -eq 1]]; then
    cargo watch -q -c -x "test $1 -- --nocapture"
    elif [[$# -eq 2]]; then
    cargo watch -q -c -x "test --test '$1' '$2' -- --nocapture"
    else
    cargo watch -q -c -x "test -- --nocapture"
    fi
}
```
