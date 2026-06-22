#!/usr/bin/env sh
# Shared project shell environment for non-login shells.
if [ -n "${HOME:-}" ] && [ -d "$HOME/.cargo/bin" ]; then
  case ":$PATH:" in
    *":$HOME/.cargo/bin:"*) ;;
    *) PATH="$HOME/.cargo/bin:$PATH" ;;
  esac
  export PATH
fi

if [ -n "${HOME:-}" ] && [ -d "$HOME/.local/bin" ]; then
  case ":$PATH:" in
    *":$HOME/.local/bin:"*) ;;
    *) PATH="$HOME/.local/bin:$PATH" ;;
  esac
  export PATH
fi
