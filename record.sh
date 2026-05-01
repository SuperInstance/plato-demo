#!/usr/bin/env bash
# record.sh — Record a PLATO demo session as a terminal cast
#
# Dependencies (install one):
#   pip install asciinema agg         # recommended
#   pip install termtosvg            # alt
#
# Usage:
#   ./record.sh              # interactive asciinema (default)
#   ./record.sh --script     # non-interactive using `script` command
#   ./record.sh --byexample  # via byexample

set -e

DEMO_DIR="/tmp/plato-demo"
RECORDING="$DEMO_DIR/recording.json"
GIF_OUTPUT="$DEMO_DIR/demo.gif"
DEMO_SCRIPT="$DEMO_DIR/demo.py"

cd "$DEMO_DIR"

if [ "$1" = "--script" ]; then
    echo "Recording with 'script' command (non-interactive)..."
    script -q -c "python3 $DEMO_SCRIPT" "$DEMO_DIR/terminal.out"
    echo "Done. Output: $DEMO_DIR/terminal.out"

elif [ "$1" = "--byexample" ]; then
    echo "Recording with byexample..."
    byexample run -l python3 "$DEMO_SCRIPT" --asciinema "$RECORDING"
    if [ -f "$RECORDING" ]; then
        echo "Converting to GIF with agg..."
        agg "$RECORDING" "$GIF_OUTPUT" 2>/dev/null || echo "agg not installed; recording saved to $RECORDING"
    fi

else
    echo "Recording with asciinema (interactive)..."
    echo "  - Run: python3 $DEMO_SCRIPT"
    echo "  - Exit: Ctrl+D or Ctrl+C"
    echo ""
    asciinema rec "$RECORDING" --command "python3 $DEMO_SCRIPT" --title "PLATO 5-Atom Reasoning Chain"
    echo ""
    echo "Recording saved: $RECORDING"
    echo ""

    if command -v agg &>/dev/null; then
        echo "Converting to GIF..."
        agg "$RECORDING" "$GIF_OUTPUT"
        echo "GIF saved: $GIF_OUTPUT"
    else
        echo "To convert to GIF, install agg: pip install agg"
        echo "Then run: agg $RECORDING $GIF_OUTPUT"
    fi
fi

echo ""
echo "Files in $DEMO_DIR:"
ls -lh "$DEMO_DIR/"