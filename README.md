# Matrix Rain

This is a matrix rain _in rust_.

Because for each column we need only to change three characters at each frame, we don't need to redraw the buffer every time. With this optimization we can reduce render time.
Thanks to that, this implementation is about four times less cpu heavy than _cmatrix_.

## ToDo

- Dynamic resizing
- Different colors
- Different characters
