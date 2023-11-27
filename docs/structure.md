# Structure

## Files

- probably yaml because it's easiest
- all inside the .tart file (non-global?)

### status.yaml
- current_board: String

### config.yaml
- user
  - name: String
  - email: String

### boards/[id].yaml
- yamls in the boards directory

## Commands

### `init`
- creates `.tart` directory
- initializes the default file stuffs
- checks configs files?

### `board <board_name>`
- switches boards currently focused on
- creates new board
- lists boards if no params

### `task <hash? or index?>`
- `>`: move forward in progress
- `<`: move backward in progress
- `--comment | -c`: add a comment

### `watch`
- updates the table from file changes within tart
- shows the boards and completion, synced with remote??