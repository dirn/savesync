# SaveSync

Synchronize game saves.

## Configuration

The following environment variables must be set to run SaveSync.

`RETRO_GAMES`: The location to which to synchronize the saves. It must be an
existing directory.

`RETRO_SAVES`: The location from which to synchronize the saves. It must be an
existing directory.

The following environment variables can optionally be set when running SaveSync.

`RETRO_BOOTSTRAP`: Fully synchronize the source location at startup. Possible values are `1`,
`true`, and `yes`.
