# TyperTask-Next Macro Script Syntax Reference (`.tt`)

TyperTask-Next uses a clean, line-oriented plain-text syntax for defining macros.

## Command Reference

### `DELAY <milliseconds>`
Pauses execution for the specified duration in milliseconds.
```tt
DELAY 500
```

### `MOVE <x> <y>`
Moves the mouse cursor to absolute screen coordinates `(x, y)`.
```tt
MOVE 1920 1080
```

### `CLICK` or `CLICK [x y]`
Simulates a left mouse click at the current cursor position or at optional coordinates `(x, y)`.
```tt
CLICK
CLICK 100 200
```

### `TYPE "<text>"`
Types the specified literal string using Unicode virtual key scancodes.
```tt
TYPE "Automated workflow complete."
```

### `KEY <VK_CODE>`
Presses and releases a specific Windows Virtual Key code (e.g., `VK_RETURN`, `VK_TAB`, `VK_ESCAPE`).
```tt
KEY VK_RETURN
```

### `REPEAT <count> { ... }`
Repeats a block of commands a given number of times.
```tt
REPEAT 5 {
    TYPE "Iteration"
    KEY VK_RETURN
    DELAY 200
}
```
